use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::{SortField, SortOrder, TaskSmall};

use super::{default_page, default_per_page};
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct TaskPaginationParams {
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: u32,

    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_per_page")]
    pub per_page: u32,

    #[serde(default)]
    pub search: Option<String>,

    #[serde(default)]
    pub sort_by: SortField,

    #[serde(default)]
    pub sort_order: SortOrder,

    #[serde(default)]
    pub completed: Option<bool>,

    #[serde(default)]
    pub assignee: Option<String>,
}

// Explicit structs for OpenAPI
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedTasks {
    pub data: Vec<TaskSmall>,
    pub page: i64,
    pub per_page: i64,
}

impl TaskPaginationParams {
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64 // Cast when needed
    }

    pub fn limit(&self) -> i64 {
        self.per_page as i64 // Cast when needed
    }

    pub fn page(&self) -> i64 {
        self.page as i64
    }
}
