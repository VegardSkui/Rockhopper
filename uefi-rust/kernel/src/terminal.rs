use crate::graphics::Screen;

mod psf2 {
    #[allow(dead_code)]
    const PSF2_MAGIC0: u8 = 0x72;
    #[allow(dead_code)]
    const PSF2_MAGIC1: u8 = 0xb5;
    #[allow(dead_code)]
    const PSF2_MAGIC2: u8 = 0x4a;
    #[allow(dead_code)]
    const PSF2_MAGIC3: u8 = 0x86;

    /// PSF2 header.
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct Header {
        pub magic: [u8; 4],
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
        font_ptr: *const u8,
        header: Header,
    }

    impl Font {
        /// Loads a PSF2 font from memory.
        ///
        /// # Safety
        /// The pointer must point to a valid PSF2 font.
        pub unsafe fn load(font_ptr: *const u8) -> Self {
            let header = *(font_ptr as *const Header);
            Self { font_ptr, header }
        }

        /// Returns the font header.
        pub fn header(&self) -> Header {
            self.header
        }

        /// Returns a pointer to the glyph of the given character.
        pub fn glyph_ptr(&self, character: u32) -> *const u8 {
            // TODO: Make sure the character exists in the font. If not, return a not found
            // glyph, or error if the not found glyph doesn't exist either.
            // TODO: PSF2 unicode translation table.

            // Should be safe if the character exists in the font (TODO) and the font is
            // valid
            unsafe {
                self.font_ptr
                    .offset((self.header.headersize + character * self.header.charsize) as isize)
            }
        }

        /// Returns how many bytes encode each row in the character.
        pub fn bytes_per_line(&self) -> u32 {
            (self.header.width + 7) / 8
        }
    }
}

// Get the font linked into the kernel
extern "C" {
    pub static _binary_font_psf_start: u8;
}

pub struct Terminal {
    screen: Screen,
    /// The screen width in characters.
    cwidth: u32,
    /// The screen height in characters.
    cheight: u32,
    /// Horizontal cursor position, in characters.
    cx: u32,
    /// Vertical cursor position, in characters.
    cy: u32,
    /// The font this terminal uses
    font: psf2::Font,
}

impl Terminal {
    pub fn new(screen: Screen) -> Self {
        // Should be safe if the font is valid and linked correctly
        let font_ptr = unsafe { &_binary_font_psf_start as *const u8 };
        let font = unsafe { psf2::Font::load(font_ptr) };

        Self {
            screen,
            cwidth: screen.horizontal_resolution() / font.header().width,
            cheight: screen.vertical_resolution() / font.header().height,
            cx: 0,
            cy: 0,
            font,
        }
    }

    /// Puts a character at the current position.
    pub fn put_char(&mut self, character: u32) {
        // Insert a new line if the cursor is to the right of the screen
        if self.cx >= self.cwidth {
            self.new_line();
        }

        let bytes_per_line = self.font.bytes_per_line();
        let mut glyph_ptr = self.font.glyph_ptr(character);

        // Calculate the pixel offset of the top left corner of the character
        let offset_y = self.cy * self.font.header().height;
        let offset_x = self.cx * self.font.header().width;

        // Draw the character to the screen
        let mut mask: u32;
        for y in 0..self.font.header().height {
            // Reset the mask for this line
            mask = 1 << (self.font.header().width - 1);

            for x in 0..self.font.header().width {
                let pixel: u32;
                if unsafe { *(glyph_ptr as *const u32) } & mask == 0 {
                    // Background
                    pixel = 0x00333333;
                } else {
                    // Foreground
                    pixel = 0x00ffffff;
                }

                // TODO: Using put_pixel each time is quite inefficient since it calculates the
                // whole offset into the frame buffer each time (if we wrote directly to the
                // frame buffer we could just add 4 bytes to the offset for each iteration of x)
                self.screen.put_pixel(offset_x + x, offset_y + y, pixel);

                // Adjust the mask for the next pixel
                mask >>= 1;
            }

            // Adjust the glyph pointer for the next line
            glyph_ptr = unsafe { glyph_ptr.offset(bytes_per_line as isize) };
        }

        // Move the cursor one step to the right
        self.cx += 1;
    }

    pub fn new_line(&mut self) {
        self.cy += 1;
        self.cx = 0;

        // If we reach the end of the screen, clear the screen and reset the cursor to
        // the top left
        // TODO: Implement a text buffer and scroll instead.
        if self.cy >= self.cheight {
            self.screen.clear();
            self.cy = 0;
        }
    }
}
