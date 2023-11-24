extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use safe_vex::controller::joystick::JoyStick;
use crate::config::Config;

/// The current state of the voltage for each of the drive-train's motors
#[derive(Debug, Clone, Copy)]
pub struct DriveState {
    /// Top-let motor of the robot
    pub l1: i32,
    /// Bottom-let motor of the robot
    pub l2: i32,
    /// Top-right motor of the robot
    pub r1: i32,
    /// Bottom-right motor of the robot
    pub r2: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum DriveArg {
    /// Forwards by a multiplier
    Forward(u8),
    /// Forwards by a multiplier
    Backward(u8),
    /// Turn left
    TLeft(u8),
    /// Turn right
    TRight(u8),
    /// Strafe left
    SLeft,
    /// Strafe right
    SRight,
}

impl DriveArg {
    #[inline]
    pub fn new(left_stick: JoyStick, l2: bool, r2: bool) -> Box<[DriveArg]> {
        use DriveArg as D;

        let left_stick = left_stick.step(Config::CONTROLLER_STICK_MIN);
        let mut args = Vec::new();
        let mut movement_arg = false;

        let stick = if left_stick.x_larger() { left_stick.x } else { left_stick.y };
        if stick != 0 {args.push({
            movement_arg = true; // you can't do two different movements at the same time

            match (left_stick.x_larger(), stick.is_positive()) {
                (true, true) => D::TRight(stick as u8), // Turn right
                (true, false) => D::TLeft(stick.unsigned_abs()), // Turn left
                (false, true) => D::Forward(stick as u8), // Move forwards
                (false, false) => D::Backward(stick.unsigned_abs()), // Move backwards
            }
        })};

        if !movement_arg {
            if l2 {
                args.push(D::SLeft);
            } else if r2 {
                args.push(D::SRight);
            }
        }

        args.into_boxed_slice()
    }
}

impl DriveState {
    #[inline]
    pub fn new(args: &[DriveArg]) -> Self {
        let mut state = Self {
            l1: 0,
            l2: 0,
            r1: 0,
            r2: 0,
        };

        use DriveArg as D;
        for arg in args {match arg {
            D::Forward(x) => {
                let voltage = calculate_voltage(*x, Config::DRIVE_FORWARD_SPEED);
                state.l1 = voltage;
                state.l2 = voltage;
                state.r1 = voltage;
                state.r2 = voltage;
            },

            D::Backward(x) => {
                let voltage = calculate_voltage(*x, Config::DRIVE_BACKWARD_SPEED);
                state.l1 = -voltage;
                state.l2 = -voltage;
                state.r1 = -voltage;
                state.r2 = -voltage;
            },

            D::TLeft(x) => {
                let voltage = calculate_voltage(*x, Config::DRIVE_TURN_SPEED);
                state.l1 = -voltage;
                state.l2 = -voltage;
                state.r1 = voltage;
                state.r2 = voltage;
            },

            D::TRight(x) => {
                let voltage = calculate_voltage(*x, Config::DRIVE_TURN_SPEED);
                state.l1 = voltage;
                state.l2 = voltage;
                state.r1 = -voltage;
                state.r2 = -voltage;
            },

            D::SLeft => {
                let voltage = calculate_voltage(i8::MAX as u8, Config::DRIVE_STRAFE_SPEED);
                state.l1 = voltage;
                state.l2 = -voltage;
                state.r1 = -voltage;
                state.r2 = voltage;
            },

            D::SRight => {
                let voltage = calculate_voltage(i8::MAX as u8, Config::DRIVE_STRAFE_SPEED);
                state.l1 = -voltage;
                state.l2 = voltage;
                state.r1 = voltage;
                state.r2 = -voltage;
            },
        }}

        state
    }
}

/// Calculates the voltage to use for each motor
#[inline]
fn calculate_voltage(stick: u8, percent: u8) -> i32 {
    (i32::MAX as f64
        * (percent.clamp(0, 100) as f64 / 100f64) // to normalize the voltage to the percentage (and prevent overflow)
        * (stick as f64 / i8::MAX as f64) // to normalize the voltage to the stick percentage
    ).clamp(i32::MIN as f64, i32::MAX as f64) as i32
}