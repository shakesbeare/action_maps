use bevy_input::gamepad::GamepadAxisType;
use bevy_input::gamepad::GamepadButtonType;
use bevy_input::keyboard::KeyCode;
use bevy_input::keyboard::ScanCode;
use bevy_input::mouse::MouseButton;
use bevy_input::touch::TouchPhase;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Key {
    KeyCode(KeyCode),
    ScanCode(ScanCode),
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ActionInput {
    Keyboard(Key),
    Mouse(MouseButton),
    Touch(TouchPhase),
    GamepadButton(GamepadButtonType),
    GamepadAxis(GamepadAxisType),
}

impl From<KeyCode> for ActionInput {
    fn from(key_code: KeyCode) -> Self {
        ActionInput::Keyboard(Key::KeyCode(key_code))
    }
}

impl From<ScanCode> for ActionInput {
    fn from(scan_code: ScanCode) -> Self {
        ActionInput::Keyboard(Key::ScanCode(scan_code))
    }
}

impl From<GamepadButtonType> for ActionInput {
    fn from(button_type: GamepadButtonType) -> Self {
        ActionInput::GamepadButton(button_type)
    }
}

impl From<GamepadAxisType> for ActionInput {
    fn from(axis_type: GamepadAxisType) -> Self {
        ActionInput::GamepadAxis(axis_type)
    }
}

impl From<MouseButton> for ActionInput {
    fn from(mouse_button: MouseButton) -> Self {
        ActionInput::Mouse(mouse_button)
    }
}

impl From<TouchPhase> for ActionInput {
    fn from(touch_phase: TouchPhase) -> Self {
        ActionInput::Touch(touch_phase)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum InputType {
    Keyboard,
    Mouse,
    Touch,
    GamepadButton,
    GamepadAxis,
}


