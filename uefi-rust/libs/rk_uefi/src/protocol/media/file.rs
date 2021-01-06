#[derive(Copy, Clone)]
#[repr(C)]
pub struct EfiFileProtocol {
    revision: u64,
    open: extern "efiapi" fn(),         // TODO
    close: extern "efiapi" fn(),        // TODO
    delete: extern "efiapi" fn(),       // TODO
    read: extern "efiapi" fn(),         // TODO
    write: extern "efiapi" fn(),        // TODO
    get_position: extern "efiapi" fn(), // TODO
    set_position: extern "efiapi" fn(), // TODO
    get_info: extern "efiapi" fn(),     // TODO
    set_info: extern "efiapi" fn(),     // TODO
    flush: extern "efiapi" fn(),        /* TODO
                                         * TODO: Revision 2 additions */
}

impl EfiFileProtocol {}
