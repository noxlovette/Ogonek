use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::lesson;
use crate::models::meta::{CreationId, PaginatedResponse};
use crate::models::{
    LessonCreate, LessonSmall, LessonSmallWithStudent, LessonUpdate, LessonWithStudent,
    PaginationParams,
};
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
) -> Result<Json<LessonWithStudent>, APIError> {
    let lesson = lesson::find_by_id(&state.db, &id, &claims.sub).await?;
    Ok(Json(lesson))
}
pub async fn list_lessons(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
    claims: Claims,
) -> Result<Json<PaginatedResponse<LessonSmallWithStudent>>, APIError> {
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
    lesson::update(&state.db, &id, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
