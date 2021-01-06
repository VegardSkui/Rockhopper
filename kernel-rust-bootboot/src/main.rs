#![no_std]
#![no_main]

use core::convert::TryInto;
use core::panic::PanicInfo;

#[no_mangle]
fn _start() -> ! {
    // Draw a white box in the top left corner, 100x100
    let ptr = 0xfffffffffc000000 as *mut u32;
    let val: u32 = 0x00ffffff;
    for y in 0..100 {
        for x in 0..100 {
            unsafe { ptr.offset((y * 1024 + x).try_into().unwrap()).write(val) }
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
