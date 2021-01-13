use crate::data_types::{EfiPhysicalAddress, EfiStatus};

#[repr(C)]
pub struct EfiGraphicsOutputProtocol {
    query_mode: extern "efiapi" fn(
        this: &Self,
        mode_number: u32,
        size_of_info: &mut usize,
        info: &mut *mut EfiGraphicsOutputModeInformation,
    ) -> EfiStatus,
    set_mode: extern "efiapi" fn(this: &Self, mode_number: u32) -> EfiStatus,
    blt: extern "efiapi" fn(), // TODO
    mode: *const EfiGraphicsOutputProtocolMode,
}

impl EfiGraphicsOutputProtocol {
    /// Returns information about an available graphics mode.
    ///
    /// If an error occurs, the status code is provided.
    pub fn query_mode(
        &self,
        mode_number: u32,
    ) -> Result<&EfiGraphicsOutputModeInformation, EfiStatus> {
        let mut size_of_info: usize = 0;
        let mut ptr = core::ptr::null_mut();
        let status = (self.query_mode)(self, mode_number, &mut size_of_info, &mut ptr);

        if status.is_error() {
            Err(status)
        } else {
            Ok(unsafe { &*ptr })
        }
    }

    pub fn set_mode(&self, mode_number: u32) -> EfiStatus {
        (self.set_mode)(self, mode_number)
    }

    pub fn mode(&self) -> &EfiGraphicsOutputProtocolMode {
        unsafe { &*self.mode }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EfiPixelBitmask {
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    reserved_mask: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub enum EfiGraphicsPixelFormat {
    PixelRedGreenBlueReserved8BitPerColor,
    PixelBlueGreenRedReserved8BitPerColor,
    PixelBitMask,
    PixelBltOnly,
    PixelFormatMax,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EfiGraphicsOutputModeInformation {
    version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pixel_format: EfiGraphicsPixelFormat,
    pixel_information: EfiPixelBitmask,
    pub pixels_per_scan_line: u32,
}

#[repr(C)]
pub struct EfiGraphicsOutputProtocolMode {
    pub max_mode: u32,
    pub mode: u32,
    pub info: *const EfiGraphicsOutputModeInformation,
    size_of_info: usize,
    pub frame_buffer_base: EfiPhysicalAddress,
    frame_buffer_size: usize,
}
