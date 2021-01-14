use rk_x86_64::register::cr3;

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
static mut FRAME_ALLOCATOR: BumpAllocator = unsafe { BumpAllocator::new(0x2000000) };

lazy_static! {
    static ref PML4_PHYS_ADDR: u64 = unsafe { FRAME_ALLOCATOR.allocate(1).unwrap() };
}

pub fn init() {
    // Zero out the PML4
    unsafe {
        core::ptr::write_bytes(*PML4_PHYS_ADDR as *mut u64, 0, 512);
    }

    // Map the whole of physical memory
    map_physical_memory(*PML4_PHYS_ADDR);

    // Copy the last entry of the existing PML4, this includes the existing kernel
    // mapping and stack
    unsafe {
        // Copy the last entry of the existing PML4 to the new PML4
        let existing_pml4_addr = cr3::read();
        //let last_entry = core::ptr::read((existing_pml4_addr + 0x8 * 511) as *const
        // u64);
        core::ptr::copy(
            (existing_pml4_addr + 0x8 * 511) as *const u64,
            (*PML4_PHYS_ADDR + 0x8 * 511) as *mut u64,
            1,
        );
    }

    // Activate the new memory mapping
    unsafe {
        cr3::write(*PML4_PHYS_ADDR);
    }
}

/// Maps 512 GiBs of physical memory.
///
/// Assumes the memory is still identity mapped.
fn map_physical_memory(pml4_addr: u64) {
    // Allocate enough frames for 1 PDP and 512 PDs, this is enough to map the first
    // 512 GiB of physical memory. We don't need to zero them out since we're going
    // to write to every byte soon.
    let pdp_phys_addr = unsafe { FRAME_ALLOCATOR }
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

trait FrameAllocator {
    /// Allocates the given number of frames and returns the physical address on
    /// success. If the frames could not be allocated, an empty error is
    /// returned.
    fn allocate(&mut self, count: usize) -> Result<u64, ()>;

    /// Tries to free a number of allocated frames, starting at the given
    /// address.
    fn free(&mut self, addr: u64, count: usize);
}

#[derive(Copy, Clone)]
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
}

impl FrameAllocator for BumpAllocator {
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
