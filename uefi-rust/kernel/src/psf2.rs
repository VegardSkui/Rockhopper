#[allow(dead_code)]
pub const PSF2_MAGIC: u32 = 0x72b54a86;

/// PSF2 header.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Header {
    pub magic: u32,
    pub version: u32,
    pub headersize: u32,
    pub flags: u32,
    /// Number of glyphs.
    pub length: u32,
    /// Number of bytes for each character.
    pub charsize: u32,
    /// Max height of a character in pixels.
    pub height: u32,
    /// Max width of a character in pixels.
    pub width: u32,
}

pub struct Font {
    addr: u64,
}

impl Font {
    /// Initializes by reading a psf 2 font from the specified memory address.
    ///
    /// # Safety
    /// The address must point to a valid psf 2 font, which must stay at the
    /// specified address for as long as it's used.
    pub const unsafe fn read(addr: u64) -> Font {
        Font { addr }
    }

    /// Returns a pointer to the the font in memory.
    fn pointer(&self) -> *const u8 {
        self.addr as *const u8
    }

    /// Returns the font header.
    pub fn header(&self) -> Header {
        // This is safe as long as the address is still valid, which the user guaranteed
        // when initializing.
        unsafe { *(self.pointer() as *const Header) }
    }

    /// Returns a pointer to the glyph of the given character.
    pub fn glyph_ptr(&self, character: u32) -> *const u8 {
        // TODO: Make sure the character exists in the font. If not, return a not found
        // glyph, or error if the not found glyph doesn't exist either.
        // TODO: PSF2 unicode translation table.

        // Should be safe if the character exists in the font (TODO) and the font is
        // valid
        let offset = self.header().headersize + character * self.header().charsize;
        unsafe { self.pointer().add(offset as usize) }
    }

    /// Returns how many bytes encode each row in a character.
    pub fn bytes_per_line(&self) -> u32 {
        (self.header().width + 7) / 8
    }
}

// Get the font linked into the kernel.
extern "C" {
    static _binary_font_psf_start: u8;
}

lazy_static! {
    /// A font linked into the kernel.
    pub static ref FONT: Font =
        unsafe { Font::read(&_binary_font_psf_start as *const _ as u64) };
}
