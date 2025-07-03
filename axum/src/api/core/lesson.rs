use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::{self, lesson};
use crate::models::meta::{CreationId, PaginatedResponse};
use crate::models::{LessonCreate, LessonFull, LessonSmall, LessonUpdate, PaginationParams};
use crate::schema::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::extract::{Json, Query};
use hyper::StatusCode;

pub async fn fetch_recent_lessons(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<LessonSmall>>, APIError> {
    let lessons = lesson::find_recent(&state.db, &claims.sub).await?;

    Ok(Json(lessons))
}

pub async fn fetch_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<LessonFull>, APIError> {
    let lesson = lesson::find_by_id(&state.db, &id, &claims.sub).await?;
    core::seen::mark_as_seen(&state.db, &claims.sub, &id, core::seen::ModelType::Lesson).await?;
    Ok(Json(lesson))
}
pub async fn list_lessons(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
    claims: Claims,
) -> Result<Json<PaginatedResponse<LessonSmall>>, APIError> {
    let lessons = lesson::find_all(&state.db, &claims.sub, &params).await?;
    let count = lesson::count(&state.db, &claims.sub).await?;

    Ok(Json(PaginatedResponse {
        data: lessons,
        total: count,
        page: params.page(),
        per_page: params.limit(),
    }))
}

pub async fn create_lesson(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<LessonCreate>,
) -> Result<Json<CreationId>, APIError> {
    let id = lesson::create(&state.db, &claims.sub, payload).await?;

    Ok(Json(id))
}

pub async fn delete_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    lesson::delete(&state.db, &id, &claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
    Json(payload): Json<LessonUpdate>,
) -> Result<StatusCode, APIError> {
    // fetch assignee before update
    let current_assignee = lesson::find_assignee(&state.db, &id, &claims.sub).await?;

    // update the lesson
    lesson::update(&state.db, &id, &claims.sub, &payload).await?;

    // check if assignee changed
    let new_assignee = payload.assignee.clone();

    if new_assignee != current_assignee {
        // remove seen record for old assignee
        if let Some(old_user) = current_assignee {
            core::seen::delete_seen(&state.db, &old_user, &id, core::seen::ModelType::Lesson)
                .await?;
        }

        // insert unseen for new assignee
        if let Some(new_user) = new_assignee {
            core::seen::insert_as_unseen(&state.db, &new_user, &id, core::seen::ModelType::Lesson)
                .await?;
        }
    }

    Ok(StatusCode::NO_CONTENT)
}
