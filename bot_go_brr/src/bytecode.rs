//! functions for dealing with byte-code

use core::fmt::Display;
use alloc::vec::Vec;
use safe_vex::{maybe::Maybe, motor::Motor};
use crate::drive_train::DriveTrain;

/// A single bytecode instruction for the robot
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteCode {
    /// Progresses a specified amount of tick-cycles
    Cycle(u32),

    /// Updates the voltage of the left motors of the drive-train
    LeftDrive {
        /// The voltage to apply to the motor
        voltage: i32,
    },

    /// Updates the voltage of the right motors of the drive-train
    RightDrive {
        /// The voltage to apply to the motor
        voltage: i32,
    },

    /// Updates the voltage of the conveyor-belt motor of the drive-train
    Belt {
        /// The voltage to apply to the motor
        voltage: i32,
    },
}

impl Display for ByteCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use ByteCode as B;
        match self {
            B::Cycle(x) => write!(f, "c +{x:?};"),

            B::LeftDrive { voltage } if *voltage < 0 => write!(f, "ld {voltage:?};"),
            B::LeftDrive { voltage } => write!(f, "ld +{voltage:?};"),

            B::RightDrive { voltage } if *voltage < 0 => write!(f, "rd {voltage:?};"),
            B::RightDrive { voltage } => write!(f, "rd +{voltage:?};"),

            B::Belt { voltage } if *voltage < 0 => write!(f, "b {voltage:?};"),
            B::Belt { voltage } => write!(f, "b +{voltage:?};"),
        }
    }
}

/// Executes bytecode and pops it off the bytecode vec for each tick-cycle
#[inline]
pub fn execute(bytecode: &mut Vec<ByteCode>, drive_train: &mut DriveTrain, belt: &mut Maybe<Motor>) {
    let mut current_inst = bytecode.pop();

    while let Some(inst) = current_inst {
        match inst {
            // skip a cycle without consuming the cycle inst
            ByteCode::Cycle(x) if x != 0 => {
                bytecode.push(ByteCode::Cycle(x-1));
                return
            },

            // skip a cycle while consuming the cycle inst
            // (must be zero)
            ByteCode::Cycle(_) => return,

            // updates the drive-train motors
            ByteCode::LeftDrive { voltage } => drive_train.drive_left(voltage),
            ByteCode::RightDrive { voltage } => drive_train.drive_right(voltage),
            
            // update the conveyor-belt motor
            ByteCode::Belt { voltage } => { belt.get().map(|motor| motor.move_voltage(voltage)); },
        }
        
        // update to next instruction
        current_inst = bytecode.pop();
    }
}

/// generates a vector of bytecode instructions from an ascii representation
#[macro_export]
macro_rules! ascii_bytecode {
    // user-facing api for the macro
    ($($inst:ident $prefix:tt $val:tt;)*) => {
        [
            $($crate::ascii_bytecode!(@internal $inst $prefix $val)),*
        ]
    };

    // instructions

    // Cycle
    (@internal c $prefix:tt $x:literal) => {
        $crate::bytecode::ByteCode::Cycle($x)
    };
    // LeftDrive
    (@internal ld $prefix:tt $x:literal) => {
        $crate::bytecode::ByteCode::LeftDrive { voltage: 0 $prefix $x }
    };
    // RightDrive
    (@internal rd $prefix:tt $x:literal) => {
        $crate::bytecode::ByteCode::RightDrive { voltage: 0 $prefix $x }
    };
    // Belt
    (@internal b $prefix:tt $x:literal) => {
        $crate::bytecode::ByteCode::Belt { voltage: 0 $prefix $x }
    };
}
