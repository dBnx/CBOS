#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader::{entry_point, BootInfo};
use cbos::tests::{exit_qemu, serial_print, serial_println, QemuExitCode};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    cbos::init(boot_info);
    init_test_idt();
    kill_kernel_stack();
    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn kill_kernel_stack() {
    kill_kernel_stack();
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(handler_double_fault)
                .set_stack_index(cbos::gdt::IST_INDEX::DOUBLE_FAULT as u16);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cbos::tests::test_panic_handler(info);
}

extern "x86-interrupt" fn handler_double_fault(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
