use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Todo {
    pub task: String,
}
