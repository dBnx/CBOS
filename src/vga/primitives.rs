use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
//pub enum FGColor {
pub enum Color {
    // Colors
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    // Bright variant
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/*
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BGColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Blink {
    False = 0,
    True = 1,
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
    pub fn get_bg(&self) -> Color {
        unsafe { core::mem::transmute::<u8, Color>(self.0 >> 4) }
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        ColorCode::new(Color::Yellow, Color::Black)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct ScreenPos {
    pub row: u8,
    pub col: u8,
}

impl ScreenPos {
    pub fn increment(&self, max_cols: u8) -> Self {
        let col = self.col + 1;
        if col < max_cols {
            ScreenPos {
                row: self.row,
                col: self.col + 1,
            }
        } else {
            self.newline()
        }
    }

    pub fn newline(&self) -> Self {
        ScreenPos {
            row: self.row + 1,
            col: 0,
        }
    }

    pub fn cr(&self) -> Self {
        ScreenPos {
            row: self.row,
            col: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenArea {
    pub rows: u8,
    pub cols: u8,
}

impl Default for ScreenArea {
    fn default() -> Self {
        Self {
            rows: super::BUFFER_ROWS,
            cols: super::BUFFER_COLS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

impl ScreenChar {
    pub fn empty_with_background(bg: Color) -> Self {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Black, bg),
        }
    }

    #[inline]
    pub fn empty() -> Self {
        ScreenChar::empty_with_background(ColorCode::default().get_bg())
    }

    #[inline]
    pub fn from_u8(character: u8) -> Self {
        ScreenChar {
            ascii_character: character,
            color_code: Default::default(),
        }
    }
}

#[repr(transparent)]
pub struct RawBuffer {
    pub chars: [[Volatile<ScreenChar>; super::BUFFER_COLS as usize]; super::BUFFER_ROWS as usize],
}

impl RawBuffer {
    #[inline]
    pub fn write_at(&mut self, pos: ScreenPos, c: ScreenChar) {
        self.chars[pos.row as usize][pos.col as usize].write(c);
    }

    #[inline]
    pub fn read_at(&self, pos: ScreenPos) -> ScreenChar {
        self.chars[pos.row as usize][pos.col as usize].read()
    }

    #[inline]
    pub fn get_size(&self) -> ScreenArea {
        Default::default()
    }

    #[inline]
    pub fn get_global_vga_buffer() -> &'static mut RawBuffer {
        unsafe { &mut *(0xb8000 as *mut RawBuffer) }
    }
}
