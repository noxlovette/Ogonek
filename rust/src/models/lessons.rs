use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Debug)]
pub struct LessonTime {
    pub custom_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LessonBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<RecordId>,
    pub title: String,
    pub topic: String,
    pub markdown: String,
    pub assignee: RecordId,
    pub created_by: RecordId,
    pub time: Option<LessonTime>,
}
