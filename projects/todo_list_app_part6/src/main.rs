mod models;
mod todos_data;
mod todos_routes;

use std::sync::{Arc, RwLock};
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use crate::models::NewTodo;
use crate::todos_data::TodosData;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut todos_data = TodosData::new();
    todos_data.add_todo(NewTodo {
        task: "Buy groceries".to_string(),
        completed: false,
    });
    todos_data.add_todo(NewTodo {
        task: "Walk the dog".to_string(),
        completed: true,
    });
    todos_data.add_todo(NewTodo {
        task: "Wash the car".to_string(),
        completed: false,
    });

    let todos_data = Arc::new(RwLock::new(todos_data));
    
    HttpServer::new(move || {
        let todos_data = web::Data::new(todos_data.clone());
        App::new()
            .app_data(todos_data)
            .service(todos_routes::todos_list)
            .service(todos_routes::todos_add)
            .service(todos_routes::todos_remove)
            .service(todos_routes::todos_toggle_completed)
            .service(fs::Files::new("/", "static").index_file("index.html"))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}