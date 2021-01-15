#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(naked_functions)]

pub mod gdt;
pub mod idt;
pub mod paging;
pub mod register;

/// Halts the CPU forever.
#[inline]
pub fn hang() -> ! {
    loop {
        unsafe { asm!("hlt") }
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
