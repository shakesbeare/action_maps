use bevy_ecs::system::Resource;
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

pub struct Action<'a> {
    pub action_name: String,
    pub input_type: InputType,
    pub input: ActionInput,
    pub callback: &'a mut dyn FnMut(),
}

#[derive(Resource)]
pub struct ActionMap<'a> {
    keyboard_actions: Vec<Action<'a>>,
    mouse_actions: Vec<Action<'a>>,
    touch_actions: Vec<Action<'a>>,
    gamepad_button_actions: Vec<Action<'a>>,
    gamepad_axis_actions: Vec<Action<'a>>,
}

impl<'a> ActionMap<'a> {
    pub fn new() -> Self {
        Self {
            keyboard_actions: Vec::new(),
            mouse_actions: Vec::new(),
            touch_actions: Vec::new(),
            gamepad_button_actions: Vec::new(),
            gamepad_axis_actions: Vec::new(),
        }
    }

    pub fn register<'b>(&mut self, action: Action<'b>)
    where
        'b: 'a,
        'a: 'b,
    {
        match action.input_type {
            InputType::Keyboard => self.keyboard_actions.push(action),
            InputType::Mouse => self.mouse_actions.push(action),
            InputType::Touch => self.touch_actions.push(action),
            InputType::GamepadButton => {
                self.gamepad_button_actions.push(action)
            }
            InputType::GamepadAxis => self.gamepad_axis_actions.push(action),
        }
    }

    #[allow(dead_code)]
    fn get_action_callback<T: Into<ActionInput>>(
        &mut self,
        lookup_input: T,
    ) -> Option<&mut dyn FnMut()> {
        let lookup_input = lookup_input.into();
        match lookup_input {
            ActionInput::Keyboard(_) => {
                for action in &mut self.keyboard_actions {
                    if action.input == lookup_input {
                        return Some(&mut *action.callback);
                    }
                }
            }
            ActionInput::Mouse(_) => {
                for action in &mut self.mouse_actions {
                    if action.input == lookup_input {
                        return Some(&mut *action.callback);
                    }
                }
            }
            ActionInput::Touch(_) => {
                for action in &mut self.touch_actions {
                    if action.input == lookup_input {
                        return Some(&mut *action.callback);
                    }
                }
            }
            ActionInput::GamepadButton(_) => {
                for action in &mut self.gamepad_button_actions {
                    if action.input == lookup_input {
                        let cb = &mut *action.callback;
                        return Some(&mut *cb);
                    }
                }
            }
            ActionInput::GamepadAxis(_) => {
                for action in &mut self.gamepad_axis_actions {
                    if action.input == lookup_input {
                        return Some(&mut *action.callback);
                    }
                }
            }
        }

        return None;
    }
}

impl<'a> Default for ActionMap<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestState {
        callback_called_count: usize,
    }

    #[test]
    fn test_action_map() {
        let mut state: TestState = TestState {
            callback_called_count: 0,
        };

        let mut action_map = ActionMap::new();
        let mut callback = || {
            state.callback_called_count += 1;
        };

        callback(); // call the first time

        action_map.register(Action {
            action_name: "test".to_string(),
            input_type: InputType::Keyboard,
            input: ActionInput::Keyboard(Key::KeyCode(KeyCode::A)),
            callback: &mut callback,
        });

        let expected_callback = action_map
            .get_action_callback(KeyCode::A)
            .unwrap();

        expected_callback(); // call the second time after it passes through the system

        assert_eq!(state.callback_called_count, 2);
    }

}
