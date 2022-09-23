use super::{ScreenArea, ScreenChar, ScreenPos, VgaBuffer, VgaBufferExt};

#[derive(Clone)]
pub struct Validator<T> {
    buffer: T,
}

impl<T: VgaBuffer> Validator<T> {
    pub fn new_from(buffer: T) -> Self {
        Self { buffer }
    }
}

impl<T: VgaBuffer> VgaBuffer for Validator<T> {
    #[inline]
    fn write_at(&mut self, pos: ScreenPos, c: ScreenChar) {
        debug_assert!(pos.row < self.area.rows);
        debug_assert!(pos.col < self.area.cols);
        self.buffer.write_at(pos, c);
    }

    #[inline]
    fn read_at(&self, pos: ScreenPos) -> ScreenChar {
        debug_assert!(pos.row < self.area.rows);
        debug_assert!(pos.col < self.area.cols);
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
