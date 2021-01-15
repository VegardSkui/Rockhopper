#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(asm)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate lazy_static;

mod gdt;
mod graphics;
mod interrupts;
mod memory;
mod psf2;
mod terminal;

use crate::graphics::Screen;
use crate::terminal::Terminal;
use core::panic::PanicInfo;
use spin::Mutex;

/// The data structure passed to the kernel on entry.
#[repr(C)]
pub struct EntryData {
    greeting: u32,
    fb_base: u64,
    fb_horizontal_resolution: u32,
    fb_vertical_resolution: u32,
    fb_pixels_per_scan_line: u32,
}

extern "C" {
    pub static entry_data: EntryData;
}

lazy_static! {
    pub static ref SCREEN: Screen = unsafe {
        // Initialize a screen from the frame buffer provided by the bootloader, which
        // should satisfy the safety requirements.
        Screen::new(
            memory::PHYS_MEM_OFFSET | entry_data.fb_base,
            entry_data.fb_horizontal_resolution,
            entry_data.fb_vertical_resolution,
            entry_data.fb_pixels_per_scan_line,
        )
    };

    // Initialize a text terminal on the screen provided by the bootloader.
    pub static ref TERMINAL: Mutex<Terminal<'static>> = Mutex::new(Terminal::new(&SCREEN));
}

#[no_mangle]
fn _start() -> ! {
    memory::init();
    gdt::init();
    interrupts::init();

    // Clear the screen
    SCREEN.clear();

    // Print the digits
    for c in 0..10 {
        println!("{}", c);
    }

    // Test the kernel heap by using vectors
    let mut vector1 = vec![10, 20, 30];
    println!("Vector1 = {:?}", vector1);
    let mut vector2 = vec![70, 80, 90];
    vector1.append(&mut vector2);
    println!("Vector1+2 = {:?}", vector1);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("ALLOCATION ERROR: {:?}", layout);
}
