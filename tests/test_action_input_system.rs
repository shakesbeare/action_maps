use action_maps::prelude::*;
use bevy::prelude::*;
use bevy::input::{keyboard::KeyboardInput, ButtonState};

#[test]
fn resource_responds_to_update() {
    let mut app = App::new();

    app.configure_sets(
        PreUpdate,
        ActionMapSet::HandleActions.after(ActionMapSet::ReadEvents),
    )
    .add_event::<bevy::input::keyboard::KeyboardInput>()
    .add_event::<bevy::input::gamepad::GamepadButtonChangedEvent>()
    .add_event::<bevy::input::gamepad::GamepadButtonInput>()
    .add_event::<bevy::input::mouse::MouseButtonInput>()
    .insert_resource(bevy::input::gamepad::GamepadSettings::default())
    .add_systems(
        PreUpdate,
        (action_maps::input::universal_input_system).in_set(ActionMapSet::ReadEvents),
    );
    let press_key = KeyboardInput {
        key_code: KeyCode::KeyA,
        state: ButtonState::Pressed,
        window: bevy::ecs::entity::Entity::from_raw(0),
    };
    let release_key = KeyboardInput {
        key_code: KeyCode::KeyA,
        state: ButtonState::Released,
        window: bevy::ecs::entity::Entity::from_raw(0),
    };

    let mut cs = ControlScheme::default();
    cs.insert("A", KeyCode::KeyA);
    app.insert_resource(cs);
    app.insert_resource(ActionInput::default());
    app.update();

    // press key
    app.world
        .resource_mut::<Events<KeyboardInput>>()
        .send(press_key);
    app.update();
    let ai = app.world.resource_mut::<ActionInput>();
    assert!(ai.pressed("A"));
    assert!(ai.just_pressed("A"));
    app.update();

    let ai = app.world.resource::<ActionInput>();
    assert!(ai.pressed("A"));
    assert!(!ai.just_pressed("A"));

    // release key
    app.world
        .resource_mut::<Events<KeyboardInput>>()
        .send(release_key);
    app.update();
    let ai = app.world.resource_mut::<ActionInput>();
    assert!(!ai.pressed("A"));

    app.update();
    let ai = app.world.resource::<ActionInput>();
    assert!(!ai.pressed("A"));
}
