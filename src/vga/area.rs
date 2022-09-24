use super::primitives::{RawBuffer, ScreenArea, ScreenChar, ScreenPos};
use super::traits::VgaBuffer;

pub struct Area {
    row_offset: u8,
    rows: u8,
    cursor: ScreenPos,
    buffer: &'static mut RawBuffer,
}

impl Area {
    pub fn new_full_vga_buffer() -> Self {
        Area::new(0, super::BUFFER_ROWS)
    }

    pub fn new(row_offset: u8, rows: u8) -> Self {
        Self {
            row_offset,
            rows,
            cursor: ScreenPos::default(),
            buffer: RawBuffer::get_global_vga_buffer(),
        }
    }

    fn relative_to_global_pos(&self, pos: ScreenPos) -> ScreenPos {
        let ScreenPos { row, col } = pos;
        debug_assert!(col < super::BUFFER_COLS);
        debug_assert!(self.row_offset + row < super::BUFFER_ROWS);
        ScreenPos {
            row: self.row_offset + row,
            col,
        }
    }
}

impl VgaBuffer for Area {
    #[inline]
    fn write_at(&mut self, pos: ScreenPos, c: ScreenChar) {
        let absolute_position = self.relative_to_global_pos(pos);
        debug_assert!(absolute_position.row < super::BUFFER_ROWS);
        debug_assert!(absolute_position.col < super::BUFFER_COLS);
        self.buffer.write_at(absolute_position, c);
    }

    #[inline]
    fn read_at(&self, pos: ScreenPos) -> ScreenChar {
        let absolute_position = self.relative_to_global_pos(pos);
        debug_assert!(absolute_position.row < super::BUFFER_ROWS);
        debug_assert!(absolute_position.col < super::BUFFER_COLS);
        self.buffer.read_at(absolute_position)
    }

    #[inline]
    fn cursor(&self) -> ScreenPos {
        self.cursor
    }

    #[inline]
    fn set_cursor(&mut self, pos: ScreenPos) {
        // Cursor is relative to the area -> DON'T CONVERT
        self.cursor = pos;
    }

    #[inline]
    fn size(&self) -> ScreenArea {
        ScreenArea {
            rows: self.rows,
            cols: super::BUFFER_COLS,
        }
    }
}
