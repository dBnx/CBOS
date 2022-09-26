#![allow(dead_code, unused_variables, non_snake_case)]
use core::fmt;

use futures_util::StreamExt;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::task::timer::TickStream;

lazy_static! {
    static ref STATUS_LINE: Mutex<StatusLine<12>> = Mutex::new(StatusLine::new("<CBAS>"));
}

/// Periodically updates the status line
pub async fn run() {
    let mut ticks = TickStream::new(16);
    while let Some(()) = ticks.next().await {
        let mut status_line = STATUS_LINE.lock();
        status_line.tick();
        crate::hal::hlt();
    }
}

/// T corresponds to the amount of vts'es
struct StatusLine<const T: usize> {
    name: &'static str,
    vts: [VtsState; T],
    clock: Clock,
    ticks: usize,
}

impl<const T: usize> StatusLine<T> {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            vts: [VtsState::default(); T],
            clock: Clock::default(),
            ticks: 0,
        }
    }

    fn update(&self) {
        set_status_line!("{}", self);
    }

    #[inline]
    fn set_name(&mut self, name: &'static str) {
        //debug_assert_eq!(name.len(), 5);
        self.name = name;
    }

    fn tick(&mut self) {
        self.clock.tick();
        self.update();
    }

    /// Returns `None` if it could not be updated due to the length of name.
    pub fn update_vts_state(&mut self, id: usize, state: VtsState) -> Option<()> {
        if id < self.vts.len() || self.vts[id] != state {
            return None;
        }
        self.vts[id] = state;
        self.update();
        Some(())
    }

    pub fn get_clock(&self) -> Clock {
        self.clock
    }
}

impl<const T: usize> fmt::Display for StatusLine<T> {
    /// Always returns a string with length 80, which corresponds to the VGA buffer length
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Used to ensure a constant width
        let mut len = self.name.len() + 8 /* Clock */;
        write!(f, "{}", self.name)?;
        for state in &self.vts {
            write!(f, "{}", state)?;
            len += 5;
        }
        for _ in len..(crate::vga::BUFFER_COLS as usize) {
            write!(f, " ")?;
        }
        write!(f, "{}", self.clock)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Clock {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Clock {
    fn tick(&mut self) {
        self.seconds += 1;
        if self.seconds >= 60 {
            self.seconds = 0;
            self.minutes += 1;
        }
        if self.minutes >= 60 {
            self.minutes = 0;
            self.hours += 1;
        }
        if self.hours > 99 {
            self.hours = 0;
        }
    }
}

/// Procudes the string "HH:mm:ss", with a length of 8
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:>2}:{:0>2}:{:0>2}",
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
            VtsState::Active => " [X] ",
            VtsState::Inactive => " [-] ",
            VtsState::Alert => " [!] ",
            VtsState::NotUsed => " [ ] ",
        };
        debug_assert_eq!(s.len(), 5);
        write!(f, "{}", s)
    }
}
