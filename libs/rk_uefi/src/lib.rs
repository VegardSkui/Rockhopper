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
static mut SYSTEM_TABLE: *const EfiSystemTable = 0 as *const EfiSystemTable;

/// Initializes the UEFI library with the given image handle and system table.
///
/// Should be called before any other function in the library.
pub fn init(image_handle: EfiHandle, system_table: &'static EfiSystemTable) {
    unsafe {
        IMAGE_HANDLE = image_handle;
        SYSTEM_TABLE = system_table;
    }
}

/// Returns a reference to the current system table.
pub fn system_table() -> &'static EfiSystemTable {
    // Safety: We assume the system table has been set
    unsafe { &*SYSTEM_TABLE }
}
