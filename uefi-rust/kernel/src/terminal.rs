use crate::graphics::Screen;
use crate::psf2::FONT;
use core::fmt;

/// Text-based output.
pub struct Terminal<'a> {
    screen: &'a Screen,
    /// The screen width in characters.
    cwidth: u32,
    /// The screen height in characters.
    cheight: u32,
    /// Horizontal cursor position, in characters.
    cx: u32,
    /// Vertical cursor position, in characters.
    cy: u32,
}

impl<'a> Terminal<'a> {
    pub fn new(screen: &'a Screen) -> Self {
        Self {
            screen,
            cwidth: screen.horizontal_resolution() / FONT.header().width,
            cheight: screen.vertical_resolution() / FONT.header().height,
            cx: 0,
            cy: 0,
        }
    }

    /// Puts a character at the current position.
    pub fn put_char(&mut self, character: char) {
        // Recognize newline characters and exit early
        if character == '\n' {
            self.new_line();
            return;
        }

        // Insert a new line if the cursor is to the right of the screen
        if self.cx >= self.cwidth {
            self.new_line();
        }

        // Get the pointer to the glyph for the requested character. If the font doesn't
        // include the character, use the question mark instead. If the font cannot
        // represent the question mark either, panic.
        let mut glyph_ptr = FONT
            .glyph_ptr(character as u32)
            .unwrap_or_else(|_| FONT.glyph_ptr('?' as u32).unwrap());

        // Calculate the pixel offset of the top left corner of the character
        let offset_y = self.cy * FONT.header().height;
        let offset_x = self.cx * FONT.header().width;

        // Draw the character to the screen
        let mut mask: u32;
        let bytes_per_line = FONT.bytes_per_line();
        for y in 0..FONT.header().height {
            // Reset the mask for this line
            mask = 1 << (FONT.header().width - 1);

            for x in 0..FONT.header().width {
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

    /// Puts a string starting at the current cursor position.
    pub fn put_string(&mut self, s: &str) {
        for c in s.chars() {
            self.put_char(c);
        }
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

impl fmt::Write for Terminal<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.put_string(s);
        Ok(())
    }
}

/// Prints using the global terminal, should only be used through the print!
/// macro.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    crate::TERMINAL.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::terminal::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
