use actix_web::{web, get, HttpResponse, Responder};
use crate::models::Todo;

#[get("/todos/list")]
pub async fn todos_list(todos: web::Data<Vec<Todo>>) -> impl Responder {

    let todos_list_html = todos
        .iter()
        .map(|todo| format!("<li>{}</li>", todo.task))
        .collect::<Vec<String>>().join("\n");

    HttpResponse::Ok().body(todos_list_html)
}