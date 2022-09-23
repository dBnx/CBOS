use crate::vga::{BasicWriter, ColorWriter, RollingWriter, SubareaWriter};
use crate::vga::{Color, ColorCode};
use lazy_static::lazy_static;
use spin::Mutex;

type VgaWriter = ColorWriter<RollingWriter<SubareaWriter<BasicWriter>>>;

lazy_static! {
    pub static ref STDOUT: Mutex<VgaWriter> = Mutex::new(ColorWriter::new_from_with(
        RollingWriter::new_from(SubareaWriter::new(0, 20)),
        ColorCode::default()
    ));
}
lazy_static! {
    pub static ref STDERR: Mutex<VgaWriter> = Mutex::new(ColorWriter::new_from_with(
        RollingWriter::new_from(SubareaWriter::new(20, 5)),
        ColorCode::new(Color::Black, Color::White)
    ));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    STDOUT.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ($crate::vga::macros::_eprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::eprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _eprint(args: core::fmt::Arguments) {
    use core::fmt::Write;
    STDERR.lock().write_fmt(args).unwrap();
}
