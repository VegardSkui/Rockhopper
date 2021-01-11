use crate::println;
use rk_x86_64::idt::{InterruptDescriptorTable, InterruptFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler(breakpoint_handler);
        idt.double_fault.set_handler(double_fault_handler);
        idt
    };
}

pub fn init() {
    // Load the IDT, this is safe because the IDT is static and will exists for as
    // long as the kernel is running.
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(interrupt_frame: &InterruptFrame) {
    println!("BREAKPOINT: {:#x?}", interrupt_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    interrupt_frame: &InterruptFrame,
    _error_code: u64,
) -> ! {
    panic!("DOUBLE FAULT: {:#x?}", interrupt_frame);
}
