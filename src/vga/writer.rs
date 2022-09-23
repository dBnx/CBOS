use super::{RawBuffer, ScreenArea, ScreenChar, ScreenPos, VgaBuffer, VgaBufferExt};

mod color;
mod rolling;
mod subarea;
pub use color::ColorWriter;
pub use rolling::RollingWriter;
pub use subarea::SubareaWriter;

pub struct BasicWriter {
    cursor: ScreenPos,
    buffer: &'static mut RawBuffer,
}

impl core::fmt::Write for BasicWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

impl VgaBuffer for BasicWriter {
    #[inline]
    fn write_at(&mut self, pos: ScreenPos, c: ScreenChar) {
        self.buffer.write_at(pos, c);
    }

    #[inline]
    fn read_at(&self, pos: ScreenPos) -> ScreenChar {
        self.buffer.read_at(pos)
    }

    #[inline]
    fn cursor(&self) -> ScreenPos {
        self.cursor
    }

    #[inline]
    fn set_cursor(&mut self, pos: ScreenPos) {
        self.cursor = pos;
    }

    #[inline]
    fn size(&self) -> ScreenArea {
        self.buffer.get_size()
    }
}

impl Clone for BasicWriter {
    fn clone(&self) -> Self {
        Self {
            cursor: self.cursor,
            buffer: RawBuffer::get_global_vga_buffer(),
        }
    }
}

impl BasicWriter {
    pub fn new() -> Self {
        //let mut buffer = unsafe { &mut *(0xb8000 as *mut crate::Buffer) };
        Self {
            cursor: ScreenPos::default(),
            buffer: RawBuffer::get_global_vga_buffer(),
        }
    }
}

impl Default for BasicWriter {
    fn default() -> Self {
        Self::new()
    }
}
