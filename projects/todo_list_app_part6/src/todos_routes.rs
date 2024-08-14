use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use actix_web::{
    get, post, web, HttpResponse, Responder, Result,
    error::InternalError, http::StatusCode, Error
};

use crate::todos_data::TodosData;
use crate::models::NewTodo;


pub fn render_todo_list_html(todos_data: &TodosData) -> String {
    let todos_list_html = todos_data
        .iter()
        .map(|todo| {
            let completed = if todo.completed {
                "checked"
            } else {
                ""
            };
            format!(r##"
                <li>
                    <form id="todo_toggle_complete_form_{}" hx-post="/todos/toggle_completed"
                        hx-target="#todos-list"
                        hx-trigger="click from:(form#todo_toggle_complete_form_{} input[type=checkbox])">
                        <input type='hidden' name='todo_id' value='{}'>
                        <input type='checkbox' {}>
                    </form>
                    {}
                    <form id="todo_remove_form_{}" hx-post="/todos/remove"
                        hx-target="#todos-list">
                        <input type='hidden' name='todo_id' value='{}'>
                        <button>X</button>
                    </form>
                </li>"##,
                todo.id, todo.id, todo.id, completed,
                todo.task, todo.id, todo.id)
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
    new_todo: web::Form<NewTodo>) -> Result<impl Responder> {

    if new_todo.task == "makeerror" {
        return Err::<HttpResponse, Error>(
            InternalError::new(
                "Example Error",
                StatusCode::INTERNAL_SERVER_ERROR).into());
    }

    let mut todos_data = todos_data
        .write()
        .map_err(
            |_| InternalError::new("Failed to acquire write lock", StatusCode::INTERNAL_SERVER_ERROR)
        )?;

    todos_data.add_todo(new_todo.into_inner());

    let todos_list_html = render_todo_list_html(&todos_data);
    Ok(HttpResponse::Ok().body(todos_list_html))
}

#[post("/todos/remove")]
pub async fn todos_remove(
    todos_data: web::Data<Arc<RwLock<TodosData>>>,
    todo_id: web::Form<HashMap<String, String>>) -> Result<impl Responder> {

    let todo_id: i32 = match todo_id.get("todo_id")
        .and_then(|id| id.parse().ok()) {
            Some(id) => id,
            None => return Ok(HttpResponse::BadRequest().body("Invalid todo_id")),
        };

    let mut todos_data = todos_data
        .write()
        .map_err(
            |_| InternalError::new("Failed to acquire write lock", StatusCode::INTERNAL_SERVER_ERROR)
        )?;
    todos_data.remove_todo(todo_id);

    let todos_list_html = render_todo_list_html(&todos_data);
    Ok(HttpResponse::Ok().body(todos_list_html))
}

#[post("/todos/toggle_completed")]
pub async fn todos_toggle_completed(
    todos_data: web::Data<Arc<RwLock<TodosData>>>,
    todo_id: web::Form<HashMap<String, String>>) -> Result<impl Responder> {

    let todo_id: i32 = match todo_id.get("todo_id")
        .and_then(|id| id.parse().ok()) {
            Some(id) => id,
            None => return Ok(HttpResponse::BadRequest().body("Invalid todo_id")),
        };

    let mut todos_data = todos_data
        .write()
        .map_err(
            |_| InternalError::new("Failed to acquire write lock", StatusCode::INTERNAL_SERVER_ERROR)
        )?;

    todos_data.toggle_completed(todo_id);

    let todos_list_html = render_todo_list_html(&todos_data);
    Ok(HttpResponse::Ok().body(todos_list_html))
}