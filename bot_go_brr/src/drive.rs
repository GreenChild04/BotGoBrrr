extern crate alloc;

use crate::niceif;
use vex_rt::motor::Motor;
use crate::config::Config;
use crate::button::ButtonArg;
use alloc::string::ToString;

#[derive(Debug, Clone, Copy)]
pub enum DriveArg {
    Forward(ButtonArg, bool),
    Backward(ButtonArg, bool),
    Left(ButtonArg, bool),
    Right(ButtonArg, bool),
    Stop(ButtonArg, bool),
    Stall(ButtonArg, bool),
}

impl DriveArg {
    pub fn execute(&self, drive: &mut Drive) {
        match self {
            DriveArg::Forward(_, precise) => drive.forwards(*precise),
            DriveArg::Backward(_, precise) => drive.backwards(*precise),
            DriveArg::Left(_, precise) => drive.left(*precise),
            DriveArg::Right(_, precise) => drive.right(*precise),
            DriveArg::Stop(_, _) => drive.stop(),
            DriveArg::Stall(_, _) => (),
        }
    }

    pub fn add(first: Self, second: Self) -> Self {
        match (first, second) {
            (x, DriveArg::Stop(_, _)) => x,
            (DriveArg::Stop(_, _), y) => y,
            (_, _) => DriveArg::Stall(ButtonArg::Null, false),
        }
    }

    pub fn to_strings(&self) -> (&str, &str, bool) {
        match self {
            DriveArg::Forward(x, precise) => ("Forward", x.to_string(), *precise),
            DriveArg::Backward(x, precise) => ("Backward", x.to_string(), *precise),
            DriveArg::Left(x, precise) => ("Left", x.to_string(), *precise),
            DriveArg::Right(x, precise) => ("Right", x.to_string(), *precise),
            DriveArg::Stop(x, _) => ("Stop", x.to_string(), false),
            DriveArg::Stall(x, _) => ("Stall", x.to_string(), false),
        }
    }

    pub fn log(&self, tick: &u32) {
        use crate::utils::Log::*;
        let (name, button, precise) = self.to_strings();
        Base(
            tick,
            "Drive Arg",
            &List(
                &Title(name), "",
                &List(
                    &Wrap("(", &Title(button), ")"), " Precise: ",
                    &String(precise.to_string()),
                ),
            )
        ).log();
    }

    pub fn get_button(&self) -> &ButtonArg {
        match self {
            DriveArg::Forward(x, _) => x,
            DriveArg::Backward(x, _) => x,
            DriveArg::Left(x, _) => x,
            DriveArg::Right(x, _) => x,
            DriveArg::Stop(x, _) => x,
            DriveArg::Stall(x, _) => x,
        }
    }

    pub const fn duplicate(&self) -> Self {
        match self {
            DriveArg::Forward(x, precise) => DriveArg::Forward(x.duplicate(), *precise),
            DriveArg::Backward(x, precise) => DriveArg::Backward(x.duplicate(), *precise),
            DriveArg::Left(x, precise) => DriveArg::Left(x.duplicate(), *precise),
            DriveArg::Right(x, precise) => DriveArg::Right(x.duplicate(), *precise),
            DriveArg::Stop(x, _) => DriveArg::Stop(x.duplicate(), false),
            DriveArg::Stall(x, _precise) => DriveArg::Stall(x.duplicate(), false),
        }
    }
}

pub struct Drive {
    // Top to bottom, Left to right
    motor1: Motor,
    motor2: Motor,
    motor3: Motor,
    motor4: Motor,
}

impl Drive {
    pub fn new() -> Drive {
        Drive {
            motor1: Drive::build_motor(1),
            motor2: Drive::build_motor(2),
            motor3: Drive::build_motor(3),
            motor4: Drive::build_motor(4),
        }
    }

    pub fn drive(&mut self, arg: DriveArg) {
        arg.execute(self);
        arg.get_button().execute();
    }

    pub fn forwards(&mut self, precise: bool) {
        self.map(|x, _| x.move_i8(Drive::cal_volt(niceif!(if precise, Config::PRECISE_FORWARD_SPEED, else Config::FORWARD_SPEED))).unwrap());
    }

    pub fn stop(&mut self) {
        self.map(|x, _| x.move_i8(0).unwrap());
    }

    pub fn backwards(&mut self, precise: bool) {
        self.map(|x, _| x.move_i8(Drive::cal_volt(-niceif!(if precise, Config::PRECISE_BACKWARD_SPEED, else Config::BACKWARD_SPEED))).unwrap())
    }

    pub fn left(&mut self, precise: bool) {
        let turnspeed: i8 = niceif!(if precise, Config::PRECISE_TURN_SPEED, else Config::TURN_SPEED);
        self.map(|x, i| {
            if i & 1 == 0 { // Right Motors
                x.move_i8(Drive::cal_volt(turnspeed)).unwrap();
            } else { // Left Motors
                x.move_i8(Drive::cal_volt(-turnspeed)).unwrap();
            }
        });
    }

    pub fn right(&mut self, precise: bool) {
        let turnspeed: i8 = niceif!(if precise, Config::PRECISE_TURN_SPEED, else Config::TURN_SPEED);
        self.map(|x, i| {
            if i & 1 == 0 { // Right Motors
                x.move_i8(Drive::cal_volt(-turnspeed)).unwrap();
            } else { // Left Motors
                x.move_i8(Drive::cal_volt(turnspeed)).unwrap();
            }
        });
    }

    fn map<F>(&mut self, f: F)
    where
        F: Fn(&mut Motor, u8),
    {
        f(&mut self.motor1, 1);
        f(&mut self.motor2, 2);
        f(&mut self.motor3, 3);
        f(&mut self.motor4, 4);
    }

    fn cal_volt(speed: i8) -> i8 { (127i16 * speed as i16 / 100i16) as i8 } // Normalised speed from 1 to 100

    fn build_motor(id: u8) -> Motor {
        unsafe {
            Motor::new(
                Config::MOTORS.id_to_port(id),
                Config::GEAR_RATIO,
                Config::MOTORS.units,
                Config::MOTORS.id_to_reverse(id),
            )
        }.unwrap_or_else(|_|
            panic!("Error: Could not configure / generate motor id '{0}' at port '{1}'!", id, Config::MOTORS.id_to_port(id))
        )
    }
}