use serde::{Deserialize, Serialize};
use crate::models::lessons::Bookmark;
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: RecordId,
    pub name: String,
    pub email: String,
    pub joined_on: String,
    pub pass: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Teacher {
    pub person: Person,
    pub subject: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub person: Person,
    pub course: String,
    pub bookmarks: Vec<Bookmark>,
}
