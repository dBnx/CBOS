use super::primitives::{ScreenArea, ScreenChar, ScreenPos};

pub trait VgaBuffer {
    fn write_at(&mut self, pos: ScreenPos, c: ScreenChar);
    fn read_at(&self, pos: ScreenPos) -> ScreenChar;

    fn cursor(&self) -> ScreenPos;
    fn set_cursor(&mut self, pos: ScreenPos);
    fn size(&self) -> ScreenArea;
}

pub trait VgaBufferExt: VgaBuffer {}

impl<T: VgaBuffer> VgaBufferExt for T {}
