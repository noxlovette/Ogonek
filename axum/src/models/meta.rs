use serde::Serialize;

use crate::models::{LessonSmall, ProfileWithTS, Student, TaskSmall, User};

/// Generic response that stores paginated data
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Simply contains one string, the created ID
#[derive(Debug, Serialize)]
pub struct CreationId {
    pub id: String,
}

/// A big response that powers the dashboard view
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DashboardData {
    pub students: Vec<Student>,
    pub lessons: Vec<LessonSmall>,
    pub tasks: Vec<TaskSmall>,
    pub user: User,
    pub profile: ProfileWithTS,
}
