#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

impl KeyState {
    fn is_pressed(&self) -> bool {
        match self {
            KeyState::Pressed => true,
            KeyState::Released => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpecialKey {
    Escape,
    Backspace,
    Enter,
    Tab,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    CapsLock,
    NumberLock,
    ScrollLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

impl SpecialKey {
    pub fn is_shift(&self) -> bool {
        use SpecialKey::*;
        match self {
            ShiftLeft | ShiftRight => true,
            _ => false,
        }
    }
    pub fn is_control(&self) -> bool {
        use SpecialKey::*;
        match self {
            ControlLeft | ControlRight => true,
            _ => false,
        }
    }
    pub fn is_alt(&self) -> bool {
        use SpecialKey::*;
        match self {
            AltLeft | AltRight => true,
            _ => false,
        }
    }
    pub fn is_function_key(&self) -> bool {
        use SpecialKey::*;
        match self {
            F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 | F9 | F10 | F11 | F12 => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
    Char(char),
    Keypad(char),
    Special(SpecialKey),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyEvent {
    pub key: Key,
    pub state: KeyState,
}

pub fn parse_ibm_xt(scancode: u8) -> Option<KeyEvent> {
    use Key::*;
    use SpecialKey::*;

    let state = match scancode & 0x80 {
        0 => KeyState::Pressed,
        _ => KeyState::Released,
    };

    //let scancode = !(!scancode & 0x80);
    let key: Key = match scancode {
        0x00 => None,
        0x01 => Some(Special(Escape)),
        0x02 => Some(Char('1')),
        0x03 => Some(Char('2')),
        0x04 => Some(Char('3')),
        0x05 => Some(Char('4')),
        0x06 => Some(Char('5')),
        0x07 => Some(Char('6')),
        0x08 => Some(Char('7')),
        0x09 => Some(Char('8')),
        0x0a => Some(Char('9')),
        0x0b => Some(Char('0')),
        0x0c => Some(Char('-')),
        0x0d => Some(Char('=')),
        0x0e => Some(Special(Backspace)),
        0x0f => Some(Special(Tab)),

        0x10 => Some(Char('Q')),
        0x11 => Some(Char('W')),
        0x12 => Some(Char('E')),
        0x13 => Some(Char('R')),
        0x14 => Some(Char('T')),
        0x15 => Some(Char('Y')),
        0x16 => Some(Char('U')),
        0x17 => Some(Char('I')),
        0x18 => Some(Char('O')),
        0x19 => Some(Char('P')),
        0x1a => Some(Char('[')),
        0x1b => Some(Char(']')),
        0x1c => Some(Special(Enter)),
        0x1d => Some(Special(ControlLeft)),
        0x1e => Some(Char('A')),
        0x1f => Some(Char('S')),

        0x20 => Some(Char('D')),
        0x21 => Some(Char('F')),
        0x22 => Some(Char('G')),
        0x23 => Some(Char('H')),
        0x24 => Some(Char('J')),
        0x25 => Some(Char('K')),
        0x26 => Some(Char('L')),
        0x27 => Some(Char(';')),
        0x28 => Some(Char('\'')),
        0x29 => Some(Char('`')),
        0x2a => Some(Special(ShiftLeft)),
        0x2b => Some(Char('\\')),
        0x2c => Some(Char('Z')),
        0x2d => Some(Char('X')),
        0x2e => Some(Char('C')),
        0x2f => Some(Char('V')),

        0x30 => Some(Char('B')),
        0x31 => Some(Char('N')),
        0x32 => Some(Char('M')),
        0x33 => Some(Char(',')),
        0x34 => Some(Char('.')),
        0x35 => Some(Char('/')),
        0x36 => Some(Special(ShiftRight)),
        0x37 => Some(Keypad('*')),
        0x38 => Some(Special(AltLeft)),
        0x39 => Some(Char(' ')),
        0x3a => Some(Special(CapsLock)),
        0x3b => Some(Special(F1)),
        0x3c => Some(Special(F2)),
        0x3d => Some(Special(F3)),
        0x3e => Some(Special(F4)),
        0x3f => Some(Special(F5)),

        0x40 => Some(Special(F6)),
        0x41 => Some(Special(F7)),
        0x42 => Some(Special(F8)),
        0x43 => Some(Special(F9)),
        0x44 => Some(Special(F10)),
        0x45 => Some(Special(NumberLock)),
        0x46 => Some(Special(ScrollLock)),
        0x47 => Some(Keypad('7')),
        0x48 => Some(Keypad('8')),
        0x49 => Some(Keypad('9')),
        0x4a => Some(Keypad('-')),
        0x4b => Some(Keypad('4')),
        0x4c => Some(Keypad('5')),
        0x4d => Some(Keypad('6')),
        0x4e => Some(Keypad('+')),
        0x4f => Some(Keypad('1')),
        0x50 => Some(Keypad('2')),
        0x51 => Some(Keypad('3')),
        0x52 => Some(Keypad('0')),
        0x53 => Some(Keypad('.')),

        0x57 => Some(Special(F11)),
        0x58 => Some(Special(F12)),
        _ => None,
    }?;

    Some(KeyEvent { key, state })
}
