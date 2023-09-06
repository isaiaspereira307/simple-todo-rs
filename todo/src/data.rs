use im::Vector;
use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Data, Lens, Default)]
pub struct TodoState {
    pub todos: Vector<TodoItem>,
    pub new_text: String,
}

#[derive(Debug, Clone, Data, Lens, Default, PartialEq, Deserialize, Serialize)]
pub struct TodoItem {
    pub checked: bool,
    pub text: String,
}