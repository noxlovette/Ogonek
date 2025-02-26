use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::prelude::FromRow;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

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
        self.per_page.unwrap_or(50).min(100).max(1)
    }
    
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.limit()
    }
    
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskBody {
    pub id: String,
    pub title: String,
    pub markdown: String,
    pub priority: i16,
    pub completed: bool,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde_as(as = "Option<Rfc3339>")]
    pub due_date: Option<OffsetDateTime>,
    pub file_path: Option<String>,
    pub created_by: String,
    pub assignee: String,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TaskBodyWithStudent {
    pub id: String,
    pub title: String,
    pub markdown: String,
    pub priority: i16,
    pub completed: bool,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde_as(as = "Option<Rfc3339>")]
    pub due_date: Option<OffsetDateTime>,
    pub file_path: Option<String>,
    pub created_by: String,
    pub assignee: String,
    pub assignee_name: String, // or whatever new field you need
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreateBody {
    pub title: String,
    pub markdown: String,
    pub priority: Option<i16>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub due_date: Option<OffsetDateTime>,
    pub assignee: Option<String>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub markdown: Option<String>,
    pub priority: Option<i16>,
    pub completed: Option<bool>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub due_date: Option<OffsetDateTime>,
    pub assignee: Option<String>,
    pub file_path: Option<String>,
}
