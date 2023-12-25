use bevy::prelude::*;
use action_maps::prelude::*;

#[test]
fn resource_responds_to_update() {
    let mut app = App::new();

    app.configure_sets(
        PreUpdate,
        ActionMapSet::HandleActions.after(ActionMapSet::ReadEvents),
    )
    .add_systems(
        PreUpdate,
        (action_maps::action_input_system).in_set(ActionMapSet::ReadEvents),
    );

    app.insert_resource(Input::<KeyCode>::default());
    app.insert_resource(Input::<ScanCode>::default());
    app.insert_resource(Input::<GamepadButton>::default());
    app.insert_resource(Input::<MouseButton>::default());

    let mut cs = ControlScheme::default();
    cs.insert("A", KeyCode::A);

    app.insert_resource(cs);
    app.insert_resource(ActionInput::default());
    app.update();

    app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::A);
    app.update(); // expected one frame delay

    let ai = app.world.resource::<ActionInput>();
    assert!(ai.pressed("A"));
    assert!(ai.just_pressed("A"));
    app.update();
    
    let ai = app.world.resource::<ActionInput>();
    assert!(ai.pressed("A"));
    assert!(!ai.just_pressed("A"));

    app.world.resource_mut::<Input<KeyCode>>().release_all();
    app.update(); // expected one frame delay
    let ai = app.world.resource::<ActionInput>();
    assert!(!ai.pressed("A"));
}

