extern crate bevy;

use action_maps::get_scan_code;
use action_maps::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct Controllable;

#[derive(Component)]
enum HasColor {
    Red,
    Purple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Actions {
    Up,
    Left,
    Down,
    Right,
    ChangeColor,
}

impl From<Actions> for Action {
    fn from(value: Actions) -> Self {
        match value {
            Actions::Up => Action::from("Up"),
            Actions::Left => Action::from("Left"),
            Actions::Down => Action::from("Down"),
            Actions::Right => Action::from("Right"),
            Actions::ChangeColor => Action::from("ChangeColor"),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ActionMapPlugin)
        .add_systems(PreStartup, setup)
        .add_systems(PreUpdate, handle_input.in_set(ActionMapSet::HandleActions))
        .run();
}

fn setup(
    mut commands: Commands,
    mut control_scheme: ResMut<ControlScheme>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    control_scheme.set(make_controls!(
        (Actions::Up, ScanCode(get_scan_code("W").unwrap())),
        (Actions::Left, ScanCode(get_scan_code("A").unwrap())),
        (Actions::Down, ScanCode(get_scan_code("S").unwrap())),
        (Actions::Right, ScanCode(get_scan_code("D").unwrap())),
        (Actions::ChangeColor, MouseButton::Left)
    ));

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
        Controllable,
        HasColor::Purple,
    ));
}

fn handle_input(
    actions: Res<ActionInput>,
    mut query: Query<&mut Transform, With<Controllable>>,
    mut mats: Query<(&mut Handle<ColorMaterial>, &mut HasColor)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for mut transform in query.iter_mut() {
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

    if actions.just_pressed(Actions::ChangeColor) {
        let (mut mat, mut has_color) = mats.single_mut();
        match *has_color {
            HasColor::Red => {
                *mat = materials.add(ColorMaterial::from(Color::PURPLE));
                *has_color = HasColor::Purple;
            }
            HasColor::Purple => {
                *mat = materials.add(ColorMaterial::from(Color::RED));
                *has_color = HasColor::Red;
            }
        }
    }
}
