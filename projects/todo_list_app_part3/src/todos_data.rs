use crate::models::Todo;

#[derive(Clone)]
pub struct TodosData {
    todos: Vec<Todo>,
}

impl TodosData {
    pub fn new() -> Self {
        Self {
            todos: vec![],
        }
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Todo> {
        self.todos.iter()
    }
}