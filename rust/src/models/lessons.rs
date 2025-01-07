use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LessonBody {
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
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LessonCreateBody {
    pub title: String,
    pub topic: String,
    pub markdown: String,
    pub assignee: String,
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
