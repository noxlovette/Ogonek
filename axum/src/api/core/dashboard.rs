use crate::api::USER_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::account::{preferences, student, user};
use crate::db::crud::tracking::{ModelType, activity, seen};
use crate::db::crud::{account, core, flashcards};
use crate::models::{BadgeWrapper, DashboardData, LearnDataDashboard};
use crate::schema::AppState;
use axum::extract::Json;
use axum::extract::State;

/// This data populates the sidebar with badges,
/// the dashboard view with data,
/// and the stores for profile and user
#[utoipa::path(
    get, tag = USER_TAG,
    path = "/dashboard",
    responses(
        (status = 200, description = "Dashboard data retrieved", body=DashboardData),
        (status = 401, description = "Unauthorized")
    ),
    security(("api_key" = []))
)]
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

    let task_count = seen::get_seen_badge(&state.db, &claims.sub, ModelType::Task).await?;
    let lesson_count = seen::get_seen_badge(&state.db, &claims.sub, ModelType::Lesson).await?;
    let decks = flashcards::deck::find_recent(&state.db, &claims.sub).await?;
    let deck_count = seen::get_seen_badge(&state.db, &claims.sub, ModelType::Deck).await?;
    let activity = activity::get_activity(&state.db, &claims.sub).await?;
    let due_cards = flashcards::learn::fetch_due_count(&state.db, &claims.sub).await?;
    let stats = flashcards::learn::get_simple_stats(&state.db, &claims.sub).await?;
    let preferences = preferences::get_or_create_defaults(&state.db, &claims.sub).await?;

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
        decks: BadgeWrapper {
            count: deck_count,
            data: decks,
        },
        user,
        profile,
        activity,
        learn: LearnDataDashboard { due_cards, stats },
        preferences,
    }))
}
