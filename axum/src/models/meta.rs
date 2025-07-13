use serde::Serialize;

use crate::{
    db::crud::tracking::ActivityLog,
    models::{DeckSmall, LearnDataDashboard, LessonSmall, ProfileWithTS, Student, TaskSmall, User},
};

/// Generic response that stores paginated data
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Generic wrapper with the badge count in it
#[derive(Debug, Serialize)]
pub struct BadgeWrapper<T> {
    pub data: Vec<T>,
    pub count: i64,
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
    pub lessons: BadgeWrapper<LessonSmall>,
    pub tasks: BadgeWrapper<TaskSmall>,
    pub decks: BadgeWrapper<DeckSmall>,
    pub user: User,
    pub profile: ProfileWithTS,
    pub activity: Vec<ActivityLog>,
    pub learn: LearnDataDashboard,
}
