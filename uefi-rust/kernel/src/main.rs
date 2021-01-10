#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

#[macro_use]
extern crate lazy_static;

mod graphics;
mod interrupts;
mod psf2;
mod terminal;

use crate::graphics::Screen;
use core::panic::PanicInfo;

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
            entry_data.fb_base,
            entry_data.fb_horizontal_resolution,
            entry_data.fb_vertical_resolution,
            entry_data.fb_pixels_per_scan_line,
        )
    };
}

#[no_mangle]
fn _start() -> ! {
    // Set up interrupts
    interrupts::init();

    // Clear the screen
    SCREEN.clear();

    // Initialize a new text terminal
    let mut terminal = terminal::Terminal::new(*SCREEN);

    // Print the digits
    for c in 0..10 {
        terminal.put_char(0x30 + c);
    }
    terminal.new_line();

    // Print the letters of the alphabet in both upper and lower case
    for c in 0..26 {
        terminal.put_char(0x41 + c);
        terminal.put_char(0x61 + c);
        terminal.new_line();
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
