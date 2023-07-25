use super::drive::DriveArg;
use super::button::ButtonArg;

#[macro_export]
macro_rules! gen_algor {
    ($([$arg:expr, $count:expr]),* $(,)?) => {{
        const RESULT: [Option::<DriveArg>; $($count+)* 0usize] = {
            let mut result: [Option::<DriveArg>; $($count+)* 0usize] = [Option::<DriveArg>::None; $($count+)* 0usize];
            let mut idx: usize = 0;
            $({
                let mut i: usize = 0;
                while i < $count {
                    result[idx] = Some($arg);
                    idx += 1;
                    i += 1;
                }
            };)*
            result
        };
        Algor(&RESULT)
    }};
    // ($($((keyword:ident))? $arg:ident$(($butt:ident))? $count:expr);* $(;)?) => {
    //     if 
    // };
    ( $(precise $arg:ident($butt:ident) $count:expr);* $(;)?) => {
        gen_algor!($([DriveArg::$arg($butt, true), $count],)*)
    };
    (hidden $($arg:ident($butt:ident) $count:expr);* $(;)?) => {
        gen_algor!($([DriveArg::$arg($butt, false), $count],)*)
    };
    (hidden $($arg:ident $count:expr);* $(;)?) => {
        gen_algor!($([DriveArg::$arg(ButtonArg::Null, false), $count],)*)
    };
    (hidden $(precise $arg:ident $count:expr);* $(;)?) => {
        gen_algor!($([DriveArg::$arg(ButtonArg::Null, true), $count],)*)
    };
}

pub struct Algor(&'static [Option<DriveArg>]);
impl Algor {
    pub fn get(&self, tick: &u128) -> Option<DriveArg> {
        let tick = *tick as usize;
        if self.0.len() <= tick { None }
        else { Some(self.0[tick].as_ref().unwrap().duplicate()) }
    }
}

// Algorithms
impl Algor {
    pub const AUTONOMOUS: Algor = gen_algor! {
        hidden precise Forward 30;
    };
}