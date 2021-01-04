//! Common UEFI data types from Table 5 of the UEFI Specification, Version 2.8.

use core::ffi::c_void;

/// A 1-byte character using the ISO-Latin-1 character set.
#[repr(transparent)]
pub struct Char8(pub u8);

/// A 2-byte character stored in the UCS-2 encoding format.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Char16(pub u16);

#[repr(C)]
pub struct EfiGuid(pub u32, pub u16, pub u16, pub [u8; 8]);

#[repr(transparent)]
pub struct EfiStatus(pub usize);

#[repr(transparent)]
pub struct EfiHandle(pub *const c_void);

#[repr(transparent)]
pub struct EfiEvent(*const c_void);
