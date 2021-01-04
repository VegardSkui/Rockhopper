use crate::data_types::{Char16, EfiGuid, EfiHandle};
use crate::protocol::{EfiSimpleTextInputProtocol, EfiSimpleTextOutputProtocol};
use core::ffi::c_void;

/// Data structure that precedes all of the standard EFI table types.
#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

/// Contains pointers to the runtime and boot services tables.
#[repr(C)]
pub struct EfiSystemTable {
    hdr: EfiTableHeader,
    firmware_vendor: *const Char16,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    con_in: *const EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    con_out: *const EfiSimpleTextOutputProtocol,
    standard_error_handle: EfiHandle,
    std_err: *const EfiSimpleTextOutputProtocol,
    runtime_services: *const EfiRuntimeServices,
    boot_services: *const EfiBootServices,
    number_of_table_entries: usize,
    configuration_table: *const EfiConfigurationTable,
}

impl EfiSystemTable {
    /// Returns the simple text output protocol.
    pub fn con_out(&self) -> &mut EfiSimpleTextOutputProtocol {
        let con_out_ptr = self.con_out as *mut EfiSimpleTextOutputProtocol;
        unsafe { &mut *con_out_ptr }
    }
}

/// Contains a table header and pointers to all of the boot services.
#[repr(C)]
pub struct EfiBootServices {
    hdr: EfiTableHeader,
    // TODO
}

/// Contains a table header and pointers to all of the runtime services.
#[repr(C)]
pub struct EfiRuntimeServices {
    hdr: EfiTableHeader,
    // TODO
}

#[repr(C)]
pub struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: *const c_void,
}
