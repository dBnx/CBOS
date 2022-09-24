use super::area::Area;
use super::primitives::{ColorCode, ScreenArea, ScreenChar, ScreenPos};
use super::traits::VgaBuffer;
use super::Mutex;

pub struct View {
    color_code: ColorCode,
    area: &'static Mutex<Area>,
}

impl View {
    pub fn new(area: &'static Mutex<Area>, color_code: ColorCode) -> Self {
        Self { color_code, area }
    }

    /// Clears the whole area with the current `color_code`
    pub fn clear(&mut self) {
        let rows = self.area.lock().size().rows;
        for i in 0..rows {
            self.clear_row(i);
        }
    }

    /// Prints a `str` with the current `color_code`
    pub fn print(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\r' | b'\n' => self.put_byte(byte),
                _ => self.put_byte(0xfe),
            }
        }
    }

    /// Prints a line with the current `color_code`
    pub fn println(&mut self, s: &str) {
        self.print(s);
        self.put_byte(b'\n');
    }

    #[inline]
    pub fn set_cursor_on_last_line(&mut self) {
        let size = self.size();
        self.set_cursor(ScreenPos {
            row: size.rows - 1,
            col: 0,
        });
    }

    /// Shifts the content of the area up by one row
    pub fn shift_up(&mut self) {
        let size = self.size();
        for row in 1..size.rows {
            for col in 0..size.cols {
                let c = self.read_at(ScreenPos { row, col });
                self.write_at(ScreenPos { row: row - 1, col }, c);
            }
        }
        self.clear_row(size.rows - 1);
    }

    pub fn clear_row(&mut self, row: u8) {
        let clear = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..self.size().cols {
            let pos = ScreenPos { row, col };
            self.write_at(pos, clear);
        }
    }

    // -----------------------------------------------------------------------------
    //
    fn new_line(&mut self) {
        let mut cursor = self.cursor();
        let size = self.size();
        // TODO: cmp should not be needed
        cursor.row = core::cmp::min(cursor.row + 1, self.size().rows);
        cursor.col = 0;
        if cursor.row >= size.rows {
            self.shift_up();
            cursor.row = size.rows - 1;
        }
        self.set_cursor(cursor);
    }

    fn increment_cursor(&mut self) {
        let ScreenPos { mut row, col } = self.cursor();
        let size = self.size();
        let mut col = col + 1;
        if col >= size.cols {
            col = 0;
            row = row + 1;
        }
        if row >= size.rows {
            self.shift_up();
            row = size.rows - 1;
        }
        self.set_cursor(ScreenPos { row, col });
    }

    pub fn put_byte(&mut self, ascii: u8) {
        let mut cursor = self.cursor();
        match ascii {
            b'\r' => {
                cursor.col = 0;
                self.set_cursor(cursor)
            }
            b'\n' => self.new_line(),
            ascii => {
                self.write_at(
                    cursor,
                    ScreenChar {
                        ascii_character: ascii,
                        color_code: self.color_code,
                    },
                );
                self.increment_cursor();
            }
        }
    }
}

impl VgaBuffer for View {
    #[inline]
    fn write_at(&mut self, pos: ScreenPos, c: ScreenChar) {
        self.area.lock().write_at(pos, c);
    }

    #[inline]
    fn read_at(&self, pos: ScreenPos) -> ScreenChar {
        self.area.lock().read_at(pos)
    }

    #[inline]
    fn cursor(&self) -> ScreenPos {
        self.area.lock().cursor()
    }

    #[inline]
    fn set_cursor(&mut self, pos: ScreenPos) {
        self.area.lock().set_cursor(pos);
    }

    #[inline]
    fn size(&self) -> ScreenArea {
        self.area.lock().size()
    }
}

impl core::fmt::Write for View {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}
