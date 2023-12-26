use bevy::ecs::change_detection::DetectChangesMut;
use bevy::input::keyboard::NativeKeyCode;
use bevy::ecs::event::Event;
use bevy::ecs::event::EventReader;
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::input::gamepad::GamepadButton;
use bevy::input::gamepad::GamepadButtonChangedEvent;
use bevy::input::gamepad::GamepadButtonInput;
use bevy::input::gamepad::GamepadButtonType;
use bevy::input::gamepad::GamepadSettings;
use bevy::input::keyboard::KeyCode;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButton;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;

use crate::actions::MultiInput;
use crate::controls::MultiScheme;
use crate::prelude::ActionInput;
use crate::prelude::ControlScheme;

pub fn universal_input_system(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut button_input_events: EventWriter<GamepadButtonInput>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut action_input: ResMut<ActionInput>,
    control_scheme: Res<ControlScheme>,
    settings: Res<GamepadSettings>,
) {
    action_input.bypass_change_detection().clear();
    let keyboard_events: Vec<&KeyboardInput> = keyboard_events.read().collect();
    let gamepad_events: Vec<&GamepadButtonChangedEvent> =
        gamepad_events.read().collect();
    let mouse_button_events: Vec<&MouseButtonInput> =
        mouse_button_events.read().collect();
    update_inputs(
        &keyboard_events,
        &gamepad_events,
        &mut button_input_events,
        &mouse_button_events,
        &mut action_input,
        &control_scheme,
        &settings,
    );
}

pub fn multi_universal_input_system(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut button_input_writer: EventWriter<GamepadButtonInput>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut multi_input: ResMut<MultiInput>,
    multi_scheme: Res<MultiScheme>,
    settings: Res<GamepadSettings>,
) {
    multi_input.bypass_change_detection();
    let keyboard_events: Vec<&KeyboardInput> = keyboard_events.read().collect();
    let gamepad_events: Vec<&GamepadButtonChangedEvent> =
        gamepad_events.read().collect();
    let mouse_button_events: Vec<&MouseButtonInput> =
        mouse_button_events.read().collect();

    for i in 0..multi_input.keys().len() {
        let action_input = multi_input.get_mut(i).unwrap();
        let control_scheme = multi_scheme.get(i).unwrap();
        action_input.clear();
        update_inputs(
            &keyboard_events,
            &gamepad_events,
            &mut button_input_writer,
            &mouse_button_events,
            action_input,
            control_scheme,
            &settings,
        );
    }
}

fn update_inputs(
    keyboard_events: &Vec<&KeyboardInput>,
    gamepad_events: &Vec<&GamepadButtonChangedEvent>,
    button_input_events: &mut EventWriter<GamepadButtonInput>,
    mouse_button_events: &Vec<&MouseButtonInput>,
    action_input: &mut ActionInput,
    control_scheme: &ControlScheme,
    settings: &Res<GamepadSettings>,
) {
    for event in keyboard_events {
        let KeyboardInput {
            key_code,
            state,
            ..
        } = event;

        let key: UniversalInput = (*key_code).into();
        if let Some(action) = control_scheme.get(key) {
            match state {
                ButtonState::Pressed => action_input.press(*action),
                ButtonState::Released => action_input.release(*action),
            }
        }
    }

    for event in gamepad_events {
        let button = GamepadButton::new(event.gamepad, event.button_type);
        let value = event.value;
        let button_settings = settings.get_button_settings(button);
        let input: UniversalInput = button.into();
        let Some(action) = control_scheme.get(input) else {
            return;
        };

        // if is released...
        if value <= button_settings.release_threshold() {
            if action_input.pressed(*action) {
                button_input_events.send(GamepadButtonInput {
                    button,
                    state: ButtonState::Released,
                });
            }
            action_input.release(*action);
        } else if value >= button_settings.press_threshold() {
            button_input_events.send(GamepadButtonInput {
                button,
                state: ButtonState::Pressed,
            });
            action_input.press(*action);
        }
    }

    for event in mouse_button_events {
        let button: UniversalInput = event.button.into();

        if let Some(action) = control_scheme.get(button) {
            match event.state {
                ButtonState::Pressed => action_input.press(*action),
                ButtonState::Released => action_input.release(*action),
            }
        }
    }
}

#[derive(Event)]
pub struct UniversalInputEvent(UniversalInput);

/// Keys represent the physical key
#[repr(u32)]
#[derive(Hash, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UniversalInput {
/// This variant is used when the key cannot be translated to any other variant.
    ///
    /// The native keycode is provided (if available) so you're able to more reliably match
    /// key-press and key-release events by hashing the [`KeyCode`]. It is also possible to use
    /// this for keybinds for non-standard keys, but such keybinds are tied to a given platform.
    Unidentified(NativeKeyCode),
    /// <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
    /// This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
    /// (hankaku/zenkaku/kanji) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
    /// located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labeled <kbd>#</kbd> on a UK (102) keyboard.
    Backslash,
    /// <kbd>[</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,</kbd> on a US keyboard.
    Comma,
    /// <kbd>0</kbd> on a US keyboard.
    Digit0,
    /// <kbd>1</kbd> on a US keyboard.
    Digit1,
    /// <kbd>2</kbd> on a US keyboard.
    Digit2,
    /// <kbd>3</kbd> on a US keyboard.
    Digit3,
    /// <kbd>4</kbd> on a US keyboard.
    Digit4,
    /// <kbd>5</kbd> on a US keyboard.
    Digit5,
    /// <kbd>6</kbd> on a US keyboard.
    Digit6,
    /// <kbd>7</kbd> on a US keyboard.
    Digit7,
    /// <kbd>8</kbd> on a US keyboard.
    Digit8,
    /// <kbd>9</kbd> on a US keyboard.
    Digit9,
    /// <kbd>=</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labeled <kbd>\\</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
    /// Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
    /// Russian keyboard.
    IntlYen,
    /// <kbd>a</kbd> on a US keyboard.
    /// Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    KeyA,
    /// <kbd>b</kbd> on a US keyboard.
    KeyB,
    /// <kbd>c</kbd> on a US keyboard.
    KeyC,
    /// <kbd>d</kbd> on a US keyboard.
    KeyD,
    /// <kbd>e</kbd> on a US keyboard.
    KeyE,
    /// <kbd>f</kbd> on a US keyboard.
    KeyF,
    /// <kbd>g</kbd> on a US keyboard.
    KeyG,
    /// <kbd>h</kbd> on a US keyboard.
    KeyH,
    /// <kbd>i</kbd> on a US keyboard.
    KeyI,
    /// <kbd>j</kbd> on a US keyboard.
    KeyJ,
    /// <kbd>k</kbd> on a US keyboard.
    KeyK,
    /// <kbd>l</kbd> on a US keyboard.
    KeyL,
    /// <kbd>m</kbd> on a US keyboard.
    KeyM,
    /// <kbd>n</kbd> on a US keyboard.
    KeyN,
    /// <kbd>o</kbd> on a US keyboard.
    KeyO,
    /// <kbd>p</kbd> on a US keyboard.
    KeyP,
    /// <kbd>q</kbd> on a US keyboard.
    /// Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    KeyQ,
    /// <kbd>r</kbd> on a US keyboard.
    KeyR,
    /// <kbd>s</kbd> on a US keyboard.
    KeyS,
    /// <kbd>t</kbd> on a US keyboard.
    KeyT,
    /// <kbd>u</kbd> on a US keyboard.
    KeyU,
    /// <kbd>v</kbd> on a US keyboard.
    KeyV,
    /// <kbd>w</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    KeyW,
    /// <kbd>x</kbd> on a US keyboard.
    KeyX,
    /// <kbd>y</kbd> on a US keyboard.
    /// Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyY,
    /// <kbd>z</kbd> on a US keyboard.
    /// Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
    /// QWERTZ (e.g., German) keyboard.
    KeyZ,
    /// <kbd>-</kbd> on a US keyboard.
    Minus,
    /// <kbd>.</kbd> on a US keyboard.
    Period,
    /// <kbd>'</kbd> on a US keyboard.
    Quote,
    /// <kbd>;</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/</kbd> on a US keyboard.
    Slash,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
    /// This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
    AltRight,
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
    /// Labeled <kbd>Delete</kbd> on Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the right
    /// <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
    Enter,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperLeft,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
    SuperRight,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,
    /// <kbd> </kbd> (space)
    Space,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,
    /// Japanese: <kbd>変</kbd> (henkan)
    Convert,
    /// Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd> (katakana/hiragana/romaji)
    KanaMode,
    /// Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
    ///
    /// Japanese (Mac keyboard): <kbd>か</kbd> (kana)
    Lang1,
    /// Korean: Hanja <kbd>한</kbd> (hanja)
    ///
    /// Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <kbd>無変換</kbd> (muhenkan)
    NonConvert,
    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard is encoded as [`Backspace`].
    ///
    /// [`Backspace`]: Self::Backspace
    Delete,
    /// <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd>
    End,
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    Help,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
    PageUp,
    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// On the Mac, this is used for the numpad <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control
    Numpad4,
    /// <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
    /// or remote control
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
    /// or remote control
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
    /// <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
    /// numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].
    ///
    /// [`NumLock`]: Self::NumLock
    NumpadClear,
    /// <kbd>C</kbd> (Clear Entry)
    NumpadClearEntry,
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    NumpadComma,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>,</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    /// The Enter key on the numpad.
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found
    /// below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    NumpadHash,
    /// <kbd>M</kbd> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <kbd>M</kbd> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <kbd>M</kbd> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <kbd>M</kbd> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <kbd>M</kbd> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
    /// operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
    ///
    /// Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
    NumpadMultiply,
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <kbd>*</kbd> on a phone or remote control device.
    ///
    /// This key is typically found below the <kbd>7</kbd> key and to the left of
    /// the <kbd>0</kbd> key.
    ///
    /// Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
    /// numeric keypads.
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    ///
    /// This also the "back" button (triangle) on Android.
    BrowserBack,
    /// BrowserFavorites
    BrowserFavorites,
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    BrowserForward,
    /// The "home" button on Android.
    BrowserHome,
    /// BrowserRefresh
    BrowserRefresh,
    /// BrowserSearch
    BrowserSearch,
    /// BrowserStop
    BrowserStop,
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
    /// keyboards.
    Eject,
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    LaunchApp2,
    /// LaunchMail
    LaunchMail,
    /// MediaPlayPause
    MediaPlayPause,
    /// MediaSelect
    MediaSelect,
    /// MediaStop
    MediaStop,
    /// MediaTrackNext
    MediaTrackNext,
    /// MediaTrackPrevious
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards, replacing the
    /// <kbd>Eject</kbd> key.
    Power,
    /// Sleep
    Sleep,
    /// AudioVolumeDown
    AudioVolumeDown,
    /// AudioVolumeMute
    AudioVolumeMute,
    /// AudioVolumeUp
    AudioVolumeUp,
    /// WakeUp
    WakeUp,
    /// Legacy modifier key. Also called "Super" in certain places.
    Meta,
    /// Legacy modifier key.
    Hyper,
    /// Turbo
    Turbo,
    /// Abort
    Abort,
    /// Resume
    Resume,
    /// Suspend
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    Katakana,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F1,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F2,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F3,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F4,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F5,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F6,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F7,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F8,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F9,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F10,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F11,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F12,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F13,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F14,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F15,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F16,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F17,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F18,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F19,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F20,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F21,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F22,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F23,
    /// General-purpose function key.
    /// Usually found at the top of the keyboard.
    F24,
    /// General-purpose function key.
    F25,
    /// General-purpose function key.
    F26,
    /// General-purpose function key.
    F27,
    /// General-purpose function key.
    F28,
    /// General-purpose function key.
    F29,
    /// General-purpose function key.
    F30,
    /// General-purpose function key.
    F31,
    /// General-purpose function key.
    F32,
    /// General-purpose function key.
    F33,
    /// General-purpose function key.
    F34,
    /// General-purpose function key.
    F35,

    /// The bottom action button of the action pad (i.e. PS: Cross, Xbox: A).
    GamepadSouth(usize),
    /// The right action button of the action pad (i.e. PS: Circle, Xbox: B).
    GamepadEast(usize),
    /// The upper action button of the action pad (i.e. PS: Triangle, Xbox: Y).
    GamepadNorth(usize),
    /// The left action button of the action pad (i.e. PS: Square, Xbox: X).
    GamepadWest(usize),

    /// The C button.
    GamepadC(usize),
    /// The Z button.
    GamepadZ(usize),

    /// The first left trigger.
    GamepadLeftTrigger(usize),
    /// The second left trigger.
    GamepadLeftTrigger2(usize),
    /// The first right trigger.
    GamepadRightTrigger(usize),
    /// The second right trigger.
    GamepadRightTrigger2(usize),
    /// The select button.
    GamepadSelect(usize),
    /// The start button.
    GamepadStart(usize),
    /// The mode button.
    GamepadMode(usize),

    /// The left thumb stick button.
    GamepadLeftThumb(usize),
    /// The right thumb stick button.
    GamepadRightThumb(usize),

    /// The up button of the D-Pad.
    GamepadDPadUp(usize),
    /// The down button of the D-Pad.
    GamepadDPadDown(usize),
    /// The left button of the D-Pad.
    GamepadDPadLeft(usize),
    /// The right button of the D-Pad.
    GamepadDPadRight(usize),

    /// Miscellaneous buttons, considered non-standard (i.e. Extra buttons on a flight stick that do not have a gamepad equivalent).
    GamepadOther(u8, usize),

    /// The left mouse button.
    MouseLeft,
    /// The right mouse button.
    MouseRight,
    /// The middle mouse button.
    MouseMiddle,
    MouseBack,
    MouseForward,
    /// Another mouse button with the associated number.
    MouseOther(u16),

    /// Represents an input button that is not accessible
    /// Most often occurs when there was an error converting to a UniversalInput
    Unknown(u32),
}

impl From<KeyCode> for UniversalInput {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Unidentified(_) => UniversalInput::Unknown(0), // TODO
            KeyCode::Backquote => UniversalInput::Backquote,
            KeyCode::Backslash => UniversalInput::Backslash,
            KeyCode::BracketLeft => UniversalInput::BracketLeft,
            KeyCode::BracketRight => UniversalInput::BracketRight,
            KeyCode::Comma => UniversalInput::Comma,
            KeyCode::Digit0 => UniversalInput::Digit0,
            KeyCode::Digit1 => UniversalInput::Digit1,
            KeyCode::Digit2 => UniversalInput::Digit2,
            KeyCode::Digit3 => UniversalInput::Digit3,
            KeyCode::Digit4 => UniversalInput::Digit4,
            KeyCode::Digit5 => UniversalInput::Digit5,
            KeyCode::Digit6 => UniversalInput::Digit6,
            KeyCode::Digit7 => UniversalInput::Digit7,
            KeyCode::Digit8 => UniversalInput::Digit8,
            KeyCode::Digit9 => UniversalInput::Digit9,
            KeyCode::Equal => UniversalInput::Equal,
            KeyCode::IntlBackslash => UniversalInput::IntlBackslash,
            KeyCode::IntlRo => UniversalInput::IntlRo,
            KeyCode::IntlYen => UniversalInput::IntlYen,
            KeyCode::KeyA => UniversalInput::KeyA,
            KeyCode::KeyB => UniversalInput::KeyB,
            KeyCode::KeyC => UniversalInput::KeyC,
            KeyCode::KeyD => UniversalInput::KeyD,
            KeyCode::KeyE => UniversalInput::KeyE,
            KeyCode::KeyF => UniversalInput::KeyF,
            KeyCode::KeyG => UniversalInput::KeyG,
            KeyCode::KeyH => UniversalInput::KeyH,
            KeyCode::KeyI => UniversalInput::KeyI,
            KeyCode::KeyJ => UniversalInput::KeyJ,
            KeyCode::KeyK => UniversalInput::KeyK,
            KeyCode::KeyL => UniversalInput::KeyL,
            KeyCode::KeyM => UniversalInput::KeyM,
            KeyCode::KeyN => UniversalInput::KeyN,
            KeyCode::KeyO => UniversalInput::KeyO,
            KeyCode::KeyP => UniversalInput::KeyP,
            KeyCode::KeyQ => UniversalInput::KeyQ,
            KeyCode::KeyR => UniversalInput::KeyR,
            KeyCode::KeyS => UniversalInput::KeyS,
            KeyCode::KeyT => UniversalInput::KeyT,
            KeyCode::KeyU => UniversalInput::KeyU,
            KeyCode::KeyV => UniversalInput::KeyV,
            KeyCode::KeyW => UniversalInput::KeyW,
            KeyCode::KeyX => UniversalInput::KeyX,
            KeyCode::KeyY => UniversalInput::KeyY,
            KeyCode::KeyZ => UniversalInput::KeyZ,
            KeyCode::Minus => UniversalInput::Minus,
            KeyCode::Period => UniversalInput::Period,
            KeyCode::Quote => UniversalInput::Quote,
            KeyCode::Semicolon => UniversalInput::Semicolon,
            KeyCode::Slash => UniversalInput::Slash,
            KeyCode::AltLeft => UniversalInput::AltLeft,
            KeyCode::AltRight => UniversalInput::AltRight,
            KeyCode::Backspace => UniversalInput::Backspace,
            KeyCode::CapsLock => UniversalInput::CapsLock,
            KeyCode::ContextMenu => UniversalInput::ContextMenu,
            KeyCode::ControlLeft => UniversalInput::ControlLeft,
            KeyCode::ControlRight => UniversalInput::ControlRight,
            KeyCode::Enter => UniversalInput::Enter,
            KeyCode::SuperLeft => UniversalInput::SuperLeft,
            KeyCode::SuperRight => UniversalInput::SuperRight,
            KeyCode::ShiftLeft => UniversalInput::ShiftLeft,
            KeyCode::ShiftRight => UniversalInput::ShiftRight,
            KeyCode::Space => UniversalInput::Space,
            KeyCode::Tab => UniversalInput::Tab,
            KeyCode::Convert => UniversalInput::Convert,
            KeyCode::KanaMode => UniversalInput::KanaMode,
            KeyCode::Lang1 => UniversalInput::Lang1,
            KeyCode::Lang2 => UniversalInput::Lang2,
            KeyCode::Lang3 => UniversalInput::Lang3,
            KeyCode::Lang4 => UniversalInput::Lang4,
            KeyCode::Lang5 => UniversalInput::Lang5,
            KeyCode::NonConvert => UniversalInput::NonConvert,
            KeyCode::Delete => UniversalInput::Delete,
            KeyCode::End => UniversalInput::End,
            KeyCode::Help => UniversalInput::Help,
            KeyCode::Home => UniversalInput::Home,
            KeyCode::Insert => UniversalInput::Insert,
            KeyCode::PageDown => UniversalInput::PageDown,
            KeyCode::PageUp => UniversalInput::PageUp,
            KeyCode::ArrowDown => UniversalInput::ArrowDown,
            KeyCode::ArrowLeft => UniversalInput::ArrowLeft,
            KeyCode::ArrowRight => UniversalInput::ArrowRight,
            KeyCode::ArrowUp => UniversalInput::ArrowUp,
            KeyCode::NumLock => UniversalInput::NumLock,
            KeyCode::Numpad0 => UniversalInput::Numpad0,
            KeyCode::Numpad1 => UniversalInput::Numpad1,
            KeyCode::Numpad2 => UniversalInput::Numpad2,
            KeyCode::Numpad3 => UniversalInput::Numpad3,
            KeyCode::Numpad4 => UniversalInput::Numpad4,
            KeyCode::Numpad5 => UniversalInput::Numpad5,
            KeyCode::Numpad6 => UniversalInput::Numpad6,
            KeyCode::Numpad7 => UniversalInput::Numpad7,
            KeyCode::Numpad8 => UniversalInput::Numpad8,
            KeyCode::Numpad9 => UniversalInput::Numpad9,
            KeyCode::NumpadAdd => UniversalInput::NumpadAdd,
            KeyCode::NumpadBackspace => UniversalInput::NumpadBackspace,
            KeyCode::NumpadClear => UniversalInput::NumpadClear,
            KeyCode::NumpadClearEntry => UniversalInput::NumpadClearEntry,
            KeyCode::NumpadComma => UniversalInput::NumpadComma,
            KeyCode::NumpadDecimal => UniversalInput::NumpadDecimal,
            KeyCode::NumpadDivide => UniversalInput::NumpadDivide,
            KeyCode::NumpadEnter => UniversalInput::NumpadEnter,
            KeyCode::NumpadEqual => UniversalInput::NumpadEqual,
            KeyCode::NumpadHash => UniversalInput::NumpadHash,
            KeyCode::NumpadMemoryAdd => UniversalInput::NumpadMemoryAdd,
            KeyCode::NumpadMemoryClear => UniversalInput::NumpadMemoryClear,
            KeyCode::NumpadMemoryRecall => UniversalInput::NumpadMemoryRecall,
            KeyCode::NumpadMemoryStore => UniversalInput::NumpadMemoryStore,
            KeyCode::NumpadMemorySubtract => UniversalInput::NumpadMemorySubtract,
            KeyCode::NumpadMultiply => UniversalInput::NumpadMultiply,
            KeyCode::NumpadParenLeft => UniversalInput::NumpadParenLeft,
            KeyCode::NumpadParenRight => UniversalInput::NumpadParenRight,
            KeyCode::NumpadStar => UniversalInput::NumpadStar,
            KeyCode::NumpadSubtract => UniversalInput::NumpadSubtract,
            KeyCode::Escape => UniversalInput::Escape,
            KeyCode::Fn => UniversalInput::Fn,
            KeyCode::FnLock => UniversalInput::FnLock,
            KeyCode::PrintScreen => UniversalInput::PrintScreen,
            KeyCode::ScrollLock => UniversalInput::ScrollLock,
            KeyCode::Pause => UniversalInput::Pause,
            KeyCode::BrowserBack => UniversalInput::BrowserBack,
            KeyCode::BrowserFavorites => UniversalInput::BrowserFavorites,
            KeyCode::BrowserForward => UniversalInput::BrowserForward,
            KeyCode::BrowserHome => UniversalInput::BrowserHome,
            KeyCode::BrowserRefresh => UniversalInput::BrowserRefresh,
            KeyCode::BrowserSearch => UniversalInput::BrowserSearch,
            KeyCode::BrowserStop => UniversalInput::BrowserStop,
            KeyCode::Eject => UniversalInput::Eject,
            KeyCode::LaunchApp1 => UniversalInput::LaunchApp1,
            KeyCode::LaunchApp2 => UniversalInput::LaunchApp2,
            KeyCode::LaunchMail => UniversalInput::LaunchMail,
            KeyCode::MediaPlayPause => UniversalInput::MediaPlayPause,
            KeyCode::MediaSelect => UniversalInput::MediaSelect,
            KeyCode::MediaStop => UniversalInput::MediaStop,
            KeyCode::MediaTrackNext => UniversalInput::MediaTrackNext,
            KeyCode::MediaTrackPrevious => UniversalInput::MediaTrackPrevious,
            KeyCode::Power => UniversalInput::Power,
            KeyCode::Sleep => UniversalInput::Sleep,
            KeyCode::AudioVolumeDown => UniversalInput::AudioVolumeDown,
            KeyCode::AudioVolumeMute => UniversalInput::AudioVolumeMute,
            KeyCode::AudioVolumeUp => UniversalInput::AudioVolumeUp,
            KeyCode::WakeUp => UniversalInput::WakeUp,
            KeyCode::Meta => UniversalInput::Meta,
            KeyCode::Hyper => UniversalInput::Hyper,
            KeyCode::Turbo => UniversalInput::Turbo,
            KeyCode::Abort => UniversalInput::Abort,
            KeyCode::Resume => UniversalInput::Resume,
            KeyCode::Suspend => UniversalInput::Suspend,
            KeyCode::Again => UniversalInput::Again,
            KeyCode::Copy => UniversalInput::Copy,
            KeyCode::Cut => UniversalInput::Cut,
            KeyCode::Find => UniversalInput::Find,
            KeyCode::Open => UniversalInput::Open,
            KeyCode::Paste => UniversalInput::Paste,
            KeyCode::Props => UniversalInput::Props,
            KeyCode::Select => UniversalInput::Select,
            KeyCode::Undo => UniversalInput::Undo,
            KeyCode::Hiragana => UniversalInput::Hiragana,
            KeyCode::Katakana => UniversalInput::Katakana,
            KeyCode::F1 => UniversalInput::F1,
            KeyCode::F2 => UniversalInput::F2,
            KeyCode::F3 => UniversalInput::F3,
            KeyCode::F4 => UniversalInput::F4,
            KeyCode::F5 => UniversalInput::F5,
            KeyCode::F6 => UniversalInput::F6,
            KeyCode::F7 => UniversalInput::F7,
            KeyCode::F8 => UniversalInput::F8,
            KeyCode::F9 => UniversalInput::F9,
            KeyCode::F10 => UniversalInput::F10,
            KeyCode::F11 => UniversalInput::F11,
            KeyCode::F12 => UniversalInput::F12,
            KeyCode::F13 => UniversalInput::F13,
            KeyCode::F14 => UniversalInput::F14,
            KeyCode::F15 => UniversalInput::F15,
            KeyCode::F16 => UniversalInput::F16,
            KeyCode::F17 => UniversalInput::F17,
            KeyCode::F18 => UniversalInput::F18,
            KeyCode::F19 => UniversalInput::F19,
            KeyCode::F20 => UniversalInput::F20,
            KeyCode::F21 => UniversalInput::F21,
            KeyCode::F22 => UniversalInput::F22,
            KeyCode::F23 => UniversalInput::F23,
            KeyCode::F24 => UniversalInput::F24,
            KeyCode::F25 => UniversalInput::F25,
            KeyCode::F26 => UniversalInput::F26,
            KeyCode::F27 => UniversalInput::F27,
            KeyCode::F28 => UniversalInput::F28,
            KeyCode::F29 => UniversalInput::F29,
            KeyCode::F30 => UniversalInput::F30,
            KeyCode::F31 => UniversalInput::F31,
            KeyCode::F32 => UniversalInput::F32,
            KeyCode::F33 => UniversalInput::F33,
            KeyCode::F34 => UniversalInput::F34,
            KeyCode::F35 => UniversalInput::F35,
        }
    }
}

impl From<GamepadButton> for UniversalInput {
    fn from(value: GamepadButton) -> Self {
        let button_type = value.button_type;
        let id = value.gamepad.id;

        match button_type {
            GamepadButtonType::South => UniversalInput::GamepadSouth(id),
            GamepadButtonType::East => UniversalInput::GamepadEast(id),
            GamepadButtonType::North => UniversalInput::GamepadNorth(id),
            GamepadButtonType::West => UniversalInput::GamepadWest(id),
            GamepadButtonType::C => UniversalInput::GamepadC(id),
            GamepadButtonType::Z => UniversalInput::GamepadZ(id),
            GamepadButtonType::LeftTrigger => UniversalInput::GamepadLeftTrigger(id),
            GamepadButtonType::LeftTrigger2 => UniversalInput::GamepadLeftTrigger2(id),
            GamepadButtonType::RightTrigger => UniversalInput::GamepadRightTrigger(id),
            GamepadButtonType::RightTrigger2 => {
                UniversalInput::GamepadRightTrigger2(id)
            }
            GamepadButtonType::Select => UniversalInput::GamepadSelect(id),
            GamepadButtonType::Start => UniversalInput::GamepadStart(id),
            GamepadButtonType::Mode => UniversalInput::GamepadMode(id),
            GamepadButtonType::LeftThumb => UniversalInput::GamepadLeftThumb(id),
            GamepadButtonType::RightThumb => UniversalInput::GamepadRightThumb(id),
            GamepadButtonType::DPadUp => UniversalInput::GamepadDPadUp(id),
            GamepadButtonType::DPadDown => UniversalInput::GamepadDPadDown(id),
            GamepadButtonType::DPadLeft => UniversalInput::GamepadDPadLeft(id),
            GamepadButtonType::DPadRight => UniversalInput::GamepadDPadRight(id),
            GamepadButtonType::Other(c) => UniversalInput::GamepadOther(c, id),
        }
    }
}

impl From<MouseButton> for UniversalInput {
    fn from(value: MouseButton) -> Self {
        match value {
            MouseButton::Left => UniversalInput::MouseLeft,
            MouseButton::Right => UniversalInput::MouseRight,
            MouseButton::Middle => UniversalInput::MouseMiddle,
            MouseButton::Back => UniversalInput::MouseBack,
            MouseButton::Forward => UniversalInput::MouseForward,
            MouseButton::Other(id) => UniversalInput::MouseOther(id),
        }
    }
}
