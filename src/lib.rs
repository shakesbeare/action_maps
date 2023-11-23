#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![allow(clippy::needless_return, clippy::multiple_crate_versions)]

#![warn(dead_code)]

pub mod input_type;
pub mod action;
pub mod keyboard;
pub mod control_scheme;
pub mod input;


pub mod prelude {
    pub use crate::action::*;
    pub use crate::control_scheme::*;
    pub use crate::keyboard::get_scan_code;
}
