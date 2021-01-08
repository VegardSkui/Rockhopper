use crate::data_types::{
    EfiAllocateType, EfiGuid, EfiHandle, EfiMemoryDescriptor, EfiMemoryType, EfiPhysicalAddress,
    EfiStatus, EfiTpl,
};
use crate::table::EfiTableHeader;
use core::ffi::c_void;

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
    free_pages: extern "efiapi" fn(memory: EfiPhysicalAddress, pages: usize) -> EfiStatus,
    get_memory_map: extern "efiapi" fn(
        memory_map_size: &mut usize,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,
    allocate_pool: extern "efiapi" fn(
        pool_type: EfiMemoryType,
        size: usize,
        buffer: &mut *mut c_void,
    ) -> EfiStatus,
    free_pool: extern "efiapi" fn(buffer: *mut c_void) -> EfiStatus,

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
    load_image: extern "efiapi" fn(),   // TODO
    start_image: extern "efiapi" fn(),  // TODO
    exit: extern "efiapi" fn(),         // TODO
    unload_image: extern "efiapi" fn(), // TODO
    exit_boot_services: extern "efiapi" fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,

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
    copy_mem: extern "efiapi" fn(destination: *mut c_void, source: *mut c_void, length: usize),
    set_mem: extern "efiapi" fn(),         // TODO
    create_event_ex: extern "efiapi" fn(), // TODO
}

impl EfiBootServices {
    /// Allocates memory pages.
    pub fn allocate_pages(
        &self,
        type1: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: usize,
    ) -> Result<EfiPhysicalAddress, EfiStatus> {
        let mut buffer_ptr = EfiPhysicalAddress(0);
        let status = (self.allocate_pages)(type1, memory_type, pages, &mut buffer_ptr);
        if status.is_error() {
            Err(status)
        } else {
            Ok(buffer_ptr)
        }
    }

    /// Frees memory pages.
    pub fn free_pages(&self, memory: EfiPhysicalAddress, pages: usize) -> EfiStatus {
        (self.free_pages)(memory, pages)
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

    /// Allocates pool memory.
    ///
    /// Size in bytes.
    pub fn allocate_pool(
        &self,
        pool_type: EfiMemoryType,
        size: usize,
    ) -> Result<*mut c_void, EfiStatus> {
        let mut buffer_ptr = core::ptr::null_mut();
        let status = (self.allocate_pool)(pool_type, size, &mut buffer_ptr);
        if status.is_error() {
            Err(status)
        } else {
            Ok(buffer_ptr)
        }
    }

    /// Returns pool memory to the system.
    ///
    /// The buffer must have been allocated by
    /// [allocate_pool](Self::allocate_pool).
    pub fn free_pool(&self, buffer: *mut c_void) -> EfiStatus {
        (self.free_pool)(buffer)
    }

    pub fn handle_protocol(
        &self,
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface: &mut *mut c_void,
    ) -> EfiStatus {
        (self.handle_protocol)(handle, protocol, interface)
    }

    /// Terminates all boot services.
    pub fn exit_boot_services(&self, image_handle: EfiHandle, map_key: usize) -> EfiStatus {
        (self.exit_boot_services)(image_handle, map_key)
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

    /// Copies the contents of one buffer to another.
    pub fn copy_mem(&self, destination: *mut c_void, source: *mut c_void, length: usize) {
        (self.copy_mem)(destination, source, length)
    }
}
