# Hello World Actix Web and HTMX Project

1. Create a new Rust project with the following command:

    ```bash
    cargo new hello_world_actixweb_htmx
    ```

1. Change in to the project folder.

    ```bash
    cd hello_world_actixweb_htmx
    ```

1. Add Actix Web to the project.

    ```bash
    cargo add actix-web
    ```

    This will add the `actix-web` crate to the `Cargo.toml` file for the project.

1. Open the `src/main.rs` file and add the following code:

    ```rust
    use actix_web::{get, App, HttpResponse, HttpServer, Responder};

    // setups a handler for the "/" URL path, when called it will return
    // a "Hello world!" response
    #[get("/")]
    async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    // run the main web application
    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(hello) // register the hello handler
        })
        // bind the server to the IP address and port
        .bind(("127.0.0.1", 8080))? 
        .run() // runs the server and waits for connections
        .await
    }
    ```

1. From the terminal, run the project with the following command:

    ```bash
    cargo run
    ```

1. Open a web browser, and navigate to `http://127.0.0.1:8080`. You should see the text `Hello world!` displayed on the page.

    So the web server is running and responding to requests. Now, we need to integrate an `index.html` file into the application.

    To stop the web server, type `Ctrl+C` in the terminal.

1. The `index.html` file will be a static file. Create a new folder called `static` in the `src` folder. From the root folder of the project, in the terminal, run the following command.

    ```bash
    mkdir static
    ```

1. Create a new file called `index.html` in the `src/static` folder, and add the following content to the file.

    ```html
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Hello World</title>
    </head>
    <body>
        <h1>Hello world!</h1>
    </body>
    </html>
    ```

1. To serve this file with Actix Web, we need to install the `actix-files` crate. Add the crate to the `Cargo.toml` file the following `cargo add` command.

    ```bash
    cargo add actix-files
    ```

1. Replace the contents of the `main.rs` file with this Rust code.

    ```rust
    use actix_files as fs;
    use actix_web::{App, HttpServer};

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(move || {
            // use actix_files to serve static files
            App::new().service(fs::Files::new("/", "static").index_file("index.html"))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
    ```

1. From the terminal, run the project with the following command:

    ```bash
    cargo run
    ```

1. Open a web browser, and navigate to `http://127.0.0.1:8080`. You should see the text `Hello world!` displayed on the page.

    So the web server is running and responding to requests. Now, we need to integrate an `index.html` file into the application.

    To stop the web server, type `Ctrl+C` in the terminal.

1. Update the `index.html` file to include a reference to the HTMX library. Add the following code to the `<head>` section of the `index.html` file.

    ```html
    <script defer src="https://unpkg.com/htmx.org@2.0.2"></script>
    ```

1. Replace the content of the `<body>` element in the `index.html` file with the following code.

    ```html
    <button hx-get="/hello-world" hx-swap="outerHTML">Hello world!</button>
    ```

1. The updated `index.html` file should look like this.

    ```html
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Hello World</title>
        <script defer src="https://unpkg.com/htmx.org@2.0.2"></script>
    </head>
    <body>
        <button hx-get="/hello-world" hx-swap="outerHTML">Hello world!</button>
    </body>
    </html>
    ```

1. Create a new file named `route.ts` in the `src` folder. Add the following code to the file.

    ```rust
    use actix_web::{get, HttpResponse, Responder};

    #[get("/hello-world")]
    pub async fn hello_world() -> impl Responder {
        HttpResponse::Ok().body("<h1>Hello world!</h1>")
    }
    ```

1. Update the `main.rs` file to include the `routes.rs` file as a module. Add the following code to the `main.rs` file.

    ```rust
    mod routes;

    use actix_files as fs;
    use actix_web::{App, HttpServer};

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .service(routes::hello_world)
                .service(fs::Files::new("/", "static").index_file("index.html"))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    }
    ```

1. From the terminal, run the project with the following command:

    ```bash
    cargo run
    ```

1. Open a web browser, and navigate to `http://127.0.0.1:8080`. You should see a button with the content `Hello world!` displayed on the page. Click the button. The content of the page should change to `Hello world!` inside of an `<h1>` element.
