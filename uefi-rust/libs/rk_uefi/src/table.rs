use crate::data_types::{
    Char16, EfiAllocateType, EfiGuid, EfiHandle, EfiMemoryDescriptor, EfiMemoryType,
    EfiPhysicalAddress, EfiStatus, EfiTpl,
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
    free_pages: extern "efiapi" fn(), // TODO
    get_memory_map: extern "efiapi" fn(
        memory_map_size: &mut usize,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,
    allocate_pool: extern "efiapi" fn(), // TODO
    free_pool: extern "efiapi" fn(),     // TODO

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
    handle_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface: &mut *mut c_void,
    ) -> EfiStatus,
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
    get_next_monotonic_count: extern "efiapi" fn(), // TODO
    stall: extern "efiapi" fn(microseconds: usize) -> EfiStatus, // TODO
    set_watchdog_timer: extern "efiapi" fn(),       // TODO

    // DriverSupport Services
    connect_controller: extern "efiapi" fn(),    // TODO
    disconnect_controller: extern "efiapi" fn(), // TODO

    // Open and Close Protocol Services
    open_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface: &mut *mut c_void,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus,
    close_protocol: extern "efiapi" fn(),            // TODO
    open_protocol_information: extern "efiapi" fn(), // TODO

    // Library Services
    protocols_per_handle: extern "efiapi" fn(), // TODO
    locate_handle_buffer: extern "efiapi" fn(), // TODO
    locate_protocol: extern "efiapi" fn(
        protocol: &EfiGuid,
        registration: *mut c_void,
        interface: &mut *mut c_void,
    ) -> EfiStatus,
    install_multiple_protocol_interfaces: extern "efiapi" fn(), // TODO
    uninstall_multiple_protocol_interfaces: extern "efiapi" fn(), // TODO

    // 32-bit CRC Services
    calculate_crc32: extern "efiapi" fn(), // TODO

    // Miscellaneous Services
    copy_mem: extern "efiapi" fn(),        // TODO
    set_mem: extern "efiapi" fn(),         // TODO
    create_event_ex: extern "efiapi" fn(), // TODO
}

impl EfiBootServices {
    pub fn allocate_pages(
        &self,
        type1: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: usize,
        memory: &mut EfiPhysicalAddress,
    ) -> EfiStatus {
        (self.allocate_pages)(type1, memory_type, pages, memory)
    }

    pub fn get_memory_map(
        &self,
        memory_map_size: &mut usize,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus {
        (self.get_memory_map)(
            memory_map_size,
            memory_map,
            map_key,
            descriptor_size,
            descriptor_version,
        )
    }

    pub fn handle_protocol(
        &self,
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface: &mut *mut c_void,
    ) -> EfiStatus {
        (self.handle_protocol)(handle, protocol, interface)
    }

    pub fn stall(&self, microseconds: usize) -> EfiStatus {
        (self.stall)(microseconds)
    }

    pub fn open_protocol(
        &self,
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface: &mut *mut c_void,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus {
        (self.open_protocol)(
            handle,
            protocol,
            interface,
            agent_handle,
            controller_handle,
            attributes,
        )
    }

    pub fn locate_protocol(
        &self,
        protocol: &EfiGuid,
        registration: *mut c_void,
        interface: &mut *mut c_void,
    ) -> EfiStatus {
        (self.locate_protocol)(protocol, registration, interface)
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
