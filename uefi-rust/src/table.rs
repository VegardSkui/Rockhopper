use crate::data_types::{
    Char16, EfiAllocateType, EfiGuid, EfiHandle, EfiMemoryType, EfiPhysicalAddress, EfiStatus,
    EfiTpl,
};
use crate::protocol::{EfiSimpleTextInputProtocol, EfiSimpleTextOutputProtocol};
use core::ffi::c_void;

/// Data structure that precedes all of the standard EFI table types.
#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    _reserved: u32,
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

/// Contains a table header and pointers to all of the boot services.
#[repr(C)]
pub struct EfiBootServices {
    hdr: EfiTableHeader,

    // Task Priority Services
    raise_tpl: extern "efiapi" fn(new_tpl: EfiTpl) -> EfiTpl,
    restore_tpl: extern "efiapi" fn(old_tpl: EfiTpl),

    // Memory Services
    allocate_pages: extern "efiapi" fn(
        type1: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: usize,
        memory: &mut EfiPhysicalAddress,
    ) -> EfiStatus,
    free_pages: extern "efiapi" fn(),     // TODO
    get_memory_map: extern "efiapi" fn(), // TODO
    allocate_pool: extern "efiapi" fn(),  // TODO
    free_pool: extern "efiapi" fn(),      // TODO

    // Event & Timer Services
    create_event: extern "efiapi" fn(),   // TODO
    set_timer: extern "efiapi" fn(),      // TODO
    wait_for_event: extern "efiapi" fn(), // TODO
    signal_event: extern "efiapi" fn(),   // TODO
    close_event: extern "efiapi" fn(),    // TODO
    check_event: extern "efiapi" fn(),    // TODO

    // Protocol Handler Services
    install_protocol_interface: extern "efiapi" fn(), // TODO
    reinstall_protocol_interface: extern "efiapi" fn(), // TODO
    uninstall_protocol_interface: extern "efiapi" fn(), // TODO
    handle_protocol: extern "efiapi" fn(),            // TODO
    _reserved: *const c_void,
    register_protocol_notify: extern "efiapi" fn(), // TODO
    locate_handle: extern "efiapi" fn(),            // TODO
    locate_device_path: extern "efiapi" fn(),       // TODO
    install_configuration_table: extern "efiapi" fn(), // TODO

    // Image Services
    load_image: extern "efiapi" fn(),         // TODO
    start_image: extern "efiapi" fn(),        // TODO
    exit: extern "efiapi" fn(),               // TODO
    unload_image: extern "efiapi" fn(),       // TODO
    exit_boot_services: extern "efiapi" fn(), // TODO

    // Miscellaneous Services
    get_next_monotonix_count: extern "efiapi" fn(), // TODO
    stall: extern "efiapi" fn(microseconds: usize) -> EfiStatus, // TODO
    set_watchdog_timer: extern "efiapi" fn(),       // TODO
}

impl EfiBootServices {
    pub fn stall(&self, microseconds: usize) -> EfiStatus {
        (self.stall)(microseconds)
    }
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
