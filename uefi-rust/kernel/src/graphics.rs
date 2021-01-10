#[derive(Copy, Clone)]
pub struct Screen {
    fb_base: u64,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixels_per_scan_line: u32,
}

impl Screen {
    /// Creates a new screen with the given frame buffer.
    ///
    /// # Safety
    /// `fb_base` must point to a valid linear frame buffer with the size given
    /// by the other arguments. Pixels must be 32 bits wide.
    pub unsafe fn new(
        fb_base: u64,
        horizontal_resolution: u32,
        vertical_resolution: u32,
        pixels_per_scan_line: u32,
    ) -> Self {
        Screen {
            fb_base,
            horizontal_resolution,
            vertical_resolution,
            pixels_per_scan_line,
        }
    }

    /// Returns the horizontal resolution of the screen.
    pub fn horizontal_resolution(self) -> u32 {
        self.horizontal_resolution
    }

    /// Returns the vertical resolution of the screen.
    pub fn vertical_resolution(self) -> u32 {
        self.vertical_resolution
    }

    /// Clears the screen (fills it with black pixels).
    pub fn clear(&self) {
        // This is safe assuming the screen parameters are correct.
        unsafe {
            core::ptr::write_bytes(
                self.fb_base as *mut u32,
                0,
                (self.pixels_per_scan_line * self.vertical_resolution) as usize,
            );
        }
    }

    /// Puts a single pixel on the screen at the specified coordinates.
    pub fn put_pixel(&self, x: u32, y: u32, pixel: u32) {
        // This should be safe as long as the screen buffer is valid
        unsafe {
            (self.fb_base as *mut u32)
                .offset((self.pixels_per_scan_line * y + x) as isize)
                .write(pixel);
        }
    }
}
