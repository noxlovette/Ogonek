use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub teacher: String,
    pub student: String,
    pub subject: String,
    pub topic: String,
    pub category: String,
    pub date: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub lesson: String,
    pub timestamp: String,
    pub notes: Option<String>,
}