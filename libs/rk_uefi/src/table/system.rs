use crate::data_types::{Char16, EfiGuid, EfiHandle};
use crate::protocol::{EfiSimpleTextInputProtocol, EfiSimpleTextOutputProtocol};
use crate::table::{EfiBootServices, EfiRuntimeServices, EfiTableHeader};
use core::ffi::c_void;

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
    /// Returns the revision of the EFI Specification this table conforms to.
    pub fn revision(&self) -> u32 {
        self.hdr.revision
    }

    pub fn firmware_vendor(&self) -> &Char16 {
        unsafe { &*self.firmware_vendor }
    }

    pub fn firmware_revision(&self) -> u32 {
        self.firmware_revision
    }

    /// Returns the simple text output protocol.
    pub fn con_out(&self) -> &mut EfiSimpleTextOutputProtocol {
        let con_out_ptr = self.con_out as *mut EfiSimpleTextOutputProtocol;
        unsafe { &mut *con_out_ptr }
    }

    pub fn boot_services(&self) -> &EfiBootServices {
        unsafe { &*self.boot_services }
    }
}

#[repr(C)]
pub struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: *const c_void,
}
