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
pub struct LessonCreateResponse {
    pub id: String,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LessonBodyWithStudent {
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
    pub assignee_name: String, // New field for the joined name
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LessonCreateBody {
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

// What the client can send (payload structure)
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentNoteUpdate {
    pub notes: Option<String>,
    pub is_bookmarked: Option<bool>,
}
