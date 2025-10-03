use crate::{EventSmall, LessonSmall, Profile, Student, TaskSmall, User, UserPreferences};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppContext {
    pub user: User,
    pub profile: Profile,
    pub call_url: Option<String>,
    pub students: Vec<Student>,
    pub preferences: UserPreferences,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardData {
    pub tasks: Vec<TaskSmall>,
    pub events: Vec<EventSmall>,
    pub lessons: Vec<LessonSmall>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NotificationBadges {
    pub unseen_tasks: i64,
    pub unseen_lessons: i64,
    pub unseen_decks: i64,
    pub due_cards: Option<i64>,
}
