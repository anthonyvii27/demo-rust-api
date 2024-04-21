use actix_web::{web, App, HttpServer};
use std::env;
use std::sync::{Arc, Mutex};

mod models;
mod database;
mod handlers;
mod request;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let file_path = current_dir.join("books.json");
    let db = Arc::new(Mutex::new(database::Database::new(file_path.to_str().unwrap())));

    println!("Server is running on: 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/books", web::post().to(handlers::create_book))
            .route("/books/{id}", web::get().to(handlers::get_book))
            .route("/books/{id}", web::put().to(handlers::update_book))
            .route("/books/{id}", web::delete().to(handlers::delete_book))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

