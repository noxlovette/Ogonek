use super::files::FileSmall;
use crate::types::datetime_serialization;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, ToSchema, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaskFull {
    pub id: String,
    pub title: String,
    pub markdown: String,
    #[validate(range(min = 1, max = 3))]
    pub priority: i16,
    pub completed: bool,
    #[serde(with = "datetime_serialization")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "datetime_serialization")]
    pub updated_at: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
    pub created_by: String,
    pub assignee: String,
    pub assignee_name: String,
}

#[derive(Serialize, Debug, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskSmall {
    pub id: String,
    pub title: String,
    pub priority: i16,
    pub completed: bool,
    pub assignee_name: String,
    pub seen: Option<bool>,
    #[serde(with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreate {
    pub title: String,
    pub markdown: String,
    pub priority: Option<i16>,
    #[serde(with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub markdown: Option<String>,
    pub completed: Option<bool>,
    #[serde(with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskFileBind {
    pub file_ids: Vec<String>,
}

#[derive(ToSchema, Serialize, Debug)]
pub struct TaskWithFilesResponse {
    pub task: TaskFull,
    pub files: Vec<FileSmall>,
}
