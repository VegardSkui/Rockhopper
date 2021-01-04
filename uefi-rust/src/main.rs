#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(asm)]

extern crate rlibc;

#[macro_use]
mod macros;

pub mod data_types;
pub mod guid;
pub mod protocol;
pub mod table;

use crate::data_types::{EfiHandle, EfiStatus};
use crate::table::EfiSystemTable;
use core::panic::PanicInfo;

static mut IMAGE_HANDLE: EfiHandle = EfiHandle(core::ptr::null());
static mut SYSTEM_TABLE: *mut EfiSystemTable = 0 as *mut EfiSystemTable;

pub fn system_table() -> &'static EfiSystemTable {
    // Safety: We assume the system table has been set
    unsafe { &*SYSTEM_TABLE }
}

/// The main entry point for the UEFI application.
#[no_mangle]
fn efi_main(image_handle: EfiHandle, system_table: &'static mut EfiSystemTable) -> EfiStatus {
    unsafe {
        IMAGE_HANDLE = image_handle;
        SYSTEM_TABLE = system_table;
    }

    system_table.con_out().reset(false);

    println!("Hello World!");

    // Print the firmware vendor and revision
    print!("Firmware: ");
    system_table
        .con_out()
        .output_string(system_table.firmware_vendor());
    println!(", rev. {:#010x}", system_table.firmware_revision());

    // Print the UEFI revision
    let revision = system_table.revision();
    println!("UEFI v{}.{}", (revision >> 16) as u16, revision as u16);

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
