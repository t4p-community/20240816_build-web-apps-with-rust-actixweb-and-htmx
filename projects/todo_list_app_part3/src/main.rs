mod models;
mod todos_data;
mod todos_routes;

use std::sync::{Arc, RwLock};
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use crate::models::Todo;
use crate::todos_data::TodosData;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut todos_data = TodosData::new();
    todos_data.add_todo(Todo {
        task: "Buy groceries".to_string(),
    });
    todos_data.add_todo(Todo {
        task: "Walk the dog".to_string(),
    });
    todos_data.add_todo(Todo {
        task: "Wash the car".to_string(),
    });

    let todos_data = Arc::new(RwLock::new(todos_data));
    
    HttpServer::new(move || {
        let todos_data = web::Data::new(todos_data.clone());
        App::new()
            .app_data(todos_data)
            .service(todos_routes::todos_list)
            .service(todos_routes::todos_add)
            .service(fs::Files::new("/", "static").index_file("index.html"))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}