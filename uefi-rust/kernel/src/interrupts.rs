use rk_x86_64::idt::InterruptDescriptorTable;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[0].set_handler(divide_by_zero_handler);
        idt
    };
}

pub fn init() {
    // Load the IDT, this is safe because the IDT is static and will exists for as
    // long as the kernel is running.
    IDT.load();
}

extern "x86-interrupt" fn divide_by_zero_handler() {
    loop {}
}
