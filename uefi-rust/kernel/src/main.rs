#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

/// The data structure passed to the kernel on entry.
#[repr(C)]
pub struct EntryData {
    greeting: u32,
}

extern "C" {
    pub static entry_data: EntryData;
}

#[no_mangle]
fn _start() -> ! {
    // Assuming we have a frame buffer at 0x8000_0000 with a ??G? pixel format this
    // should draw some green pixels on the screen.
    let fb = 0x8000_0000 as *mut u32;
    for i in 40_000..80_000 {
        unsafe {
            fb.offset(i).write_volatile(0x0000ff00);
        }
    }

    unsafe {
        asm!("", in("r12") entry_data.greeting);
        asm!("int3");
        // Looking in the R12 register at the time of the INT3 exception we
        // should see the value transferred from the bootloader. We'll also have
        // a page fault and system crash since we haven't set up our interrupt
        // table yet.
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
