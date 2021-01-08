#![no_std]
#![no_main]

use core::panic::PanicInfo;

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

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
