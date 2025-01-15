use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherStudent {
    pub teacher_id: String, // VARCHAR(21)
    pub student_id: String, // VARCHAR(21)
    pub status: String,
    pub markdown: Option<String>,
    #[serde_as(as = "Rfc3339")]
    pub joined: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct AddStudentRequest {
    pub student_id: String,
}


pub struct UpdateStudentRequest {
    pub student_id: String,
    pub markdown: Option<String>,
}