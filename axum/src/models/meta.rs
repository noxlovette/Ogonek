use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    db::crud::tracking::ActivityLog,
    models::{
        DeckSmall, LearnDataDashboard, LessonSmall, ProfileWithTS, Student, TaskSmall, User,
        UserPreferences,
    },
};

/// Generic response that stores paginated data
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Generic wrapper with the badge count in it
#[derive(Serialize, ToSchema)]
pub struct BadgeWrapper<T> {
    pub data: Vec<T>,
    pub count: i64,
}

/// Simply contains one string, the created ID
#[derive(Serialize, ToSchema)]
pub struct CreationId {
    pub id: String,
}

/// A big response that powers the dashboard view
#[derive(Serialize, ToSchema)]
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
    pub preferences: UserPreferences,
}
