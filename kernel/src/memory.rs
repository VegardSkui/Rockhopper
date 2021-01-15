use core::alloc::{GlobalAlloc, Layout};
use rk_x86_64::paging::{PageTable, PageTableEntry};
use rk_x86_64::register::cr3;
use spin::Mutex;

//
//                            x86-64 Memory Map
//
//        Lower Half
// (0)    0x0000_0000_0000_0000 - 0x0000_7FFF_FFFF_FFFF    (128 TiB)
//
//        Higher Half
// (1)    0xFFFF_8000_0000_0000 - 0xFFFF_807F_FFFF_FFFF    (512 GiB)
// (2)    0xFFFF_8080_0000_0000 - 0xFFFF_FF7F_FFFF_FFFF    (127 TiB)
// (3)    0xFFFF_FF80_0000_0000 - 0xFFFF_FFFF_FFFF_FFFF    (512 GiB)
//
// (0)    Free
// (1)    Physical Memory Mapping
// (2)    Free
// (3)    Kernel                       (Last entry of PML4)
//

pub const PHYS_MEM_OFFSET: u64 = 0xffff_8000_0000_0000;

// Creates a new bump allocator starting at 32 MiB.
static FRAME_ALLOCATOR: LockedBumpAllocator = unsafe { LockedBumpAllocator::new(0x2000000) };

/// The virtual starting address of the kernel heap.
const HEAP_BASE: u64 = 0xffff_ff80_0000_0000;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeapAllocator = LockedHeapAllocator::new(HEAP_BASE);

pub fn init() {
    let pml4_phys_addr = FRAME_ALLOCATOR
        .allocate(1)
        .expect("Could not allocate memory for PML4");
    let mapper =
        Mutex::new(unsafe { Mapper::new(PHYS_MEM_OFFSET | pml4_phys_addr, &FRAME_ALLOCATOR) });

    // Zero out the PML4
    unsafe {
        core::ptr::write_bytes(pml4_phys_addr as *mut u64, 0, 512);
    }

    // Map the whole of physical memory
    map_physical_memory(pml4_phys_addr);

    // Copy the last entry of the existing PML4, this includes the existing kernel
    // mapping and stack
    unsafe {
        // Copy the last entry of the existing PML4 to the new PML4
        let existing_pml4_addr = cr3::read();
        core::ptr::copy(
            (existing_pml4_addr + 0x8 * 511) as *const u64,
            (pml4_phys_addr + 0x8 * 511) as *mut u64,
            1,
        );
    }

    // Activate the new memory mapping
    unsafe {
        cr3::write(pml4_phys_addr);
    }

    // Allocate and map 80 KiB of memory for the kernel heap
    let heap_phys_addr = FRAME_ALLOCATOR
        .allocate(20)
        .expect("Could not allocate memory for the heap");
    for i in 0..20 {
        unsafe {
            mapper
                .lock()
                .map(HEAP_BASE + 0x1000 * i, heap_phys_addr + 0x1000 * i, 0b11)
        }
    }
}

/// Maps 512 GiBs of physical memory.
///
/// Assumes the memory is still identity mapped.
fn map_physical_memory(pml4_addr: u64) {
    // Allocate enough frames for 1 PDP and 512 PDs, this is enough to map the first
    // 512 GiB of physical memory. We don't need to zero them out since we're going
    // to write to every byte soon.
    let pdp_phys_addr = FRAME_ALLOCATOR
        .allocate(1 + 512)
        .expect("Could not allocate page frames for physical memory mapping");
    let pds_phys_addr = pdp_phys_addr + 0x1000;

    // Add the PDs as entries in the PDP
    for i in 0..512 {
        unsafe {
            core::ptr::write(
                (pdp_phys_addr + 0x8 * i) as *mut u64,
                (pds_phys_addr + 0x1000 * i) | 0b11,
            );
        }
    }

    // Map the first 512 GiB of physical memory with huge pages in the PDs
    for i in 0..512 * 512 {
        unsafe {
            core::ptr::write(
                (pds_phys_addr + 0x8 * i) as *mut u64,
                (0x20_0000 * i) | 0b1000_0011,
            );
        }
    }

    // Reference the PDP in the PML4
    unsafe {
        core::ptr::write((pml4_addr + 0x8 * 256) as *mut u64, pdp_phys_addr | 0b11);
    }
}

struct HeapAllocator {
    start: u64,
    // TODO: Limit size
}

impl HeapAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        if self.start % layout.align() as u64 != 0 {
            self.start += self.start % layout.align() as u64;
        }
        let addr = self.start;
        self.start += layout.size() as u64;
        addr as *mut u8
    }

    unsafe fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        // TODO
    }
}

struct LockedHeapAllocator(Mutex<HeapAllocator>);

impl LockedHeapAllocator {
    pub const fn new(start: u64) -> Self {
        Self(Mutex::new(HeapAllocator { start }))
    }
}

unsafe impl GlobalAlloc for LockedHeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(ptr, layout)
    }
}

/// Represents a virtual address (x86-64).
struct VirtAddr(u64);

impl VirtAddr {
    pub const fn new(addr: u64) -> Self {
        Self(addr)
    }

    /// Returns the raw address represented.
    pub const fn addr(&self) -> u64 {
        self.0
    }

    /// Returns the index into the PML4 for the address.
    pub const fn pml4_index(&self) -> u16 {
        ((self.0 >> 39) % 512) as u16
    }

    /// Returns the index into the PDP for the address.
    pub const fn pdp_index(&self) -> u16 {
        ((self.0 >> 30) % 512) as u16
    }

    /// Returns the index into the PD for the address.
    pub const fn pd_index(&self) -> u16 {
        ((self.0 >> 21) % 512) as u16
    }

    /// Returns the index into the PT for the address.
    pub const fn pt_index(&self) -> u16 {
        ((self.0 >> 12) % 512) as u16
    }
}

struct Mapper<'a, A: FrameAllocator> {
    pml4: PageTable,
    frame_allocator: &'a A,
    // TODO: Make Mapper independent of the specific offset of the physical mapping.
}

impl<'a, A: FrameAllocator> Mapper<'a, A> {
    /// Initializes a new mapper with the PML4 at the given address.
    ///
    /// # Safety
    /// `pml4_addr` must point to a safe location for a valid PML4 (possibly all
    /// zeroes).
    unsafe fn new(pml4_addr: u64, frame_allocator: &'a A) -> Self {
        Self {
            pml4: PageTable::new(pml4_addr),
            frame_allocator,
        }
    }

    /// Maps a virtual address to a physical address, allocating space for new
    /// page tables as necessary.
    ///
    /// # Safety
    /// Overwrites any existing mappings at the same address, which can break
    /// memory safety.
    unsafe fn map(&mut self, virt: u64, phys: u64, flags: u64) {
        assert!(virt % 4096 == 0, "virt is not page aligned");
        assert!(phys % 4096 == 0, "phys is not page aligned");

        // TODO: Support for different page sizes.

        // TODO: This function (like its bootloader-counterpart) could do with some
        // cleaning up.

        let virt = VirtAddr::new(virt);

        let pdp_entry = self.pml4.entry(virt.pml4_index());
        let pdp_phys_addr: u64;
        let pdp_virt_addr: VirtAddr;
        if pdp_entry.is_present() {
            pdp_phys_addr = pdp_entry.addr();
            pdp_virt_addr = VirtAddr::new(PHYS_MEM_OFFSET | pdp_phys_addr);
        } else {
            pdp_phys_addr = self
                .frame_allocator
                .allocate(1)
                .expect("Could not allocate frame for PDP");
            pdp_virt_addr = VirtAddr::new(PHYS_MEM_OFFSET | pdp_phys_addr);
            // Zero out the PDP
            core::ptr::write_bytes(pdp_virt_addr.addr() as *mut u64, 0, 512);
            // Reference the PDP in the PML4
            self.pml4
                .set_entry(virt.pml4_index(), PageTableEntry::new(pdp_phys_addr, 0b11));
        }
        let pdp = PageTable::new(pdp_virt_addr.addr());

        let pd_entry = pdp.entry(virt.pdp_index());
        let pd_phys_addr: u64;
        let pd_virt_addr: VirtAddr;
        if pd_entry.is_present() {
            pd_phys_addr = pd_entry.addr();
            pd_virt_addr = VirtAddr::new(PHYS_MEM_OFFSET | pd_phys_addr);
        } else {
            pd_phys_addr = self
                .frame_allocator
                .allocate(1)
                .expect("Could not allocate frame for PD");
            pd_virt_addr = VirtAddr::new(PHYS_MEM_OFFSET | pd_phys_addr);
            // Zero out the PD
            core::ptr::write_bytes(pd_virt_addr.addr() as *mut u64, 0, 512);
            // Reference the PD in the PDP
            pdp.set_entry(virt.pdp_index(), PageTableEntry::new(pd_phys_addr, 0b11));
        }
        let pd = PageTable::new(pd_virt_addr.addr());

        let pt_entry = pd.entry(virt.pd_index());
        let pt_phys_addr: u64;
        let pt_virt_addr: VirtAddr;
        if pt_entry.is_present() {
            pt_phys_addr = pt_entry.addr();
            pt_virt_addr = VirtAddr::new(PHYS_MEM_OFFSET | pt_phys_addr);
        } else {
            pt_phys_addr = self
                .frame_allocator
                .allocate(1)
                .expect("Could not allocate frame for PT");
            pt_virt_addr = VirtAddr::new(PHYS_MEM_OFFSET | pt_phys_addr);
            // Zero out the PT
            core::ptr::write_bytes(pt_virt_addr.addr() as *mut u64, 0, 512);
            // Reference the PT in the PD
            pd.set_entry(virt.pd_index(), PageTableEntry::new(pt_phys_addr, 0b11));
        }
        let pt = PageTable::new(pt_virt_addr.addr());

        // Set the entry in the PT
        pt.set_entry(virt.pt_index(), PageTableEntry::new(phys, flags));

        // TODO: Flush the TLB to make sure the new mapping is used
    }

    /// Translates a virtual address into it's corresponding physical address.
    ///
    /// Returns an empty error if the address cannot be found.
    fn translate(&self, _virt: u64) -> Result<u64, ()> {
        todo!()
    }
}

trait FrameAllocator {
    /// Allocates the given number of frames and returns the physical address on
    /// success. If the frames could not be allocated, an empty error is
    /// returned.
    fn allocate(&self, count: usize) -> Result<u64, ()>;

    /// Tries to free a number of allocated frames, starting at the given
    /// address.
    fn free(&self, addr: u64, count: usize);
}

struct BumpAllocator {
    // TODO: Use free memory areas, such that we can avoid reserved memory and still allocate free
    // space around it.
    // TODO: Implement some kind of end check (this would be included in the above todo).
    /// The address of the next free frame to be allocated.
    next: u64,
}

impl BumpAllocator {
    /// Creates a new bump allocator with the given physical start address.
    pub const unsafe fn new(start: u64) -> Self {
        Self { next: start }
    }

    fn allocate(&mut self, count: usize) -> Result<u64, ()> {
        let addr = self.next;
        self.next += (count * 4096) as u64;
        Ok(addr)
    }

    fn free(&mut self, _addr: u64, _count: usize) {
        unimplemented!("The bump allocator cannot free frames");
        // Technically we could free all the frames at once, but that situation
        // arising is highly unlikely, so it's not implemented.
    }
}

struct LockedBumpAllocator(Mutex<BumpAllocator>);

impl LockedBumpAllocator {
    pub const unsafe fn new(addr: u64) -> Self {
        Self(Mutex::new(BumpAllocator::new(addr)))
    }
}

impl FrameAllocator for LockedBumpAllocator {
    fn allocate(&self, count: usize) -> Result<u64, ()> {
        self.0.lock().allocate(count)
    }

    fn free(&self, addr: u64, count: usize) {
        self.0.lock().free(addr, count)
    }
}
