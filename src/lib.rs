#![no_std]
// Features
#![feature(abi_x86_interrupt)]
// Lints
#![allow(dead_code)]
// Testing
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod exceptions;
pub mod gdt;
pub mod hal;
pub mod interrupts;
pub mod serial;
pub mod util;
#[macro_use]
pub mod vga;
#[macro_use]
pub mod prelude;
pub use prelude::*;

pub fn init() {
    // Avoid bug where first two commands don't output
    println!("<>");
    eprintln!("<>");
    kprintln!("<>");
    kprintln!("<>");

    println!("Booting cbos ...");
    gdt::init_gdt();
    exceptions::init_idt();
    interrupts::init_pic();
}

pub mod tests;
#[cfg(test)]
use tests::test_runner;

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    tests::test_panic_handler(info)
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hal::hlt_loop();
}
