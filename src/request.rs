use serde::{Deserialize, Serialize};
use crate::models::Category;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUpdateBookRequest {
    pub name: String,
    pub author: String,
    pub number_of_pages: i32,
    pub category: Category,
}