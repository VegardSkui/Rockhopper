use crate::data_types::{Char16, EfiStatus};

/// Protocol interfaces for devices that support console style text displaying.
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    reset: extern "efiapi" fn(this: &Self, extended_verification: bool) -> EfiStatus,
    output_string: extern "efiapi" fn(this: &Self, string: &Char16) -> EfiStatus,
    test_string: extern "efiapi" fn(this: &Self, string: &Char16) -> EfiStatus,
    query_mode: extern "efiapi" fn(
        this: &Self,
        mode_number: usize,
        columns: &mut usize,
        rows: &mut usize,
    ) -> EfiStatus,
    set_mode: extern "efiapi" fn(this: &Self, mode_number: usize) -> EfiStatus,
    set_attribute: extern "efiapi" fn(this: &Self, attribute: usize) -> EfiStatus,
    clear_screen: extern "efiapi" fn(this: &Self) -> EfiStatus,
    set_cursor_position: extern "efiapi" fn(this: &Self, column: usize, row: usize) -> EfiStatus,
    enable_cursor: extern "efiapi" fn(this: &Self, visible: bool) -> EfiStatus,
    mode: *const EfiSimpleTextOutputMode,
}

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&mut self, extended_verification: bool) -> EfiStatus {
        (self.reset)(self, extended_verification)
    }

    pub fn output_string(&mut self, string: &Char16) -> EfiStatus {
        (self.output_string)(self, string)
    }
}

#[repr(C)]
pub struct EfiSimpleTextOutputMode {
    max_mode: i32,
    mode: i32,
    attribute: i32,
    cursor_column: i32,
    cursor_row: i32,
    cursor_visible: bool,
}
