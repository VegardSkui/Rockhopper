#![no_std]
#![no_main]
#![feature(asm)]

extern crate rlibc;

use rk_uefi::{print, println};
use rk_uefi::data_types::{EfiHandle, EfiStatus};
use rk_uefi::table::EfiSystemTable;
use core::panic::PanicInfo;

/// The main entry point for the UEFI application.
#[no_mangle]
fn efi_main(image_handle: EfiHandle, system_table: &'static mut EfiSystemTable) -> EfiStatus {
    rk_uefi::init(image_handle, &mut *system_table);

    rk_uefi::system_table().con_out().reset(false);

    println!("Hello World!");

    // Print the firmware vendor and revision
    print!("Firmware: ");
    rk_uefi::system_table()
        .con_out()
        .output_string(rk_uefi::system_table().firmware_vendor());
    println!(", rev. {:#010x}", rk_uefi::system_table().firmware_revision());

    // Print the UEFI revision
    let revision = rk_uefi::system_table().revision();
    println!("UEFI v{}.{}", (revision >> 16) as u16, revision as u16);

    println!("\nStalling for 1 second");
    rk_uefi::system_table().boot_services().stall(1_000_000);
    println!("Done!");

    hang()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hang()
}

/// Halt the CPU forever.
#[inline]
pub fn hang() -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}
