#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![allow(clippy::needless_return, clippy::multiple_crate_versions)]
#![warn(dead_code)]

pub mod action;
pub mod control_scheme;
pub mod input;
pub mod input_type;
pub mod multi_input;
pub mod multi_scheme;

pub mod prelude {
    pub use crate::action::*;
    pub use crate::control_scheme::*;
    pub use crate::input::*;
    pub use crate::make_controls;
}

pub mod multiplayer {
    pub use crate::action::*;
    pub use crate::control_scheme::*;
    pub use crate::input::*;
    pub use crate::make_multi_input;
    pub use crate::multi_input::*;
    pub use crate::multi_scheme::*;
}

/// Rudimentary helper function to get the scan code for a key.
/// This will hopefully be rendered obsolete in Bevy 0.13 with
/// [this](https://github.com/bevyengine/bevy/pull/10702) PR.
/// ```rust
/// use action_maps::get_scan_code;
/// let qwerty_w_scan_code = get_scan_code("W");
/// ```
pub fn get_scan_code(key: &str) -> u32 {
    match std::env::consts::OS {
        "macos" => match key {
            "," => 0x2B,
            "." => 0x2F,
            "Esc" => 0x35,
            "1" => 0x12,
            "2" => 0x13,
            "3" => 0x14,
            "4" => 0x15,
            "5" => 0x17,
            "6" => 0x16,
            "7" => 0x1A,
            "8" => 0x1C,
            "9" => 0x19,
            "0" => 0x1D,
            "A" => 0x00,
            "S" => 0x01,
            "D" => 0x02,
            "F" => 0x03,
            "H" => 0x04,
            "G" => 0x05,
            "Z" => 0x06,
            "X" => 0x07,
            "C" => 0x08,
            "V" => 0x09,
            "B" => 0x0B,
            "Q" => 0x0C,
            "W" => 0x0D,
            "E" => 0x0E,
            "R" => 0x0F,
            "Y" => 0x10,
            "T" => 0x11,
            "Equal" => 0x18,
            "Minus" => 0x1B,
            "]" => 0x1E,
            "O" => 0x1F,
            "U" => 0x20,
            "[" => 0x21,
            "I" => 0x22,
            "P" => 0x23,
            "Enter" => 0x24,
            "L" => 0x25,
            "J" => 0x26,
            "Quote" => 0x27,
            "K" => 0x28,
            "Semicolon" => 0x29,
            "Backslash" => 0x2A,
            "Comma" => 0x2B,
            "Slash" => 0x2C,
            "N" => 0x2D,
            "M" => 0x2E,
            "Period" => 0x2F,
            "Tab" => 0x30,
            "Space" => 0x31,
            "Backspace" => 0x33,
            "F1" => 0x7A,
            "F2" => 0x78,
            "F4" => 0x76,
            "F5" => 0x60,
            "F6" => 0x61,
            "F7" => 0x62,
            "F3" => 0x63,
            "F8" => 0x64,
            "F9" => 0x65,
            "F11" => 0x67,
            "F12" => 0x6F,
            "Insert" => 0x72,
            "Home" => 0x73,
            "PageUp" => 0x74,
            "Delete" => 0x75,
            "End" => 0x77,
            "PageDown" => 0x79,
            "Left" => 0x7B,
            "Right" => 0x7C,
            "Down" => 0x7D,
            "Up" => 0x7E,
            _ => unreachable!("Unknown key: {}", key),
        },
        "windows" => match key {
            "Esc" => 0x01,
            "1" => 0x02,
            "2" => 0x03,
            "3" => 0x04,
            "4" => 0x05,
            "5" => 0x06,
            "6" => 0x07,
            "7" => 0x08,
            "8" => 0x09,
            "9" => 0x0A,
            "0" => 0x0B,
            "-" => 0x0C,
            "=" => 0x0D,
            "Backspace" => 0x0E,
            "Tab" => 0x0F,
            "Q" => 0x10,
            "W" => 0x11,
            "E" => 0x12,
            "R" => 0x13,
            "T" => 0x14,
            "Y" => 0x15,
            "U" => 0x16,
            "I" => 0x17,
            "O" => 0x18,
            "P" => 0x19,
            "[" => 0x1A,
            "]" => 0x1B,
            "Enter" => 0x1C,
            "Ctrl" => 0x1D,
            "A" => 0x1E,
            "S" => 0x1F,
            "D" => 0x20,
            "F" => 0x21,
            "G" => 0x22,
            "H" => 0x23,
            "J" => 0x24,
            "K" => 0x25,
            "L" => 0x26,
            ";" => 0x27,
            "'" => 0x28,
            "`" => 0x29,
            "LShift" => 0x2A,
            "\\" => 0x2B,
            "Z" => 0x2C,
            "X" => 0x2D,
            "C" => 0x2E,
            "V" => 0x2F,
            "B" => 0x30,
            "N" => 0x31,
            "M" => 0x32,
            "," => 0x33,
            "." => 0x34,
            "/" => 0x35,
            "RShift" => 0x36,
            "PtScr" => 0x37,
            "Alt" => 0x38,
            "Space" => 0x39,
            "CpsLk" => 0x3A,
            "F1" => 0x3B,
            "F2" => 0x3C,
            "F3" => 0x3D,
            "F4" => 0x3E,
            "F5" => 0x3F,
            "F6" => 0x40,
            "F7" => 0x41,
            "F8" => 0x42,
            "F9" => 0x43,
            "F10" => 0x44,
            "Num" => 0x45,
            "ScrlLk" => 0x46,
            "Home" => 0x47,
            "Pg" => 0x49,
            "Num-" => 0x4A,
            "Up" => 0xE048,
            "Down" => 0xE050,
            "Left" => 0xE04B,
            "Right" => 0xE04D,
            "NumpadUp" => 0x48,
            "NumpadDown" => 0x50,
            "NumpadLeft" => 0x4B,
            "NumpadRight" => 0x4D,
            "End" => 0xC8,
            "PgDown" => 0x51,
            "Ins" => 0x52,
            "Del" => 0x53,
            _ => unreachable!("Unknown key: {}", key),
        },
        _ => {
            eprintln!("Unsupported OS: {}", std::env::consts::OS);
            eprintln!("Controls on this platform will silently fail");
            0x00
        }
    }
}
