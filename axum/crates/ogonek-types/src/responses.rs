use super::{DeckSmall, LessonSmall, TaskSmall};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
/// Generic response that stores paginated data
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i64,
    pub per_page: i64,
}

// Explicit structs for OpenAPI
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedTasks {
    pub data: Vec<TaskSmall>,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedLessons {
    pub data: Vec<LessonSmall>,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedDecks {
    pub data: Vec<DeckSmall>,
    pub page: i64,
    pub per_page: i64,
}

/// Pagination
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub assignee: Option<String>,
}
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct TaskPaginationParams {
    #[validate(range(min = 1))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub completed: Option<bool>,
    pub assignee: Option<String>,
}

impl TaskPaginationParams {
    pub fn limit(&self) -> i64 {
        self.per_page.unwrap_or(50).clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.limit()
    }

    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
}

impl PaginationParams {
    pub fn limit(&self) -> i64 {
        self.per_page.unwrap_or(50).clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.limit()
    }

    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
}
