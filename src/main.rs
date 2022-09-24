#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]
#![allow(dead_code)]
//#![deny(unsafe_code)]
//use cbos::prelude::*;
use cbos::*;
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(test)]
use cbos::tests::*;

fn sleep_for_some_time(iterations: usize) {
    for _ in 0..iterations {
        volatile::Volatile::new(0).read(); // preventoptimizations
    }
}

fn quick_test() {
    //println!("This is a string.");
    println!("This is a very long string, which never fits into a single line of the VGA buffer. I promise.");
    let mut i = 0;
    loop {
        i += 1;
        //println!("{}", i);
        //eprintln!("{}", i);
        kprintln!("{}", i);
        if i % 3 == 0 {
            println!("{}", i);
        }
        sleep_for_some_time(800_000);
        sleep_for_some_time(300_000);
    }
    //cbos::hal::hlt_loop();
}
fn run() {
    crate::set_status_line!(
        "<CBOS> [1][2][3]<4>[5][6]                                                  12:13"
    );

    cbos::shell::run();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    cbos::init();

    #[cfg(test)]
    test_main();

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
