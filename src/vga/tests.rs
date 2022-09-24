pub use super::primitives::*;
pub use super::*;
pub use crate::*;

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    debug_assert!(s.len() < 80);

    // Just in case a interrupt prints something
    interrupts::without_interrupts(|| {
        println!("\r{}", s);
        let size = STDOUT.lock().size();

        for (i, c) in s.chars().enumerate() {
            let pos = ScreenPos {
                row: size.rows - 2,
                col: i as u8,
            };
            let screen_char = STDOUT.lock().read_at(pos);
            //.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}

#[ignore]
//#[test_case]
fn test_overflowing_println_output() {
    use x86_64::instructions::interrupts;

    let s = "This is a very long string, which never fits into a single line of the VGA buffer. I promise.";
    debug_assert!(s.len() > 80);

    // Just in case a interrupt prints something
    interrupts::without_interrupts(|| {
        println!("");
        println!("\r{}", s);
        //crate::util::sleep_for_some_time(100_000);
        let size = STDOUT.lock().size();

        for (i, c) in s.chars().enumerate() {
            let first_line: u8 = 1 - (i % 80) as u8;
            // TODO: Is pos correct?
            let pos = ScreenPos {
                row: size.rows - 2 - 1 * first_line,
                col: (i % 80) as u8,
            };
            let screen_char = STDOUT.lock().read_at(pos);
            //.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
