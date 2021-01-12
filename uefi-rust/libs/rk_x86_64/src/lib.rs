#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(naked_functions)]

pub mod gdt;
pub mod idt;
pub mod register;

/// Halts the CPU forever.
#[inline]
pub fn hang() -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}

/// An entry in a page table.
pub struct PageTableEntry(u64);

impl PageTableEntry {
    /// Reads a page table entry from the specified address.
    ///
    /// # Safety
    /// The address must point to a page table entry.
    pub unsafe fn read(addr: *const u64) -> Self {
        Self {
            0: core::ptr::read(addr),
        }
    }

    #[inline]
    pub fn is_present(&self) -> bool {
        self.0 & 1 == 1
    }

    #[inline]
    pub fn addr(&self) -> u64 {
        // Remove the 12 least significant bits (the options)
        self.0 >> 12 << 12
    }
}

// Use packed representation to stop Rust from adding padding and thus breaking
// the representation.
#[repr(C, packed)]
struct DescriptorTablePointer {
    /// The size of the table - 1.
    size: u16,
    /// Pointer to the table.
    offset: u64,
}
