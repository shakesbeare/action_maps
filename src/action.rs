use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet},
    system::{Res, ResMut},
};
use bevy_input::{
    gamepad::GamepadButton,
    keyboard::{KeyCode, ScanCode},
    mouse::MouseButton,
    Input as BevyInput,
};

use crate::input::Input;

use crate::control_scheme::ControlScheme;
use crate::input_type::Key;
use crate::input_type::UniversalInput;

#[derive(Hash, Debug, PartialEq, Eq, Clone, SystemSet)]
pub enum UniversalInputSet {
    ReadEvents,
    HandleActions,
}

unsafe impl Send for Action {}
unsafe impl Sync for Action {}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action {
    name: &'static str,
}

impl From<&'static str> for Action {
    fn from(name: &'static str) -> Self {
        Action { name }
    }
}

pub fn action_input_system(
    mut actions: ResMut<Input>,
    keycodes: Res<BevyInput<KeyCode>>,
    scancodes: Res<BevyInput<ScanCode>>,
    mouse_buttons: Res<BevyInput<MouseButton>>,
    gamepad_buttons: Res<BevyInput<GamepadButton>>,
    control_scheme: Res<ControlScheme>,
) {
    actions.bypass_change_detection().clear();
    for (action, input) in control_scheme.iter() {
        match input {
            UniversalInput::Keyboard(key) => match key {
                Key::KeyCode(kc) => {
                    if keycodes.just_pressed(*kc) {
                        actions.press(*action);
                    }
                    if keycodes.just_released(*kc) {
                        actions.release(*action);
                    }
                }
                Key::ScanCode(sc) => {
                    if scancodes.just_pressed(*sc) {
                        actions.press(*action);
                    }
                    if scancodes.just_released(*sc) {
                        actions.release(*action);
                    }
                }
            },
            UniversalInput::MouseButton(mb) => {
                if mouse_buttons.just_pressed(*mb) {
                    actions.press(*action);
                }
                if mouse_buttons.just_released(*mb) {
                    actions.release(*action);
                }
            }
            UniversalInput::GamepadButton(gb) => {
                if gamepad_buttons.just_pressed(*gb) {
                    actions.press(*action);
                }
                if gamepad_buttons.just_released(*gb) {
                    actions.release(*action);
                }
            }
        }
    }
}

pub struct ActionMapPlugin;

impl Plugin for ActionMapPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            UniversalInputSet::HandleActions
                .after(UniversalInputSet::ReadEvents),
        )
        .init_resource::<ControlScheme>()
        .init_resource::<Input>()
        .add_systems(
            PreUpdate,
            (action_input_system)
                .in_set(UniversalInputSet::ReadEvents),
        );
    }
}
