#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

pub mod idt;
pub mod register;

/// Halts the CPU forever.
#[inline]
pub fn hang() -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}

pub struct PageTableEntry(u64);

impl PageTableEntry {
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
