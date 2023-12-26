extern crate bevy;

use action_maps::multiplayer_prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Controllable;

#[derive(Component)]
struct PlayerId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Actions {
    Up,
    Left,
    Down,
    Right,
}

impl From<Actions> for Action {
    fn from(value: Actions) -> Self {
        match value {
            Actions::Up => Action::from("Up"),
            Actions::Left => Action::from("Left"),
            Actions::Down => Action::from("Down"),
            Actions::Right => Action::from("Right"),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MultiActionMapPlugin)
        .add_systems(PreStartup, setup)
        .add_systems(PreUpdate, handle_input.in_set(ActionMapSet::HandleActions))
        .run();
}

fn setup(
    mut commands: Commands,
    mut inputs: ResMut<MultiInput>,
    mut control_schemes: ResMut<MultiScheme>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    make_multi_input!(
        inputs,
        control_schemes,
        (
            (Actions::Up, KeyCode::KeyW),
            (Actions::Left, KeyCode::KeyA),
            (Actions::Down, KeyCode::KeyS),
            (Actions::Right, KeyCode::KeyD),
        ),
        (
            (Actions::Up, KeyCode::ArrowUp),
            (Actions::Left, KeyCode::ArrowLeft),
            (Actions::Down, KeyCode::ArrowDown),
            (Actions::Right, KeyCode::ArrowRight),
        )
    );

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-75., 0., 1.)),
            ..default()
        },
        Controllable,
        PlayerId(0),
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
        Controllable,
        PlayerId(1),
    ));
}

fn handle_input(
    multi_input: Res<MultiInput>,
    mut query: Query<(&mut Transform, &PlayerId), With<Controllable>>,
) {
    for (mut transform, PlayerId(id)) in query.iter_mut() {
        let actions = multi_input.get(*id).unwrap();
        if actions.pressed(Actions::Left) {
            transform.translation.x -= 1.;
        }
        if actions.pressed(Actions::Right) {
            transform.translation.x += 1.;
        }
        if actions.pressed(Actions::Up) {
            transform.translation.y += 1.;
        }
        if actions.pressed(Actions::Down) {
            transform.translation.y -= 1.;
        }
    }
}
