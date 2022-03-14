/*
 * Gem Research Operating System
 * VGA Buffer
 * © 2022 Upper Altitude
 */

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    // A global `Writer` instance that can be used for printing to the VGA text buffer.
    // Used by the `print!` and `println!` macros.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightCyan, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/*************************
 *         Colors        *
 *************************/

// Represent the available VGA colors.
#[allow(dead_code)] // Disable warnings for unused variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Enable copy semantics.
#[repr(u8)] // Ensure that it has the same memory layout as its single field.
pub enum Color {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

// Create a `ColorCode` on top of u8 to represent a foreground and background color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  // Enable copy semantics.
#[repr(transparent)] // Ensure that it has the same memory layout as its single field.
struct ColorCode(u8);

impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 4 | (foreground as u8))
	}
}

/*************************
 *      Text Buffer
 *************************/

// A screen character in the VGA text buffer, consisting of an ASCII character and a `ColorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // Guarantee the struct’s fields are laid out exactly like in a C struct and thus guarantee the correct field ordering.
struct ScreenChar {
	ascii_character: u8,
	color_code: ColorCode,
}

// The height of the text buffer (normally 25 lines).
const BUFFER_HEIGHT: usize = 25;

// The width of the text buffer (normally 80 columns).
const BUFFER_WIDTH: usize = 80;

// A structure representing the VGA text buffer.
#[repr(transparent)] // Ensure that it has the same memory layout as its single field.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/*************************
 *        Printing
 *************************/

// A writer type that allows writing ASCII bytes and strings to an underlying `Buffer`.
// Wraps lines at `BUFFER_WIDTH`. Supports newline characters and implements the `core::fmt::Write` trait.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

// Use the Writer to modify the buffer's characters.
impl Writer {
    // Writes an ASCII byte to the buffer.
    // Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    // Writes the given ASCII string to the buffer.
    // Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does **not**
    // support strings with non-ASCII characters, since they can't be printed in the VGA text mode.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Valid ASCII byte or newline.
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // Fallback: Invalid or not an ASCII byte.
                // We print "■".
                _ => self.write_byte(0xfe),
            }

        }
    }

    // Shifts all lines one line up and clears the last row.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    // Clears a row by overwriting it with blank characters.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
