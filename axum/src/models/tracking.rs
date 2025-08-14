use crate::models::datetime_serialization;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

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
    pub fn as_str(&self) -> &'static str {
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
    Subscribe,
    Unsubscribe,
}

impl ActionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionType::Delete => "deleted",
            ActionType::Create => "new",
            ActionType::Update => "updated",
            ActionType::Complete => "completed",
            ActionType::Subscribe => "subscribed",
            ActionType::Unsubscribe => "unsubscribed",
        }
    }
}

#[derive(ToSchema, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLog {
    pub model_type: String,
    pub model_id: String,
    pub action: String,
    #[serde(with = "datetime_serialization::option")]
    pub created_at: Option<DateTime<Utc>>,
}
