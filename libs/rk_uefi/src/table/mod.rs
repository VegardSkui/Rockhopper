mod system;
pub use system::*;

mod boot;
pub use boot::*;

mod runtime;
pub use runtime::*;

/// Data structure that precedes all of the standard EFI table types.
#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    _reserved: u32,
}
