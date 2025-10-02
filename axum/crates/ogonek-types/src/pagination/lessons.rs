use crate::{LessonSmall, SortField, SortOrder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use super::{default_page, default_per_page};

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct LessonPaginationParams {
    #[serde(default)]
    pub topic: Option<String>,

    #[serde(default)]
    pub assignee: Option<String>,

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
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedLessons {
    pub data: Vec<LessonSmall>,
    pub page: i64,
    pub per_page: i64,
}

impl LessonPaginationParams {
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
