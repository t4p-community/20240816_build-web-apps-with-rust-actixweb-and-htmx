use std::sync::{Arc, RwLock};
use actix_web::{get, post, web, HttpResponse, Responder};

use crate::todos_data::TodosData;
use crate::models::Todo;


#[get("/todos/list")]
pub async fn todos_list(todos_data: web::Data<Arc<RwLock<TodosData>>>) -> impl Responder {

    let todos_data = todos_data.read().unwrap();

    let todos_list_html = todos_data
        .iter()
        .map(|todo| format!("<li>{}</li>", todo.task))
        .collect::<Vec<String>>().join("\n");

    HttpResponse::Ok().body(todos_list_html)
}

#[post("/todos/add")]
pub async fn todos_add(
    todos_data: web::Data<Arc<RwLock<TodosData>>>,
    new_todo: web::Form<Todo>) -> impl Responder {

    let mut todos_data = todos_data.write().unwrap();
    todos_data.add_todo(new_todo.into_inner());

    let todos_list_html = todos_data
        .iter()
        .map(|todo| format!("<li>{}</li>", todo.task))
        .collect::<Vec<String>>().join("\n");

    HttpResponse::Ok().body(todos_list_html)
}