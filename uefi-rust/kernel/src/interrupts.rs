use rk_x86_64::idt::InterruptDescriptorTable;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
    unsafe {
        // Set handlers
        IDT[0].set_handler(divide_by_zero_handler);

        // Load the IDT, this is safe because the IDT is static and will exists for as
        // long as the kernel is running.
        IDT.load();
    }
}

extern "x86-interrupt" fn divide_by_zero_handler() {
    loop {}
}
