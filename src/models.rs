use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Category {
    CrimeStory,
    Horror,
    Classic,
    LoveStory,
    ScienceFiction,
    Fantasy,
    FairyTale,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub number_of_pages: i32,
    pub category: Category,
    pub created_at: DateTime<Utc>,
}

impl Book {
    pub fn new(name: String, author: String, number_of_pages: i32, category: Category) -> Self {
        Book {
            id: Uuid::new_v4().to_string(),
            name,
            author,
            number_of_pages,
            category,
            created_at: Utc::now(),
        }
    }
}
