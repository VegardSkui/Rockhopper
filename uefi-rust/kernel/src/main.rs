#![no_std]
#![no_main]
#![feature(asm)]

mod graphics;

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

#[no_mangle]
fn _start() -> ! {
    // Initialize a screen from the frame buffer provided by the bootloader
    let screen: Screen;
    unsafe {
        screen = Screen::new(
            entry_data.fb_base,
            entry_data.fb_horizontal_resolution,
            entry_data.fb_vertical_resolution,
            entry_data.fb_pixels_per_scan_line,
        );
    }

    // Clear the screen
    screen.clear();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
