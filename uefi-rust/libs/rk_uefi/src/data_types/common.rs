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

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct EfiStatus(usize);

impl EfiStatus {
    /// The high bit, indicating an error status if set.
    ///
    /// This implementation assumes a 64-bit system.
    const ERROR_BIT: usize = 1 << 63;

    /// Returns whether the status code indicates an error.
    pub fn is_error(self) -> bool {
        // Check if the error bit is set
        self.0 & Self::ERROR_BIT != 0
    }

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
    pub const EFI_LOAD_ERROR: EfiStatus = EfiStatus(Self::ERROR_BIT | 1);
    pub const EFI_INVALID_PARAMETER: EfiStatus = EfiStatus(Self::ERROR_BIT | 2);
    pub const EFI_UNSUPPORTED: EfiStatus = EfiStatus(Self::ERROR_BIT | 3);
    pub const EFI_BAD_BUFFER_SIZE: EfiStatus = EfiStatus(Self::ERROR_BIT | 4);
    pub const EFI_BUFFER_TOO_SMALL: EfiStatus = EfiStatus(Self::ERROR_BIT | 5);
    pub const EFI_NOT_READY: EfiStatus = EfiStatus(Self::ERROR_BIT | 6);
    pub const EFI_DEVICE_ERROR: EfiStatus = EfiStatus(Self::ERROR_BIT | 7);
    pub const EFI_WRITE_PROTECTED: EfiStatus = EfiStatus(Self::ERROR_BIT | 8);
    pub const EFI_OUT_OF_RESOURCES: EfiStatus = EfiStatus(Self::ERROR_BIT | 9);
    pub const EFI_VOLUME_CORRUPTED: EfiStatus = EfiStatus(Self::ERROR_BIT | 10);
    pub const EFI_VOLUME_FULL: EfiStatus = EfiStatus(Self::ERROR_BIT | 11);
    pub const EFI_NO_MEDIA: EfiStatus = EfiStatus(Self::ERROR_BIT | 12);
    pub const EFI_MEDIA_CHANGED: EfiStatus = EfiStatus(Self::ERROR_BIT | 13);
    pub const EFI_NOT_FOUND: EfiStatus = EfiStatus(Self::ERROR_BIT | 14);
    pub const EFI_ACCESS_DENIED: EfiStatus = EfiStatus(Self::ERROR_BIT | 15);
    pub const EFI_NO_RESPONSE: EfiStatus = EfiStatus(Self::ERROR_BIT | 16);
    pub const EFI_NO_MAPPING: EfiStatus = EfiStatus(Self::ERROR_BIT | 17);
    pub const EFI_TIMEOUT: EfiStatus = EfiStatus(Self::ERROR_BIT | 18);
    pub const EFI_NOT_STARTED: EfiStatus = EfiStatus(Self::ERROR_BIT | 19);
    pub const EFI_ALREADY_STARTED: EfiStatus = EfiStatus(Self::ERROR_BIT | 20);
    pub const EFI_ABORTED: EfiStatus = EfiStatus(Self::ERROR_BIT | 21);
    pub const EFI_ICMP_ERROR: EfiStatus = EfiStatus(Self::ERROR_BIT | 22);
    pub const EFI_TFTP_ERROR: EfiStatus = EfiStatus(Self::ERROR_BIT | 23);
    pub const EFI_PROTOCOL_ERROR: EfiStatus = EfiStatus(Self::ERROR_BIT | 24);
    pub const EFI_INCOMPATIBLE_VERSION: EfiStatus = EfiStatus(Self::ERROR_BIT | 25);
    pub const EFI_SECURITY_VIOLATION: EfiStatus = EfiStatus(Self::ERROR_BIT | 26);
    pub const EFI_CRC_ERROR: EfiStatus = EfiStatus(Self::ERROR_BIT | 27);
    pub const EFI_END_OF_MEDIA: EfiStatus = EfiStatus(Self::ERROR_BIT | 28);
    pub const EFI_END_OF_FILE: EfiStatus = EfiStatus(Self::ERROR_BIT | 31);
    pub const EFI_INVALID_LANGUAGE: EfiStatus = EfiStatus(Self::ERROR_BIT | 32);
    pub const EFI_COMPROMISED_DATA: EfiStatus = EfiStatus(Self::ERROR_BIT | 33);
    pub const EFI_IP_ADDRESS_CONFLICT: EfiStatus = EfiStatus(Self::ERROR_BIT | 34);
    pub const EFI_HTTP_ERROR: EfiStatus = EfiStatus(Self::ERROR_BIT | 35);
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
            Self::EFI_LOAD_ERROR => write!(f, "EFI_LOAD_ERROR"),
            Self::EFI_INVALID_PARAMETER => write!(f, "EFI_INVALID_PARAMETER"),
            Self::EFI_UNSUPPORTED => write!(f, "EFI_UNSUPPORTED"),
            Self::EFI_BAD_BUFFER_SIZE => write!(f, "EFI_BAD_BUFFER_SIZE"),
            Self::EFI_BUFFER_TOO_SMALL => write!(f, "EFI_BUFFER_TOO_SMALL"),
            Self::EFI_NOT_READY => write!(f, "EFI_NOT_READY"),
            Self::EFI_DEVICE_ERROR => write!(f, "EFI_DEVICE_ERROR"),
            Self::EFI_WRITE_PROTECTED => write!(f, "EFI_WRITE_PROTECTED"),
            Self::EFI_OUT_OF_RESOURCES => write!(f, "EFI_OUT_OF_RESOURCES"),
            Self::EFI_VOLUME_CORRUPTED => write!(f, "EFI_VOLUME_CORRUPTED"),
            Self::EFI_VOLUME_FULL => write!(f, "EFI_VOLUME_FULL"),
            Self::EFI_NO_MEDIA => write!(f, "EFI_NO_MEDIA"),
            Self::EFI_MEDIA_CHANGED => write!(f, "EFI_MEDIA_CHANGED"),
            Self::EFI_NOT_FOUND => write!(f, "EFI_NOT_FOUND"),
            Self::EFI_ACCESS_DENIED => write!(f, "EFI_ACCESS_DENIED"),
            Self::EFI_NO_RESPONSE => write!(f, "EFI_NO_RESPONSE"),
            Self::EFI_NO_MAPPING => write!(f, "EFI_NO_MAPPING"),
            Self::EFI_TIMEOUT => write!(f, "EFI_TIMEOUT"),
            Self::EFI_NOT_STARTED => write!(f, "EFI_NOT_STARTED"),
            Self::EFI_ALREADY_STARTED => write!(f, "EFI_ALREADY_STARTED"),
            Self::EFI_ABORTED => write!(f, "EFI_ABORTED"),
            Self::EFI_ICMP_ERROR => write!(f, "EFI_ICMP_ERROR"),
            Self::EFI_TFTP_ERROR => write!(f, "EFI_TFTP_ERROR"),
            Self::EFI_PROTOCOL_ERROR => write!(f, "EFI_PROTOCOL_ERROR"),
            Self::EFI_INCOMPATIBLE_VERSION => write!(f, "EFI_INCOMPATIBLE_VERSION"),
            Self::EFI_SECURITY_VIOLATION => write!(f, "EFI_SECURITY_VIOLATION"),
            Self::EFI_CRC_ERROR => write!(f, "EFI_CRC_ERROR"),
            Self::EFI_END_OF_MEDIA => write!(f, "EFI_END_OF_MEDIA"),
            Self::EFI_END_OF_FILE => write!(f, "EFI_END_OF_FILE"),
            Self::EFI_INVALID_LANGUAGE => write!(f, "EFI_INVALID_LANGUAGE"),
            Self::EFI_COMPROMISED_DATA => write!(f, "EFI_COMPROMISED_DATA"),
            Self::EFI_IP_ADDRESS_CONFLICT => write!(f, "EFI_IP_ADDRESS_CONFLICT"),
            Self::EFI_HTTP_ERROR => write!(f, "EFI_HTTP_ERROR"),

            // Unknown Codes
            Self(unknown) => write!(f, "EfiStatus({})", unknown),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct EfiHandle(pub *const c_void);

#[repr(transparent)]
pub struct EfiEvent(*const c_void);

#[repr(transparent)]
pub struct EfiTpl(usize);
