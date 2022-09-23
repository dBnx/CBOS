use super::{ScreenArea, ScreenChar, ScreenPos, VgaBuffer, VgaBufferExt};

/// Rolling may wrap Subarea, but not the other way!
#[derive(Clone)]
pub struct RollingWriter<T> {
    column_position: u8,
    buffer: T,
}

impl<T: VgaBuffer> RollingWriter<T> {
    pub fn new_from(buffer: T) -> Self {
        Self {
            column_position: 0,
            buffer,
        }
    }

    #[inline]
    fn set_cursor_on_last_line(&mut self) {
        let size = self.size();
        let ScreenPos { row: _, col } = self.cursor();
        self.set_cursor(ScreenPos {
            row: size.rows - 1,
            col,
        });
    }
}

impl<T: VgaBuffer> VgaBuffer for RollingWriter<T> {
    #[inline]
    fn write_at(&mut self, pos: ScreenPos, c: ScreenChar) {
        self.buffer.write_at(pos, c);
    }

    #[inline]
    fn read_at(&self, pos: ScreenPos) -> ScreenChar {
        self.buffer.read_at(pos)
    }

    #[inline]
    fn size(&self) -> ScreenArea {
        self.buffer.size()
    }

    #[inline]
    fn cursor(&self) -> ScreenPos {
        self.buffer.cursor()
    }

    #[inline]
    fn set_cursor(&mut self, pos: ScreenPos) {
        self.buffer.set_cursor(pos)
    }

    fn new_line(&mut self) {
        self.set_cursor_on_last_line();

        let size = self.size();
        for row in 1..size.rows {
            for col in 0..size.cols {
                let c = self.read_at(ScreenPos { row, col });
                self.write_at(ScreenPos { row: row - 1, col }, c);
            }
        }
        self.clear_row(size.rows - 1);

        let mut cursor = self.cursor();
        cursor.col = 0;
        self.set_cursor(cursor);
    }
}

impl<T: VgaBuffer> core::fmt::Write for RollingWriter<T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}
