use crate::models::{Todo, NewTodo};

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

    pub fn add_todo(&mut self, new_todo: NewTodo) {

        let id = self.todos
            .iter()
            .map(|todo| todo.id)
            .max()
            .unwrap_or(0) + 1;

        let todo = Todo {
            id,
            task: new_todo.task,
            completed: new_todo.completed,
        };

        self.todos.push(todo);
    }

    pub fn remove_todo(&mut self, id: i32) {
        self.todos.retain(|todo| todo.id != id);
    }

    pub fn toggle_completed(&mut self, id: i32) {
        for todo in self.todos.iter_mut() {
            if todo.id == id {
                todo.completed = !todo.completed;
                break;
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Todo> {
        self.todos.iter()
    }
}