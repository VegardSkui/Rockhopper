use crate::data_types::{EfiEvent, EfiStatus};

/// Protocol interfaces for devices that support simple console style text
/// input.
#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    reset: extern "efiapi" fn(&Self, bool) -> EfiStatus,
    read_key_stroke: extern "efiapi" fn(&Self, &mut EfiInputKey) -> EfiStatus,
    wait_for_key: EfiEvent,
}

#[repr(C)]
pub struct EfiInputKey {
    scan_code: u16,
    unicode_char: u16,
}
