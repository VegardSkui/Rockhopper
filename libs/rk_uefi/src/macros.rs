use crate::data_types::Char16;
use crate::system_table;
use core::fmt::{self, Write};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub struct Writer;

impl Write for Writer {
    fn write_str(&mut self, string: &str) -> Result<(), fmt::Error> {
        let con_out = system_table().con_out();

        // Output the string one character at a time
        for c in string.chars() {
            con_out.output_string(&[Char16(c as u16), Char16(0)][0]);

            // Carriage returns are required for proper newlines in UEFI
            if c == '\n' {
                con_out.output_string(&[Char16('\r' as u16), Char16(0)][0]);
            }
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    Writer.write_fmt(args).unwrap();
}
