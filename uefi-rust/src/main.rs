#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(asm)]

extern crate rlibc;

pub mod data_types;
pub mod guid;
pub mod protocol;
pub mod table;

use crate::data_types::{EfiHandle, EfiStatus};
use crate::table::EfiSystemTable;
use core::panic::PanicInfo;

/// The main entry point for the UEFI application.
#[no_mangle]
fn efi_main(_image_handle: EfiHandle, system_table: EfiSystemTable) -> EfiStatus {
    system_table.con_out().reset(false);
    system_table.con_out().output_rust_string("Hello World!");

    // Loop forever
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO: Print info
    loop {}
}
