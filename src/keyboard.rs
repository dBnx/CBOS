use crate::util::ps2_scancodes::*;
use spin::Mutex;

#[derive(Default)]
pub struct Keyboard {
    shift: bool,
    control: bool,
    alt: bool,
    numlock: bool,
}

lazy_static! {
    static ref KEYBOARD_STATE: Mutex<Keyboard> = Mutex::new(Keyboard::default());
}

impl Mutex<Keyboard> {
    pub fn new_event(event: KeyEvent) {
        if event.is_alt() {
            self.lock().alt = event.state.is_pressed();
        }
    }
}
