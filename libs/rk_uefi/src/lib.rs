#![no_std]
#![feature(abi_efiapi)]

#[macro_use]
pub mod macros;
pub mod data_types;
pub mod guid;
pub mod protocol;
pub mod table;

use crate::data_types::EfiHandle;
use crate::table::EfiSystemTable;

static mut IMAGE_HANDLE: EfiHandle = EfiHandle(core::ptr::null());
static mut SYSTEM_TABLE: *mut EfiSystemTable = 0 as *mut EfiSystemTable;

pub fn init(image_handle: EfiHandle, system_table: &'static mut EfiSystemTable) {
    unsafe {
        IMAGE_HANDLE = image_handle;
        SYSTEM_TABLE = system_table;
    }
}

pub fn system_table() -> &'static EfiSystemTable {
    // Safety: We assume the system table has been set
    unsafe { &*SYSTEM_TABLE }
}
