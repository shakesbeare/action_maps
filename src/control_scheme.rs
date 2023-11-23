use bevy_ecs::system::Resource;
use std::collections::HashMap;

use crate::action::Action;
use crate::input_type::UniversalInput;

#[derive(Resource, Default)]
pub struct ControlScheme(HashMap<Action, UniversalInput>);

#[allow(dead_code)]
impl ControlScheme {
    pub fn insert<A, I>(&mut self, action: A, input: I)
    where
        A: Into<Action>,
        I: Into<UniversalInput>,
    {
        self.0.insert(action.into(), input.into());
    }

    pub fn remove<A>(&mut self, action: A)
    where
        A: Into<Action>,
    {
        self.0.remove(&action.into());
    }

    pub fn get<A, I>(&self, action: A) -> Option<&UniversalInput>
    where
        A: Into<Action>,
    {
        self.0.get(&action.into())
    }

    pub fn get_mut<A>(&mut self, action: A) -> Option<&mut UniversalInput>
    where
        A: Into<Action>,
    {
        self.0.get_mut(&action.into())
    }

    pub fn contains_key<A>(&self, action: A) -> bool
    where
        A: Into<Action>,
    {
        self.0.contains_key(&action.into())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Action, &UniversalInput)> {
        self.0.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&Action, &mut UniversalInput)> {
        self.0.iter_mut()
    }
}