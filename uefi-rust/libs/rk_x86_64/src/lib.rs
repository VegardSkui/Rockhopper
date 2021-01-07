#![no_std]
#![feature(asm)]

pub mod register;

/// Halts the CPU forever.
#[inline]
pub fn hang() -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}
