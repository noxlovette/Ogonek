use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::account::{student, user};
use crate::db::crud::core::seen::ModelType;
use crate::db::crud::{account, core};
use crate::models::{BadgeWrapper, DashboardData};
use crate::schema::AppState;
use axum::extract::Json;
use axum::extract::State;

/// This data populates the sidebar with badges,
/// the dashboard view with data,
/// and the stores for profile and user
pub async fn fetch_dashboard(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<DashboardData>, APIError> {
    let user = user::find_by_id(&state.db, &claims.sub).await?;
    let students = student::find_all(&state.db, &claims.sub).await?;
    let tasks = core::task::find_recent(&state.db, &claims.sub).await?;
    let lessons = core::lesson::find_recent(&state.db, &claims.sub).await?;
    let profile =
        account::profile::find_by_id(&state.db, &claims.sub, user.role == "student").await?;

    let task_count = core::seen::get_seen_badge(&state.db, &claims.sub, ModelType::Task).await?;
    let lesson_count =
        core::seen::get_seen_badge(&state.db, &claims.sub, ModelType::Lesson).await?;
    let deck_count = core::seen::get_seen_badge(&state.db, &claims.sub, ModelType::Deck).await?;

    Ok(Json(DashboardData {
        students,
        lessons: BadgeWrapper {
            count: lesson_count,
            data: lessons,
        },
        tasks: BadgeWrapper {
            count: task_count,
            data: tasks,
        },
        deck_count,
        user,
        profile,
    }))
}
