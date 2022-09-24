//use crate::prelude::*;
use crate::{eprintln, gdt, println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[cfg(test)]
mod tests;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // UNSAFE: The stack is setup and the index exists in the Task State Segment ()
        unsafe {
            idt.double_fault
                .set_handler_fn(handler_double_fault)
                .set_stack_index(gdt::IST_INDEX::DOUBLE_FAULT as u16);
        }
        idt.breakpoint.set_handler_fn(handler_breakpoint);
        crate::interrupts::setup_interupt_handlers(&mut idt);
        idt
    };
}

/// Must be called after initializing the GDT, as it setups the stack (indices) for
/// the exceptions!
pub fn init_idt() {
    println!("Enabling exception handling ...");
    IDT.load();
}

/// Reference: [AMD64 manual](https://www.amd.com/system/files/TechDocs/24593.pdf) Section 8.2.9
extern "x86-interrupt" fn handler_double_fault(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:?}", stack_frame);
}

extern "x86-interrupt" fn handler_breakpoint(stack_frame: InterruptStackFrame) {
    eprintln!("EXCEPTION: BREAKPOINT\n{:?}", stack_frame);
}
