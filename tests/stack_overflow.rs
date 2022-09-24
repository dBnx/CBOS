#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use cbos::tests::{exit_qemu, serial_print, serial_println, QemuExitCode};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    cbos::init();
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
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[failure]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

extern "x86-interrupt" fn handler_double_fault(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
