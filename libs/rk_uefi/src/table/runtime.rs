use crate::table::EfiTableHeader;

/// Contains a table header and pointers to all of the runtime services.
#[repr(C)]
pub struct EfiRuntimeServices {
    hdr: EfiTableHeader,
    // TODO
}
