use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

/// Pagination for lessons
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub assignee: Option<String>,
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

/// Mini-lesson
#[serde_with::serde_as]
#[derive(Serialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct LessonSmall {
    pub id: String,
    pub title: String,
    pub topic: String,
    pub assignee_name: String,
    pub seen: Option<bool>,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
}

/// Grown-up lesson
#[serde_with::serde_as]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LessonFull {
    pub id: String,
    pub title: String,
    pub topic: String,
    pub markdown: String,
    pub assignee: String,
    pub created_by: String,
    #[serde_as(as = "Rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde_as(as = "Rfc3339")]
    pub updated_at: OffsetDateTime,
    pub assignee_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LessonCreate {
    pub title: String,
    pub topic: String,
    pub markdown: String,
    pub assignee: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LessonUpdate {
    pub id: Option<String>,
    pub title: Option<String>,
    pub topic: Option<String>,
    pub markdown: Option<String>,
    pub assignee: Option<String>,
    pub created_by: Option<String>,
}

// NOT IMPLEMENTED
#[serde_with::serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentNote {
    pub id: String,
    pub lesson_id: String,
    pub user_id: String,
    pub is_bookmarked: Option<bool>,
    pub notes: Option<String>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub created_at: Option<OffsetDateTime>,
    #[serde_as(as = "Option<Rfc3339>")]
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentNoteUpdate {
    pub notes: Option<String>,
    pub is_bookmarked: Option<bool>,
}
