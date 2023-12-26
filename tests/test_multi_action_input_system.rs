use action_maps::multiplayer_prelude::*;
use bevy::prelude::*;
use bevy_input::{keyboard::KeyboardInput, ButtonState};

#[test]
fn multi_resource_responds_to_update() {
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
        (action_maps::input::multi_universal_input_system)
            .in_set(ActionMapSet::ReadEvents),
    );

    app.insert_resource(Input::<KeyCode>::default());
    app.insert_resource(Input::<ScanCode>::default());
    app.insert_resource(Input::<GamepadButton>::default());
    app.insert_resource(Input::<MouseButton>::default());

    let press_a = KeyboardInput {
        scan_code: 0x01,
        key_code: None,
        state: ButtonState::Pressed,
        window: bevy_ecs::entity::Entity::from_raw(0),
    };
    let release_a = KeyboardInput {
        scan_code: 0x01,
        key_code: None,
        state: ButtonState::Released,
        window: bevy_ecs::entity::Entity::from_raw(0),
    };
    let press_left = KeyboardInput {
        scan_code: 0xFF,
        key_code: None,
        state: ButtonState::Pressed,
        window: bevy_ecs::entity::Entity::from_raw(0),
    };

    let mut mi = MultiInput::default();
    let mut ms = MultiScheme::default();

    make_multi_input!(
        mi,
        ms,
        (("Left", ScanCode(0x01)),),
        (("LeftArrow", ScanCode(0xFF)),)
    );

    app.insert_resource(mi);
    app.insert_resource(ms);
    app.update();

    app.world
        .resource_mut::<Events<KeyboardInput>>()
        .send(press_a);
    app.update();
    let mi = app.world.resource_mut::<MultiInput>();
    assert!(mi.get(0).unwrap().pressed("Left"));
    assert!(!mi.get(1).unwrap().pressed("LeftArrow"));

    app.world
        .resource_mut::<Events<KeyboardInput>>()
        .send(release_a);
    app.world
        .resource_mut::<Events<KeyboardInput>>()
        .send(press_left);
    app.update();

    let mi = app.world.resource_mut::<MultiInput>();
    assert!(!mi.get(0).unwrap().pressed("Left"));
    assert!(mi.get(1).unwrap().pressed("LeftArrow"));
}
