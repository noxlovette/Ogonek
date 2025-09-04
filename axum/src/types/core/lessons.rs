use crate::types::{datetime_serialization, photos::Photo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

/// Mini-lesson
#[derive(Serialize, Debug, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LessonSmall {
    pub id: String,
    pub title: String,
    pub topic: String,
    pub assignee_name: String,
    pub seen: Option<bool>,
    #[serde(with = "datetime_serialization")]
    pub created_at: DateTime<Utc>,
}

/// Grown-up lesson
#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LessonFull {
    pub id: String,
    pub title: String,
    pub topic: String,
    pub markdown: String,
    pub assignee: String,
    #[serde(skip_serializing)]
    pub photo_id: Option<String>,
    #[serde(with = "datetime_serialization")]
    pub created_at: DateTime<Utc>,

    #[serde(with = "datetime_serialization")]
    pub updated_at: DateTime<Utc>,
    pub assignee_name: String,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct LessonWithPhoto {
    #[serde(flatten)]
    pub lesson: LessonFull,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Photo>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LessonCreate {
    pub title: String,
    pub topic: String,
    pub markdown: String,
    pub assignee: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LessonUpdate {
    pub id: Option<String>,
    pub title: Option<String>,
    pub topic: Option<String>,
    pub markdown: Option<String>,
    pub assignee: Option<String>,
    pub created_by: Option<String>,
    pub media_url: Option<String>,
}

// NOT IMPLEMENTED

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentNote {
    pub id: String,
    pub lesson_id: String,
    pub user_id: String,
    pub is_bookmarked: Option<bool>,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentNoteUpdate {
    pub notes: Option<String>,
    pub is_bookmarked: Option<bool>,
}
