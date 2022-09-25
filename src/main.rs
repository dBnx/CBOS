#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]
use bootloader::{entry_point, BootInfo};
use cbos::*;
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(test)]
use cbos::tests::*;

extern crate alloc;

fn run() {
    crate::set_status_line!(
        "<CBOS> [1][2][3]<4>[5][6]                                                  12:13"
    );
    serial_println!("Test");
    //cbos::interrupts::KEYBOARD.lock().process_keyevent(ev);
    cbos::shell::run();
    cbos::hal::hlt_loop();
}

// To ensure type safety of the entry point, the bootloader provides this macro.
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    cbos::init(boot_info);

    #[cfg(test)]
    test_main();

    kprintln!("The kernel is alive!");
    run();

    cbos::hal::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eprintln!("{}", info);
    cbos::hal::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test_panic_handler(info)
}
