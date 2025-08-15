use crate::api::STATE_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::flashcards::deck;
use crate::db::crud::core::{
    account,
    account::{preferences, student, user},
    flashcards, lesson, task,
};
use crate::db::crud::tracking::{activity, seen};
use crate::schema::AppState;
use crate::types::{AppContext, DashboardData, ModelType, NotificationBadges, PaginationParams};
use axum::extract::Json;
use axum::extract::State;

/// This data populates the dashboard/home view
#[utoipa::path(
    get,
    tag = STATE_TAG,
    path = "/dashboard",
    responses(
        (status = 200, description = "Dashboard data retrieved", body = DashboardData),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_dashboard(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<DashboardData>, APIError> {
    // Limit to three tasks
    let tasks = task::find_all(
        &state.db,
        &claims.sub,
        &PaginationParams {
            page: Some(1),
            per_page: Some(3),
            search: None,
            assignee: None,
        },
    )
    .await?;

    // Limit to three lessons
    let lessons = lesson::find_all(
        &state.db,
        &claims.sub,
        &PaginationParams {
            page: Some(1),
            per_page: Some(3),
            search: None,
            assignee: None,
        },
    )
    .await?;
    let decks = deck::find_all(
        &state.db,
        &claims.sub,
        &PaginationParams {
            page: Some(1),
            per_page: Some(3),
            search: None,
            assignee: None,
        },
    )
    .await?;
    let activity = activity::get_activity(&state.db, &claims.sub).await?;
    let learn_data = flashcards::learn::get_simple_stats(&state.db, &claims.sub).await?;

    Ok(Json(DashboardData {
        decks,
        lessons,
        tasks,
        activity,
        learn_data,
    }))
}

/// This data gives info about notification badges
#[utoipa::path(
    get,
    tag = STATE_TAG,
    path = "/badges",
    responses(
        (status = 200, description = "Badge data retrieved", body = NotificationBadges),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_badges(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<NotificationBadges>, APIError> {
    let unseen_tasks = seen::get_seen_badge(&state.db, &claims.sub, ModelType::Task).await?;
    let unseen_lessons = seen::get_seen_badge(&state.db, &claims.sub, ModelType::Lesson).await?;
    let unseen_decks = seen::get_seen_badge(&state.db, &claims.sub, ModelType::Deck).await?;
    let due_cards = flashcards::learn::fetch_due_count(&state.db, &claims.sub).await?;

    Ok(Json(NotificationBadges {
        unseen_decks,
        unseen_lessons,
        unseen_tasks,
        due_cards,
    }))
}
#[utoipa::path(
    get,
    tag = STATE_TAG,
    path = "/context",
    responses(
        (status = 200, description = "Context data retrieved", body = NotificationBadges),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_context(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<AppContext>, APIError> {
    let preferences = preferences::get_or_create_defaults(&state.db, &claims.sub).await?;
    let user = user::find_by_id(&state.db, &claims.sub).await?;
    let students = student::find_all(&state.db, &claims.sub).await?;
    let profile = account::profile::find_by_id(&state.db, &claims.sub).await?;
    let call_url = account::profile::get_call_url(&state.db, &claims.sub).await?;

    Ok(Json(AppContext {
        user,
        profile,
        students,
        preferences,
        call_url,
    }))
}
