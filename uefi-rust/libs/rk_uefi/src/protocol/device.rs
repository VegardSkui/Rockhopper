#[repr(C)]
pub struct EfiDevicePathProtocol {
    type1: u8,
    sub_type: u8,
    length: [u8; 2],
}
