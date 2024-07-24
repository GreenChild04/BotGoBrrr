macro_rules! motor_config {
    ($(#[$meta:meta])* $motor:ident: $port:expr, $reverse:expr;) => {
        /// **(motor configuration)**
        ///
        $(#[$meta])*
        pub const $motor: $crate::config::MotorConfig = $crate::config::MotorConfig { port: $port, reverse: $reverse };
    };

    ($(#[$meta:meta])* $motor:ident: $port:expr, $reverse:expr; $($tail:tt)+) => {
        /// **(motor configuration)**
        ///
        $(#[$meta])*
        pub const $motor: $crate::config::MotorConfig = $crate::config::MotorConfig { port: $port, reverse: $reverse };
        motor_config! {
            $($tail)*
        }
    };
}

#[derive(Debug, Copy, Clone)]
pub struct MotorConfig {
    pub port: u8,
    pub reverse: bool,
}

pub mod drive {
    use safe_vex::vex_rt::motor::{EncoderUnits, Gearset};

    /// The gear ratio of the drive-train's motors
    pub const GEAR_RATIO: Gearset = Gearset::ThirtySixToOne;
    /// The unit used to mreasure the motor
    pub const UNIT: EncoderUnits = EncoderUnits::Degrees;

    motor_config! {
        /// Top-left drive-train motor
        L1: 12, false;
        /// Bottom-left drive-train motor
        L2: 20, false;
        /// Top-right drive-train motor
        R1: 1, true;
        /// Bottom-right drive-train motor
        R2: 9, true;

        /// Belt motor
        BELT: 21, false;
        /// Intake motor
        INTAKE: 10, false;
    }

    // /// the robot's forward speed out of `100`
    // pub const FORWARD_SPEED: u8 = 100;
    // /// the robot's backward speed out of `100`
    // pub const BACKWARD_SPEED: u8 = 90;
    // /// the robot's turning speed out of `100`
    // pub const TURN_SPEED: u8 = 60;

    /// The robot's conveyor belt voltage out of `12000`
    pub const BELT_VOLTAGE: i32 = 12000;
    /// The robot's intake motor's voltage out of `12000`
    pub const INTAKE_VOLTAGE: i32 = BELT_VOLTAGE;

    // /// The percentage of the normal drive speed for precise movement
    // pub const PRECISE_SPEED: u8 = 60;
}

/// The minimum amount of activation the controller has to have to be activated
pub const CONTROLLER_STICK_MIN: u8 = 10;

/// Daniel's magic number for the joysticks
pub const DMN: f64 = 1.07614027714168; // 12000 = x^{128} - 1

pub mod autonomous {
    use include_tt::include_tt;

    use crate::{ascii_bytecode, bytecode::ByteCode};

    /// The autonomous bytecode executed before a vex vrc match
    pub const MATCH_AUTO: [ByteCode; 9] = include_tt!(ascii_bytecode! { #include_tt!("src/autonomous/match_auto.brb") });

    /// The autonomous bytecode executed during a vex vrc skills round
    pub const FULL_AUTO: [ByteCode; 0] = include_tt!(ascii_bytecode! { #include_tt!("src/autonomous/full_auto.brb") });
}
