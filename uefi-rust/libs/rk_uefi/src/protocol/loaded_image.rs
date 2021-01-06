use crate::data_types::{EfiHandle, EfiMemoryType, EfiStatus};
use crate::protocol::EfiDevicePathProtocol;
use crate::table::EfiSystemTable;
use core::ffi::c_void;

#[repr(C)]
pub struct EfiLoadedImageProtocol {
    revision: u32,
    parent_handle: EfiHandle,
    system_table: *const EfiSystemTable,

    device_handle: EfiHandle,
    file_path: *const EfiDevicePathProtocol,
    _reserved: *const c_void,

    load_options_size: u32,
    load_options: *const c_void,

    image_base: *const c_void,
    image_size: u64,
    image_code_type: EfiMemoryType,
    image_data_type: EfiMemoryType,
    unload: extern "efiapi" fn(image_handle: EfiHandle) -> EfiStatus,
}

impl EfiLoadedImageProtocol {
    pub fn device_handle(&self) -> EfiHandle {
        self.device_handle
    }
}
