use super::{ScreenArea, ScreenChar, ScreenPos, VgaBuffer, VgaBufferExt};
use crate::vga::ColorCode;

/// Rolling may wrap Subarea, but not the other way!
#[derive(Clone)]
pub struct ColorWriter<T> {
    color_code: ColorCode,
    buffer: T,
}

impl<T: VgaBuffer> ColorWriter<T> {
    pub fn new_from(buffer: T) -> Self {
        Self {
            color_code: Default::default(),
            buffer,
        }
    }

    pub fn new_from_with(buffer: T, color_code: ColorCode) -> Self {
        Self { color_code, buffer }
    }

    #[inline]
    fn set_color_code(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
    }
}

impl<T: VgaBuffer> VgaBuffer for ColorWriter<T> {
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

    #[inline]
    fn color_code(&self) -> ColorCode {
        self.color_code
    }

    #[inline]
    fn new_line(&mut self) {
        self.buffer.new_line()
    }
}

impl<T: VgaBuffer> core::fmt::Write for ColorWriter<T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}
