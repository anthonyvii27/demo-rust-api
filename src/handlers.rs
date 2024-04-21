use crate::database::Database;
use crate::models::Category;
use crate::request::CreateUpdateBookRequest;

use actix_web::{web, Responder};
use std::sync::{Arc, Mutex};

pub async fn create_book(data: web::Data<Arc<Mutex<Database>>>, info: web::Json<CreateUpdateBookRequest>) -> impl Responder {
    let mut db = data.lock().unwrap();
    let new_book = db.create_book(info.name.clone(), info.author.clone(), info.number_of_pages, info.category.clone());
    db.save().unwrap();
    web::Json(new_book)
}

pub async fn get_book(data: web::Data<Arc<Mutex<Database>>>, id: web::Path<String>) -> impl Responder {
    let db = data.lock().unwrap();
    if let Some(book) = db.get_book(&id) {
        web::Json(book.clone())
    } else {
        web::Json(crate::models::Book {
            id: "".to_string(),
            name: "Book not found".to_string(),
            author: "".to_string(),
            number_of_pages: 0,
            category: Category::Classic,
            created_at: chrono::Utc::now(),
        })
    }
}

pub async fn update_book(data: web::Data<Arc<Mutex<Database>>>, id: web::Path<String>, info: web::Json<CreateUpdateBookRequest>) -> impl Responder {
    let mut db = data.lock().unwrap();
    if db.update_book(&id, info.name.clone(), info.author.clone(), info.number_of_pages, info.category.clone()) {
        db.save().unwrap();
        "Book updated".to_string()
    } else {
        "Book not found".to_string()
    }
}

pub async fn delete_book(data: web::Data<Arc<Mutex<Database>>>, id: web::Path<String>) -> impl Responder {
    let mut db = data.lock().unwrap();
    if db.delete_book(&id) {
        db.save().unwrap();
        "Book deleted".to_string()
    } else {
        "Book not found".to_string()
    }
}
