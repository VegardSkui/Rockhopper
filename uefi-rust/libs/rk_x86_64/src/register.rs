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
    pub fn write(value: u64) {
        unsafe {
            asm!("mov cr0, {}", in(reg) value);
        }
    }
}

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
    pub fn write(value: u64) {
        unsafe {
            asm!("mov cr3, {}", in(reg) value);
        }
    }
}

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
    pub fn write(value: u64) {
        unsafe {
            asm!("mov cr4, {}", in(reg) value);
        }
    }
}
