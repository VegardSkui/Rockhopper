#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(naked_functions)]

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

/// Loads a segment selector into the code segment register.
///
/// # Safety
/// The `segment_selector` must reference a valid code segment descriptor.
#[naked]
pub unsafe extern "sysv64" fn load_cs(_segment_selector: u16) {
    asm!(
        // Pop the return address off the stack, rax is a scratch register in the System V ABI.
        "pop rax",
        // Push the selector onto the stack, the System V ABI specifies that the first argument
        // is passed through rdi.
        "push rdi",
        // Push the return address back onto the stack.
        "push rax",
        // Do a far return, which pops an extra element off the stack and loads it into the
        // code segment register.
        "retfq",
        options(noreturn)
    );
}
