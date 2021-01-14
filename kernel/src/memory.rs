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
