use super::{ScreenArea, ScreenChar, ScreenPos, VgaBuffer, VgaBufferExt};

#[derive(Clone)]
pub struct SubareaWriter<T: VgaBuffer> {
    pos: ScreenPos,
    area: ScreenArea,
    column_position: u8,
    buffer: T,
}

impl<T: VgaBuffer> VgaBuffer for SubareaWriter<T> {
    #[inline]
    fn write_at(&mut self, pos: ScreenPos, c: ScreenChar) {
        debug_assert!(pos.row < self.area.rows);
        debug_assert!(pos.col < self.area.cols);
        let pos = ScreenPos {
            row: pos.row + self.pos.row,
            col: pos.col + self.pos.col,
        };
        self.buffer.write_at(pos, c);
    }

    #[inline]
    fn read_at(&self, pos: ScreenPos) -> ScreenChar {
        debug_assert!(pos.row < self.area.rows);
        debug_assert!(pos.col < self.area.cols);
        let pos = ScreenPos {
            row: pos.row + self.pos.row,
            col: pos.col + self.pos.col,
        };
        self.buffer.read_at(pos)
    }

    #[inline]
    fn size(&self) -> ScreenArea {
        self.area
    }

    #[inline]
    fn cursor(&self) -> ScreenPos {
        self.buffer.cursor()
    }
    #[inline]
    fn set_cursor(&mut self, pos: ScreenPos) {
        self.buffer.set_cursor(pos)
    }
}

impl<T: VgaBuffer> SubareaWriter<T> {
    pub fn new_from(buffer: T, start: u8, height: u8) -> Self {
        let ScreenArea { rows: _, cols } = buffer.size();
        let area = ScreenArea { rows: height, cols };
        let pos = ScreenPos { row: start, col: 0 };
        Self {
            pos,
            area,
            column_position: 0,
            buffer,
        }
    }
}

impl SubareaWriter<super::BasicWriter> {
    pub fn new_split(at: u8) -> (Self, Self) {
        let buffer = super::BasicWriter::new();
        let buffer2 = super::BasicWriter::new();
        let ScreenArea { rows, .. } = buffer.size();
        (
            Self::new_from(buffer, 0, at),
            Self::new_from(buffer2, at, rows - at),
        )
    }

    pub fn new_top_half() -> Self {
        let buffer = super::BasicWriter::new();
        let ScreenArea { rows, .. } = buffer.size();
        Self::new_from(buffer, 0, rows / 2)
    }

    pub fn new_bottom_half() -> Self {
        let buffer = super::BasicWriter::new();
        let ScreenArea { rows, .. } = buffer.size();
        Self::new_from(buffer, rows / 2, rows / 2)
    }

    pub fn new(start: u8, height: u8) -> Self {
        Self::new_from(super::BasicWriter::new(), start, height)
    }
}

impl<T: VgaBuffer> core::fmt::Write for SubareaWriter<T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}
