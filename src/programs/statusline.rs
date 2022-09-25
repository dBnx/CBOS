#![allow(dead_code, unused_variables, non_snake_case)]
use core::fmt;

/// Periodically updates the status line
pub async fn run() {
    async {}.await;
}

/// T corresponds to the amount of ticks that equal to one second.
pub struct StatusLine<const T: usize> {
    name: &'static str,
    vts: [VtsState; 12],
    clock: Clock,
    ticks: usize,
}

impl<const T: usize> StatusLine<T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            vts: Default::default(),
            clock: Clock::default(),
            ticks: 0,
        }
    }

    fn update_status_line(&self) {
        set_status_line!("{}", self);
    }

    #[inline]
    pub fn set_name(&mut self, name: &'static str) {
        //debug_assert_eq!(name.len(), 5);
        self.name = name;
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        if self.ticks >= T {
            self.ticks = 0;
            self.update_status_line();
        }
    }

    /// Returns `None` if it could not be updated due to the length of name.
    pub fn update_vts_state(&mut self, id: usize, state: VtsState) -> Option<()> {
        if id < self.vts.len() || self.vts[id] != state {
            return None;
        }
        self.vts[id] = state;
        self.update_status_line();
        Some(())
    }
}

impl<const T: usize> fmt::Display for StatusLine<T> {
    /// Always returns a string with length 80, which corresponds to the VGA buffer length
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        for state in &self.vts {
            write!(f, "{}", state)?;
        }
        write!(f, "{}", self.clock)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
struct Clock {
    hours: u8,
    minutes: u8,
    seconds: u8,
}

/// Procudes the string "HH:mm:ss", with a length of 8
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>2}:{:>2}:{:>2}",
            self.hours, self.minutes, self.seconds
        )
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VtsState {
    Active,
    Inactive,
    Alert,
    #[default]
    NotUsed,
}

impl fmt::Display for VtsState {
    /// Always returns a string with length 3
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            VtsState::Active => "[]",
            _ => "   ",
        };
        // FIXME
        write!(f, "{:?}", self)
    }
}
