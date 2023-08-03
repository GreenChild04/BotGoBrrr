use crate::controller::StickState;
use crate::drive::DriveArg;
use crate::button::ButtonArg;
use crate::config::Config;

pub struct Smooth {
    modifier: StickState,
    action: StickState,
    ticks: u16,
}

impl Smooth {
    pub fn new() -> Self {
        Self {
            action: StickState::None,
            modifier: StickState::None,
            ticks: 0,
        }
    }

    pub fn reset(&mut self, state: StickState) {
        self.modifier = self.action;
        self.action = state;
        self.ticks = 0;
    }

    pub fn gen_arg(&mut self, state: StickState, button: ButtonArg) -> DriveArg {
        if state == self.action { self.ticks += 1 }
        else { self.reset(state) }
        self.execute()(button, false)
    }

    pub fn execute(&self) -> fn(ButtonArg, bool) -> DriveArg {
        use StickState::*;
        match self.action {
            None => DriveArg::Stop,
            East => DriveArg::Right,
            West => DriveArg::Left,
            North => match self.modifier {
                NorthEast(_) if self.ticks < Config::TICKS_FOR_45 as u16 => DriveArg::Left,
                NorthWest(_) if self.ticks < Config::TICKS_FOR_45 as u16 => DriveArg::Right,
                _ => DriveArg::Forward,
            },
            South => match self.modifier {
                SouthEast(_) if self.ticks < Config::TICKS_FOR_45 as u16 => DriveArg::Right,
                SouthWest(_) if self.ticks < Config::TICKS_FOR_45 as u16 => DriveArg::Left,
                _ => DriveArg::Backward,
            },
            NorthEast(n) => match self.modifier {
                North if self.ticks < Config::TICKS_FOR_45 as u16 => DriveArg::Right,
                North => DriveArg::Forward,
                // so it only does the 45 degree thing when changing from north
                _ if n => DriveArg::Forward,
                _ => DriveArg::Right,
            },
            NorthWest(n) => match self.modifier {
                North if self.ticks < Config::TICKS_FOR_45 as u16 => DriveArg::Left,
                North => DriveArg::Forward,
                // so it only does the 45 degree thing when changing from north
                _ if n => DriveArg::Forward,
                _ => DriveArg::Left,
            },
            SouthEast(n) => match self.modifier {
                South if self.ticks < Config::TICKS_FOR_45 as u16 => DriveArg::Left,
                South => DriveArg::Backward,
                // so it only does the 45 degree thing when changing from north
                _ if n => DriveArg::Backward,
                _ => DriveArg::Left,
            },
            SouthWest(n) => match self.modifier {
                South if self.ticks < Config::TICKS_FOR_45 as u16 => DriveArg::Right,
                South => DriveArg::Backward,
                // so it only does the 45 degree thing when changing from north
                _ if n => DriveArg::Backward,
                _ => DriveArg::Right,
            },
        }
    }
}