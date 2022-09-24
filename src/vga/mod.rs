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
use core::fmt::{Arguments, Write};
use x86_64::instructions::interrupts::without_interrupts;

static ROWS_FOR_STATUS: u8 = 1;
static ROWS_FOR_PROG: u8 = 20;
static ROWS_FOR_SPECIAL: u8 = BUFFER_ROWS - ROWS_FOR_PROG - ROWS_FOR_STATUS;

const BUFFER_ROWS: u8 = 25;
const BUFFER_COLS: u8 = 80;

mod area;
mod view;
//mod raw_buffer;
pub mod primitives;
mod traits;

#[cfg(test)]
mod tests;

use area::Area;
use primitives::{Color, ColorCode};
use view::View;

use lazy_static::lazy_static;
use spin::Mutex;
pub use traits::*;

//lazy_static! {
//    static ref WINDOW_STATUS: Mutex<Area> = {
//        let cc = ColorCode::new(Color::Black, Color::White);
//        Mutex::new(Area::new(0, ROWS_FOR_STATUS))
//    };
//}

// === Setup windows ===
lazy_static! {
    static ref WINDOW_STATUS: Mutex<Area> = Mutex::new(Area::new(0, ROWS_FOR_STATUS));
}

lazy_static! {
    static ref WINDOW_PROG: Mutex<Area> = Mutex::new(Area::new(ROWS_FOR_STATUS, ROWS_FOR_PROG));
}

lazy_static! {
    static ref WINDOW_SPECIAL: Mutex<Area> =
        Mutex::new(Area::new(ROWS_FOR_STATUS + ROWS_FOR_PROG, ROWS_FOR_SPECIAL));
}

// === Setup views ===
lazy_static! {
    static ref STATUS: Mutex<View> = {
        let cc = ColorCode::new(Color::Green, Color::Black);
        let mut view = View::new(&WINDOW_STATUS, cc);
        view.clear();
        Mutex::new(view)
    };
}

lazy_static! {
    pub static ref STDOUT: Mutex<View> = {
        let cc = ColorCode::default();
        let mut view = View::new(&WINDOW_PROG, cc);
        view.clear();
        Mutex::new(view)
    };
}

lazy_static! {
    pub static ref STDERR: Mutex<View> = {
        let cc = ColorCode::new(Color::LightRed, Color::Black);
        Mutex::new(View::new(&WINDOW_PROG, cc))
    };
}

lazy_static! {
    pub static ref KEROUT: Mutex<View> = {
        let cc = ColorCode::new(Color::Black, Color::White);
        let mut view = View::new(&WINDOW_SPECIAL, cc);
        view.clear();
        Mutex::new(view)
    };
}

#[macro_export]
macro_rules! set_status_line {
    ($($arg:tt)*) => ($crate::vga::_set_status_line(format_args!($($arg)*)));
}

pub fn _set_status_line(args: Arguments) {
    without_interrupts(|| {
        STATUS.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    // Could deadlock if a interrupts prints something
    without_interrupts(|| {
        STDOUT.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _eprint(args: core::fmt::Arguments) {
    without_interrupts(|| {
        STDERR.lock().write_fmt(args).unwrap();
    });
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::vga::_kprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _kprint(args: core::fmt::Arguments) {
    without_interrupts(|| {
        KEROUT.lock().write_fmt(args).unwrap();
    });
}
