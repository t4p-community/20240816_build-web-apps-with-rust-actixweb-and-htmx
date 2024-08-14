mod models;
mod todos_routes;

use actix_files as fs;
use actix_web::{web, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let todos = vec![
        models::Todo {
            task: "Buy groceries".to_string(),
        },
        models::Todo {
            task: "Walk the dog".to_string(),
        },
        models::Todo {
            task: "Wash the car".to_string(),
        },
    ];


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(todos.clone()))
            .service(todos_routes::todos_list)
            .service(fs::Files::new("/", "static").index_file("index.html"))
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}