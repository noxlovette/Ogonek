use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherStudent {
    pub teacher_id: String, 
    pub student_id: String, 
    pub status: String,
    pub telegram_id: Option<String>,
    pub markdown: Option<String>,
    #[serde_as(as = "Rfc3339")]
    pub joined: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    pub markdown: Option<String>,
    pub telegram_id: Option<String>,
}

#[derive(Deserialize)]
pub struct AddStudentRequest {
    pub student_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct UpdateStudentRequest {
    pub markdown: String,
    pub telegram_id: String
}