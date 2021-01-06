mod common;
pub use self::common::*;

mod memory;
pub use self::memory::*;

#[repr(C)]
pub struct EfiTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    _pad1: u8,
    nanosecond: u32,
    time_zone: i16,
    daylight: u8,
    _pad2: u8,
}
