use crate::data_types::{Char16, EfiEvent, EfiStatus};

/// Protocol interfaces for devices that support simple console style text
/// input.
#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    reset: extern "efiapi" fn(&EfiSimpleTextInputProtocol, bool) -> EfiStatus,
    read_key_stroke: extern "efiapi" fn(&EfiSimpleTextInputProtocol, &mut EfiInputKey) -> EfiStatus,
    wait_for_key: EfiEvent,
}

#[repr(C)]
pub struct EfiInputKey {
    scan_code: u16,
    unicode_char: u16,
}

/// Protocol interfaces for devices that support console style text displaying.
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    reset: extern "efiapi" fn(
        this: &EfiSimpleTextOutputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,
    output_string:
        extern "efiapi" fn(this: &EfiSimpleTextOutputProtocol, string: &Char16) -> EfiStatus,
    test_string:
        extern "efiapi" fn(this: &EfiSimpleTextOutputProtocol, string: &Char16) -> EfiStatus,
    query_mode: extern "efiapi" fn(
        this: &EfiSimpleTextOutputProtocol,
        mode_number: usize,
        columns: &mut usize,
        rows: &mut usize,
    ) -> EfiStatus,
    set_mode:
        extern "efiapi" fn(this: &EfiSimpleTextOutputProtocol, mode_mumber: usize) -> EfiStatus,
    set_attribute:
        extern "efiapi" fn(this: &EfiSimpleTextOutputProtocol, attribute: usize) -> EfiStatus,
    clear_screen: extern "efiapi" fn(this: &EfiSimpleTextOutputProtocol) -> EfiStatus,
    set_cursor_position: extern "efiapi" fn(
        this: &EfiSimpleTextOutputProtocol,
        column: usize,
        row: usize,
    ) -> EfiStatus,
    enable_cursor:
        extern "efiapi" fn(this: &EfiSimpleTextOutputProtocol, visible: bool) -> EfiStatus,
    mode: *const EfiSimpleTextOutputMode,
}

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&mut self, extended_verification: bool) -> EfiStatus {
        (self.reset)(self, extended_verification)
    }

    pub fn output_string(&mut self, string: &Char16) -> EfiStatus {
        (self.output_string)(self, string)
    }

    pub fn output_rust_string(&mut self, string: &str) -> EfiStatus {
        let mut buffer = [Char16(0); 100];
        // TODO: Error if string is longer than the buffer.
        for (i, c) in string.chars().enumerate() {
            buffer[i] = Char16(c as u16);
        }
        self.output_string(&buffer[0])
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
