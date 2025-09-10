use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Content {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub meta_description: Option<String>,
    pub version: i32,
    pub status: ContentStatus,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub updated_by: String,
}

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ContentStatus {
    Draft,
    Published,
}
