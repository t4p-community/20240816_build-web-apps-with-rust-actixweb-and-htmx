use actix_web::{get, post, web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::models::NewTodo;
use crate::todos_data::TodosData;

pub fn render_todo_list_html(todos_data: &TodosData) -> String {
    let todos_list_html = todos_data
        .iter()
        .map(|todo| {
            let completed = if todo.completed { "checked" } else { "" };
            format!(
                r##"
                <li><form id="todo_form_{}" hx-post="/todos/toggle_completed"
                        hx-swap="innerHTML"
                        hx-target="#todos-list"
                        hx-trigger="click from:(form#todo_form_{} input[type=checkbox])">
                    <input type='hidden' name='todo_id' value='{}'>
                    <input type='checkbox' {}> {}
                </form></li>"##,
                todo.id, todo.id, todo.id, completed, todo.task
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!("<ul>{}</ul>", todos_list_html)
}

#[get("/todos/list")]
pub async fn todos_list(todos_data: web::Data<Arc<RwLock<TodosData>>>) -> impl Responder {
    let todos_data = todos_data.read().unwrap();
    let todos_list_html = render_todo_list_html(&todos_data);
    HttpResponse::Ok().body(todos_list_html)
}

#[post("/todos/add")]
pub async fn todos_add(
    todos_data: web::Data<Arc<RwLock<TodosData>>>,
    new_todo: web::Form<NewTodo>,
) -> impl Responder {
    let mut todos_data = todos_data.write().unwrap();
    todos_data.add_todo(new_todo.into_inner());

    let todos_list_html = render_todo_list_html(&todos_data);
    HttpResponse::Ok().body(todos_list_html)
}

#[post("/todos/toggle_completed")]
pub async fn todos_toggle_completed(
    todos_data: web::Data<Arc<RwLock<TodosData>>>,
    todo_id: web::Form<HashMap<String, String>>,
) -> impl Responder {
    let todo_id: i32 = todo_id
        .get("todo_id")
        .and_then(|id| id.parse().ok())
        .expect("Invalid todo_id");

    let mut todos_data = todos_data.write().unwrap();
    todos_data.toggle_completed(todo_id);

    let todos_list_html = render_todo_list_html(&todos_data);
    HttpResponse::Ok().body(todos_list_html)
}
