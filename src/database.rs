use crate::models::{Book, Category};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Database {
    pub books: Vec<Book>,
    pub file_path: String,
}

impl Database {
    pub fn new(file_path: &str) -> Self {
        let mut books = Vec::new();
        let path = Path::new(file_path);

        if path.exists() {
            let file = File::open(path).expect("Failed to open file.");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(book) = serde_json::from_str::<Book>(&line.unwrap()) {
                    books.push(book)
                }
            }
        }

        Database {
            books,
            file_path: file_path.to_string(),
        }
    }

    pub fn save(&self) -> io::Result<()> {
        println!("Saving to file: {}", self.file_path);
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.file_path)?;

        for book in &self.books {
            let book_json = serde_json::to_string(book).unwrap();
            writeln!(file, "{}", book_json)?;
        }
        Ok(())
    }

    pub fn create_book(&mut self, name: String, author: String, number_of_pages: i32, category: Category) -> Book {
        let new_book = Book::new(name, author, number_of_pages, category);
        self.books.push(new_book.clone());
        new_book
    }

    pub fn get_book(&self, id: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.id == id)
    }

    pub fn update_book(&mut self, id: &str, name: String, author: String, number_of_pages: i32, category: Category) -> bool {
        if let Some(book) = self.books.iter_mut().find(|book| book.id == id) {
            book.name = name;
            book.author = author;
            book.number_of_pages = number_of_pages;
            book.category = category;
            return true;
        }
        false
    }

    pub fn delete_book(&mut self, id: &str) -> bool {
        if let Some(index) = self.books.iter().position(|book| book.id == id) {
            self.books.remove(index);
            return true;
        }
        false
    }
}
