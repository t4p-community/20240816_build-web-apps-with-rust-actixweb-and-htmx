use actix_web::{get, HttpResponse, Responder};

struct Todo {
    task: String,
}

#[get("/todos/list")]
pub async fn todos_list() -> impl Responder {
    let todos = [
        Todo {
            task: "Buy groceries".to_string(),
        },
        Todo {
            task: "Walk the dog".to_string(),
        },
        Todo {
            task: "Wash the car".to_string(),
        },
    ];

    let todos_list_html = todos
        .iter()
        .map(|todo| format!("<li>{}</li>", todo.task))
        .collect::<Vec<String>>()
        .join("\n");

    HttpResponse::Ok().body(todos_list_html)
}
