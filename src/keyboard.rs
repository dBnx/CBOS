use crate::prelude::*;
use alloc::string::String;
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use x86_64::instructions::interrupts::without_interrupts;

/// The maximal amount of unhandled keys. If more events happen, they are dropped.
/// To pop a key of the buffer use `pop_key`.
pub const KEY_BUFFER_SIZE: usize = 32;

lazy_static! {
    pub(crate) static ref KEYBOARD: Mutex<VecDeque<DecodedKey>> =
        Mutex::new(VecDeque::with_capacity(KEY_BUFFER_SIZE));
}

/// Returns the next key
#[must_use]
pub fn pop_key() -> Option<DecodedKey> {
    without_interrupts(|| KEYBOARD.lock().pop_front())
}

/// Returns the amount of currently queued keys
#[must_use]
pub fn len() -> usize {
    without_interrupts(|| KEYBOARD.lock().len())
}

/// Returns a string with all inputs until a Enter key is pressed or the maximal
/// amount of characters, given by `max_size`, is reached.
#[must_use]
pub fn get_new_line(max_size: usize) -> String {
    use pc_keyboard::DecodedKey::{RawKey, Unicode};
    let mut current_command = String::with_capacity(10);
    loop {
        while let Some(key) = pop_key() {
            match key {
                Unicode('\n') => {
                    return current_command;
                }
                Unicode(character) => {
                    print!("{}", character);
                    current_command.push(character);
                }
                RawKey(k) => {
                    kprintln!("{:?}", k);
                } //RawKey(_) => {}
            }
            if current_command.len() > max_size {
                return current_command;
            }
        }
        crate::hal::hlt();
    }
}
