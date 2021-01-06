use crate::data_types::{Char16, EfiGuid, EfiStatus, EfiTime};
use core::ffi::c_void;

pub const EFI_FILE_MODE_READ: u64 = 1;
pub const EFI_FILE_MODE_WRITE: u64 = 1 << 1;
pub const EFI_FILE_MODE_CREATE: u64 = 1 << 63;

pub const EFI_FILE_READ_ONLY: u64 = 0x1;
pub const EFI_FILE_HIDDEN: u64 = 0x2;
pub const EFI_FILE_SYSTEM: u64 = 0x4;
pub const EFI_FILE_RESERVED: u64 = 0x8;
pub const EFI_FILE_DIRECTORY: u64 = 0x10;
pub const EFI_FILE_ARCHIVE: u64 = 0x20;
pub const EFI_FILE_VALID_ATTR: u64 = 0x37;

pub const EFI_FILE_INFO_ID: EfiGuid = EfiGuid(
    0x09576e92,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const EFI_FILE_SYSTEM_INFO_ID: EfiGuid = EfiGuid(
    0x09576e93,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct EfiFileProtocol {
    revision: u64,
    open: extern "efiapi" fn(),         // TODO
    close: extern "efiapi" fn(this: &Self) -> EfiStatus,
    delete: extern "efiapi" fn(this: &Self) -> EfiStatus,
    read:
        extern "efiapi" fn(this: &Self, buffer_size: &mut usize, buffer: &mut c_void) -> EfiStatus,
    write: extern "efiapi" fn(),        // TODO
    get_position: extern "efiapi" fn(), // TODO
    set_position: extern "efiapi" fn(), // TODO
    get_info: extern "efiapi" fn(
        this: &Self,
        information_type: &EfiGuid,
        buffer_size: &mut usize,
        buffer: &mut c_void,
    ) -> EfiStatus,
    set_info: extern "efiapi" fn(), // TODO
    flush: extern "efiapi" fn(),    /* TODO
                                     * TODO: Revision 2 additions */
}

impl EfiFileProtocol {
    /// Returns the revision.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Closes the file.
    pub fn close(&self) -> EfiStatus {
        (self.close)(self)
    }

    /// Deletes the file.
    pub fn delete(&self) -> EfiStatus {
        (self.delete)(self)
    }

    /// Reads bytes from the file.
    pub fn read(&self, buffer_size: &mut usize, buffer: &mut c_void) -> EfiStatus {
        (self.read)(self, buffer_size, buffer)
    }

    pub fn get_info(
        &self,
        information_type: &EfiGuid,
        buffer_size: &mut usize,
        buffer: &mut c_void,
    ) -> EfiStatus {
        (self.get_info)(self, information_type, buffer_size, buffer)
    }
}

#[repr(C)]
pub struct EfiFileInfo {
    size: u64,
    file_size: u64,
    physical_size: u64,
    create_time: EfiTime,
    last_access_time: EfiTime,
    modification_time: EfiTime,
    attribute: u64,
    file_name: [Char16; 50], // FIXME: This will lead to errors with longer filenames...
}

#[repr(C)]
pub struct EfiFileSystemInfo {
    size: u64,
    read_only: bool,
    volume_size: u64,
    free_space: u64,
    block_size: u32,
    pub volume_label: [Char16; 50], // FIXME: This will lead to errors with longer volume labels...
}
