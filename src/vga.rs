//! BUG: The first line to *print* variants do not appear.
//! TODO: Last line of *print* variants are not used, when using Rolling*
//!
//! VGA Buffer: ~ 25 rows and 80 columns
//! 16b each with: 8b ASCII code point, 8b formatting: 4b foreground, 3b background, 1b Blink
//!
//! Rewrite everything: Expose only VgaBufferSubviews, that handle everything
//! subview.split_v() usw could be used.
//!
//! Lowest line for Status (OS, Time, ... Programs?)
//! Top: h splits for 2? programs

mod functions;
pub mod macros;
mod primitives;
mod writer;

pub use functions::*;
pub use primitives::*;
pub use writer::*;

const BUFFER_HEIGHT: u8 = 25;
const BUFFER_WIDTH: u8 = 80;

pub trait VgaBuffer: Clone + core::fmt::Write {
    fn write_at(&mut self, pos: ScreenPos, c: ScreenChar);
    fn read_at(&self, pos: ScreenPos) -> ScreenChar;

    fn cursor(&self) -> ScreenPos;
    fn set_cursor(&mut self, pos: ScreenPos);
    fn size(&self) -> ScreenArea;

    fn new_line(&mut self) {
        let mut cursor = self.cursor();
        cursor.row = core::cmp::min(cursor.row + 1, self.size().rows - 1);
        cursor.col = 0;
        self.set_cursor(cursor);
    }

    fn color_code(&self) -> ColorCode {
        Default::default()
    }

    fn clear_row(&mut self, row: u8) {
        let clear = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code(),
        };
        for col in 0..self.size().cols {
            let pos = ScreenPos { row, col };
            self.write_at(pos, clear);
        }
    }
}

pub trait VgaBufferExt: VgaBuffer {
    fn put_ascii(&mut self, ascii: u8) {
        let mut cursor = self.cursor();
        match ascii {
            b'\r' => {
                cursor.col = 0;
                self.set_cursor(cursor);
            }
            b'\n' => self.new_line(),
            ascii => {
                self.write_at(
                    cursor,
                    ScreenChar {
                        ascii_character: ascii,
                        color_code: self.color_code(),
                    },
                );
                self.increment_cursor();
            }
        }
    }

    fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\r' | b'\n' => self.put_ascii(byte),
                _ => self.put_ascii(0xfe),
            }
        }
    }
    fn println(&mut self, s: &str) {
        self.print(s);
        self.new_line();
    }

    fn increment_cursor(&mut self) {
        let size = self.size();
        let cursor = self.cursor();

        let col = cursor.col + 1;
        let pos = if col < size.cols {
            ScreenPos {
                row: cursor.row,
                col,
            }
        } else {
            ScreenPos {
                row: core::cmp::min(cursor.row + 1, size.rows - 1),
                col: 0,
            }
        };
        self.set_cursor(pos);
    }
}

impl<T: VgaBuffer> VgaBufferExt for T {}

//impl<T: VgaBuffer> core::fmt::Write for T {
