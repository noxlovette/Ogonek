use super::files::FileSmall;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct TaskPaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub priority: Option<i32>,
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

#[derive(Serialize, ToSchema, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaskFull {
    pub id: String,
    pub title: String,
    pub markdown: String,
    #[validate(range(min = 1, max = 3))]
    pub priority: i16,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub created_by: String,
    pub assignee: String,
    pub assignee_name: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TaskSmall {
    pub id: String,
    pub title: String,
    pub priority: i16,
    pub completed: bool,
    pub assignee_name: String,
    pub seen: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreate {
    pub title: String,
    pub markdown: String,
    pub priority: Option<i16>,
    pub due_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub markdown: Option<String>,
    pub priority: Option<i16>,
    pub completed: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskFileBind {
    pub file_ids: Vec<String>,
}

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct TaskWithFilesResponse {
    pub task: TaskFull,
    pub files: Vec<FileSmall>,
}
