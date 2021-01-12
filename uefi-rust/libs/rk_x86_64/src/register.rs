/// Control Register 0 (CR0).
pub mod cr0 {
    /// Returns the current value of the CR0 register.
    pub fn read() -> u64 {
        let value: u64;
        unsafe {
            asm!("mov {}, cr0", out(reg) value);
        }
        value
    }

    /// Loads a new value into the CR0 register.
    ///
    /// # Safety
    /// An incorrect or unexpected value can break safety guarantees.
    pub unsafe fn write(value: u64) {
        asm!("mov cr0, {}", in(reg) value);
    }
}

/// Control Register 3 (CR3) contains the current physical address of the PML4
/// table.
pub mod cr3 {
    /// Returns the current value of the CR3 register.
    pub fn read() -> u64 {
        let value: u64;
        unsafe {
            asm!("mov {}, cr3", out(reg) value);
        }
        value
    }

    /// Loads a new value into the CR3 register.
    ///
    /// # Safety
    /// Changing the PML4 can break most memory safety guarantees.
    pub unsafe fn write(value: u64) {
        asm!("mov cr3, {}", in(reg) value);
    }
}

/// Control Register 4 (CR4)
pub mod cr4 {
    /// Returns the current value of the CR4 register.
    pub fn read() -> u64 {
        let value: u64;
        unsafe {
            asm!("mov {}, cr4", out(reg) value);
        }
        value
    }

    /// Loads a new value into the CR4 register.
    ///
    /// # Safety
    /// Changing the wrong flags can violate memory safety guarantees.
    pub unsafe fn write(value: u64) {
        asm!("mov cr4, {}", in(reg) value);
    }
}

pub mod cs {
    /// Reads the value in the code segment register.
    pub fn read() -> u16 {
        let value: u16;
        unsafe {
            asm!("mov {0:x}, cs", out(reg) value);
        }
        value
    }

    /// Loads a segment selector into the code segment register.
    ///
    /// # Safety
    /// The `segment_selector` must reference a valid code segment descriptor.
    #[naked]
    pub unsafe extern "sysv64" fn write(_segment_selector: u16) {
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
}
