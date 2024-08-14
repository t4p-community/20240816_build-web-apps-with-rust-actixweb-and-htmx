use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct NewTodo {
    pub task: String,
    pub completed: bool,
}


#[derive(Clone, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub task: String,
    pub completed: bool,
}
