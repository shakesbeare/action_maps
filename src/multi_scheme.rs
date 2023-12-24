use std::collections::HashMap;
use bevy_ecs::system::Resource;
use crate::control_scheme::ControlScheme;

/// Helper type to be used for local multiplayer
/// ```rust
/// use action_maps::multiplayer::*;
/// use action_maps::get_scan_code;
/// use bevy::prelude::*;
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum Actions {
///     Up,
///     Left,
///     Down,
///     Right,
/// }
/// 
/// impl From<Actions> for Action {
///     fn from(value: Actions) -> Self {
///         match value {
///             Actions::Up => Action::from("Up"),
///             Actions::Left => Action::from("Left"),
///             Actions::Down => Action::from("Down"),
///             Actions::Right => Action::from("Right"),
///         }
///     }
/// }
///
/// fn bind_keys(mut controls: ResMut<MultiScheme>) {
///     let wasd = ControlScheme::with_controls(vec![
///         (Actions::Up, ScanCode(get_scan_code("W"))),
///         (Actions::Left, ScanCode(get_scan_code("A"))),
///         (Actions::Down, ScanCode(get_scan_code("S"))),
///         (Actions::Right, ScanCode(get_scan_code("D"))),
///    ]);
///    controls.insert(0 /* player id */, wasd);
/// }
/// ```
#[derive(Debug, Clone, Resource, Default)]
pub struct MultiScheme {
    map: HashMap<usize, ControlScheme>
}

impl MultiScheme {
    pub fn new() -> Self {
        let map = HashMap::new();
        Self {
            map,
        }
    }

    pub fn get(&self, id: usize) -> Option<&ControlScheme> {
        self.map.get(&id)
    }

    pub fn insert(&mut self, id: usize, action_input: ControlScheme) {
        self.map.insert(id, action_input);
    }

    pub fn remove(&mut self, id: usize) {
        self.map.remove(&id);
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, usize, ControlScheme> {
        self.map.keys()
    }
}
