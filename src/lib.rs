#![no_std]
// Features
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
// Lints
#![allow(dead_code)]
#![deny(clippy::perf)]
#![deny(clippy::pedantic)]
#![deny(clippy::style)]
#![deny(clippy::complexity)]
#![deny(clippy::suspicious)]
#![deny(clippy::correctness)]
// Testing
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use bootloader::entry_point;
use bootloader::BootInfo;

// Barebones os
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
pub mod keyboard;

// Memory management
pub mod allocator;
pub mod memory;
extern crate alloc;

// Multitasking and Concurrency
pub mod concurrency;
pub mod task;

// Usability
pub mod shell;

pub fn init(boot_info: &'static BootInfo) {
    // Avoid bug where first two commands don't output
    println!("Booting cbos ...");
    gdt::init_gdt();
    exceptions::init_idt();
    interrupts::init_pic();
    memory::init(boot_info);
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
entry_point!(test_kernel_main);

/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    // like before
    init(boot_info);
    test_main();
    hal::hlt_loop();
}
