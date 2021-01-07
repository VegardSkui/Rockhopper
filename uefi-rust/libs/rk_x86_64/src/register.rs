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
