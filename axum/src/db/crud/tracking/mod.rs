use serde::Serialize;
use time::OffsetDateTime;

pub mod activity;
pub mod seen;

/// This guy is a safeguard against arbitrary values in the database
/// (as I have decided not to use an enum in postgres)
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "text")]
#[sqlx(rename_all = "lowercase")]
pub enum ModelType {
    Lesson,
    Task,
    Deck,
    Word,
}

impl ModelType {
    fn as_str(&self) -> &'static str {
        match self {
            ModelType::Lesson => "lesson",
            ModelType::Task => "task",
            ModelType::Deck => "deck",
            ModelType::Word => "word",
        }
    }
}

/// Safeguard for Action Types
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "text")]
#[sqlx(rename_all = "lowercase")]
pub enum ActionType {
    Delete,
    Create,
    Update,
    Complete,
}

impl ActionType {
    fn as_str(&self) -> &'static str {
        match self {
            ActionType::Delete => "deleted",
            ActionType::Create => "created",
            ActionType::Update => "updated",
            ActionType::Complete => "completed",
        }
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLog {
    pub model_type: String,
    pub model_id: String,
    pub action: String,
    pub created_at: Option<OffsetDateTime>,
}
