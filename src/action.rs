use crate::input::{ActionInput, InputType};

pub struct Action<'a> {
    pub action_name: String,
    pub input_type: InputType,
    pub input: ActionInput,
    pub callback: &'a mut dyn FnMut(),
}
