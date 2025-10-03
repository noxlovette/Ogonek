mod decks;
mod lessons;
mod tasks;

pub use decks::*;
pub use lessons::*;
use serde::{Deserialize, Serialize};
pub use tasks::*;
use utoipa::ToSchema;
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub count: i64,
    pub total_pages: i64,
    pub per_page: i64,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    #[default]
    CreatedAt,
    UpdatedAt,
    Title,
    DueDate,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    #[default]
    Desc,
}

impl SortOrder {
    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}
impl SortField {
    /// Returns the SQL column name for tasks
    pub fn to_task_column(&self) -> &'static str {
        match self {
            Self::CreatedAt => "t.created_at",
            Self::UpdatedAt => "t.updated_at",
            Self::Title => "t.title",
            Self::DueDate => "t.due_date",
        }
    }

    pub fn to_lesson_column(&self) -> &'static str {
        match self {
            Self::CreatedAt => "l.created_at",
            Self::UpdatedAt => "l.updated_at",
            Self::Title => "l.title",
            Self::DueDate => "l.created_at",
        }
    }

    pub fn to_deck_column(&self) -> &'static str {
        match self {
            Self::CreatedAt => "d.created_at",
            Self::UpdatedAt => "d.updated_at",
            Self::Title => "d.title",
            Self::DueDate => "d.created_at",
        }
    }
}

pub fn default_page() -> u32 {
    1
}
pub fn default_per_page() -> u32 {
    20
}
