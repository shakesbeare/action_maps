use bevy::prelude::*;
use action_maps::multiplayer_prelude::*;

#[test]
fn multi_resource_responds_to_update() {
    let mut app = App::new();

    app.configure_sets(
        PreUpdate,
        ActionMapSet::HandleActions.after(ActionMapSet::ReadEvents),
    )
    .add_systems(
        PreUpdate,
        (action_maps::multi_action_input_system).in_set(ActionMapSet::ReadEvents),
    );

    app.insert_resource(Input::<KeyCode>::default());
    app.insert_resource(Input::<ScanCode>::default());
    app.insert_resource(Input::<GamepadButton>::default());
    app.insert_resource(Input::<MouseButton>::default());

    let mut mi = MultiInput::default();
    let mut ms = MultiScheme::default();

    make_multi_input!(mi, ms,
        (
            ("Left", KeyCode::A),
        ),
        (
            ("Left", KeyCode::Left),
        )
    );

    app.insert_resource(mi);
    app.insert_resource(ms);
    app.update();

    app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::A);
    app.update(); // expected one frame delay

    let mi = app.world.resource::<MultiInput>();
    assert!(mi.get(0).unwrap().pressed("Left"));
    assert!(!mi.get(1).unwrap().pressed("Left"));

    app.world.resource_mut::<Input<KeyCode>>().release(KeyCode::A);
    app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Left);
    app.update(); // expected one frame delay

    let mi = app.world.resource::<MultiInput>();
    assert!(!mi.get(0).unwrap().pressed("Left"));
    assert!(mi.get(1).unwrap().pressed("Left"));
}
