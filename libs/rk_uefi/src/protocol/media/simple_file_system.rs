use crate::data_types::EfiStatus;
use crate::protocol::EfiFileProtocol;

#[repr(C)]
pub struct EfiSimpleFileSystemProtocol {
    revision: u64,
    open_volume: extern "efiapi" fn(this: &Self, root: &mut *mut EfiFileProtocol) -> EfiStatus,
}

impl EfiSimpleFileSystemProtocol {
    pub fn open_volume(&self, root: &mut *mut EfiFileProtocol) -> EfiStatus {
        (self.open_volume)(self, root)
    }
}
