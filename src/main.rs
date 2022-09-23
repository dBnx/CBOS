#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![no_std]
#![no_main]
#![allow(dead_code)]
//#![deny(unsafe_code)]

#[cfg(test)]
mod tests;
mod vga;
use core::panic::PanicInfo;
pub use vga::macros::*;
//mod prelude;
//pub use prelude::*;

fn run() {
    println!("Test");
    println!("Test 1: {}", "2");
    println!("Test 2: {}", "2");
    println!("Test 3: {}", "2");
    println!("Test 4: {}", "2");
    eprintln!("Test");
    eprintln!("Test 1: {}", "2");
    eprintln!("Test 2: {}", "2");
    eprintln!("Test 3: {}", "2");
    eprintln!("Test 4: {}", "2");
    loop {
        //core::hint::spin_loop();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    tests::test_main();

    run();
    panic!("Damnit. Something wen't wrong");
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{}", info);
    loop {
        //core::hint::spin_loop();
    }
}
