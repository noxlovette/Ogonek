use super::files::FileSmall;
use crate::{Visibility, datetime_serialization};
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
    pub visibility: Visibility,
    pub assignee: Option<String>,
    pub assignee_name: Option<String>,
}

#[derive(Serialize, Debug, FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskSmall {
    pub id: String,
    pub title: String,
    pub priority: i16,
    pub completed: bool,
    pub visibility: Visibility,
    pub assignee_name: Option<String>,
    pub seen: Option<bool>,
    #[serde(with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskCreate {
    pub title: String,
    pub markdown: String,
    #[serde(with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
}

#[derive(Deserialize, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskUpdate {
    pub title: Option<String>,
    pub markdown: Option<String>,
    #[serde(default, with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
    pub visibility: Option<Visibility>,
    pub unassign: Option<bool>,
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

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskPublic {
    pub id: String,
    pub title: String,
    pub markdown: String,
    pub completed: bool,
    #[serde(with = "datetime_serialization::option")]
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskPublicWithFiles {
    #[serde(flatten)]
    pub task: TaskPublic,
    pub files: Vec<FileSmall>,
}
