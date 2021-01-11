use crate::println;
use rk_x86_64::idt::{InterruptDescriptorTable, InterruptFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[0].set_handler(divide_by_zero_handler);
        idt[3].set_handler(breakpoint_handler);
        idt
    };
}

pub fn init() {
    // Load the IDT, this is safe because the IDT is static and will exists for as
    // long as the kernel is running.
    IDT.load();
}

extern "x86-interrupt" fn divide_by_zero_handler(_interrupt_frame: &InterruptFrame) {
    loop {}
}

extern "x86-interrupt" fn breakpoint_handler(interrupt_frame: &InterruptFrame) {
    println!("BREAKPOINT: {:#x?}", interrupt_frame);
    loop {}
}
