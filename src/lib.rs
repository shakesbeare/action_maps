#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![allow(clippy::needless_return, clippy::multiple_crate_versions)]
#![warn(dead_code)]

mod action;
mod action_input;
mod control_scheme;
mod multi_input;
mod multi_scheme;
mod plugin;
mod universal_input;

pub mod prelude {
    pub use crate::actions::Action;
    pub use crate::actions::ActionInput;
    pub use crate::controls::ControlScheme;
    pub use crate::make_controls;
    pub use crate::plugin::ActionMapPlugin;
    pub use crate::plugin::ActionMapSet;
}

pub mod multiplayer_prelude {
    pub use crate::make_multi_input;
    pub use crate::multi_input::MultiInput;
    pub use crate::multi_scheme::MultiScheme;
    pub use crate::plugin::MultiActionMapPlugin;
    pub use crate::prelude::*;
}

pub mod actions {
    pub use crate::action::Action;
    pub use crate::action_input::*;
    pub use crate::multi_input::*;
}

pub mod controls {
    pub use crate::control_scheme::*;
    pub use crate::multi_scheme::*;
}

pub mod input {
    pub use crate::universal_input::*;
}

