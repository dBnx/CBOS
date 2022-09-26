use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

/// Remap interrupt vectors from 0-7 to 32-39 for PIC 1, as it would overlap with CPU exceptions
pub const PIC_1_OFFSET: u8 = 32;
/// Remap interrupt vectors from 8-15 to 40-47 for PIC 1, as it would overlap with CPU exceptions
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// Enables interrupts and maps 8259 interrupts to a usable range (32..47)
pub fn init_pic() {
    println!("Enabling interrupt handling ...");
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn setup_interupt_handlers(idt: &mut InterruptDescriptorTable) {
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(handler_timer_interrupt);
    idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(handler_keyboard_interrupt);
}

#[inline]
fn end_of_interrupt(what: InterruptIndex) {
    let mut pics = PICS.lock();
    unsafe { pics.notify_end_of_interrupt(what.as_u8()) }
}

extern "x86-interrupt" fn handler_timer_interrupt(_stack_frame: InterruptStackFrame) {
    use crate::task::timer;
    timer::tick();
    end_of_interrupt(InterruptIndex::Timer);
}

extern "x86-interrupt" fn handler_keyboard_interrupt(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
    use x86_64::instructions::port::Port;
    lazy_static! {
        pub static ref KB: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    crate::task::keyboard::add_scancode(scancode);

    end_of_interrupt(InterruptIndex::Keyboard);
}
