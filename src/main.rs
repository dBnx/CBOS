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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

fn run() {
    crate::set_status_line!(
        "################################################################################"
    );
    serial_println!("Test");
    use task::{
        executor::{self, Executor},
        Task,
    };

    let mut kb = task::keyboard::ScancodeStream::new();
    let mut executor = Executor::new();
    executor.set_global_spawner().unwrap();

    executor::spawn(Task::new(example_task()));
    executor::spawn(Task::new(async move { programs::run_statusline().await }));
    executor::spawn(Task::new(async move {
        programs::run_shell(&mut kb).await;
    }));
    executor.run();
    kprintln!("Reached end of run()");
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
