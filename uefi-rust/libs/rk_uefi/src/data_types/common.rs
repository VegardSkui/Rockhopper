//! Common UEFI data types from Table 5 of the UEFI Specification, Version 2.8.

use core::ffi::c_void;
use core::fmt;

/// A 1-byte character using the ISO-Latin-1 character set.
#[repr(transparent)]
pub struct Char8(pub u8);

/// A 2-byte character stored in the UCS-2 encoding format.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Char16(pub u16);

#[repr(C)]
pub struct EfiGuid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct EfiStatus(usize);

impl EfiStatus {
    /// The high bit, indicating an error status if set.
    ///
    /// This implementation assumes a 64-bit system.
    const ERROR_BIT: usize = 1 << 63;

    // Success Codes
    pub const EFI_SUCCESS: EfiStatus = EfiStatus(0);

    // Warning Codes
    pub const EFI_WARN_UNKNOWN_GLYPH: EfiStatus = EfiStatus(1);
    pub const EFI_WARN_DELETE_FAILURE: EfiStatus = EfiStatus(2);
    pub const EFI_WARN_WRITE_FAILURE: EfiStatus = EfiStatus(3);
    pub const EFI_WARN_BUFFER_TOO_SMALL: EfiStatus = EfiStatus(4);
    pub const EFI_WARN_STALE_DATA: EfiStatus = EfiStatus(5);
    pub const EFI_WARN_FILE_SYSTEM: EfiStatus = EfiStatus(6);
    pub const EFI_WARN_RESET_REQUIRED: EfiStatus = EfiStatus(7);

    // Error Codes
    pub const EFI_INVALID_PARAMETER: EfiStatus = EfiStatus(Self::ERROR_BIT | 2);
    pub const EFI_BUFFER_TOO_SMALL: EfiStatus = EfiStatus(Self::ERROR_BIT | 5);
    pub const EFI_OUT_OF_RESOURCES: EfiStatus = EfiStatus(Self::ERROR_BIT | 9);
}

impl fmt::Debug for EfiStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Success Codes
            Self::EFI_SUCCESS => write!(f, "EFI_SUCCESS"),

            // Warning Codes
            Self::EFI_WARN_UNKNOWN_GLYPH => write!(f, "EFI_WARN_UNKNOWN_GLYPH"),
            Self::EFI_WARN_DELETE_FAILURE => write!(f, "EFI_WARN_DELETE_FAILURE"),
            Self::EFI_WARN_WRITE_FAILURE => write!(f, "EFI_WARN_WRITE_FAILURE"),
            Self::EFI_WARN_BUFFER_TOO_SMALL => write!(f, "EFI_WARN_BUFFER_TOO_SMALL"),
            Self::EFI_WARN_STALE_DATA => write!(f, "EFI_WARN_STALE_DATA"),
            Self::EFI_WARN_FILE_SYSTEM => write!(f, "EFI_WARN_FILE_SYSTEM"),
            Self::EFI_WARN_RESET_REQUIRED => write!(f, "EFI_WARN_RESET_REQUIRED"),

            // Error Codes
            Self::EFI_INVALID_PARAMETER => write!(f, "EFI_INVALID_PARAMETER"),
            Self::EFI_BUFFER_TOO_SMALL => write!(f, "EFI_BUFFER_TOO_SMALL"),
            Self::EFI_OUT_OF_RESOURCES => write!(f, "EFI_OUT_OF_RESOURCES"),

            // Unknown Codes
            Self(unknown) => write!(f, "EfiStatus({})", unknown),
        }
    }
}

#[repr(transparent)]
pub struct EfiHandle(pub *const c_void);

#[repr(transparent)]
pub struct EfiEvent(*const c_void);

#[repr(transparent)]
pub struct EfiTpl(usize);
