use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::calendar::calendar::{create, delete, find_all, find_by_id, update};
use crate::schema::AppState;
use crate::types::{Calendar, CalendarCreate, CalendarUpdate};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;

/// Get a single calendar by ID
#[utoipa::path(
    get,
    path = "/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Calendar ID")
    ),
    responses(
        (status = 200, description = "Calendar retrieved successfully", body = Calendar),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_calendar(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<Calendar>, APIError> {
    let calendar = find_by_id(&state.db, &id, &claims.sub).await?;
    Ok(Json(calendar))
}

/// Get all calendars for the authenticated user
#[utoipa::path(
    get,
    path = "",
    tag = CALENDAR_TAG,
    responses(
        (status = 200, description = "Calendars retrieved successfully", body = Vec<Calendar>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_calendars(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<Calendar>>, APIError> {
    let calendars = find_all(&state.db, &claims.sub).await?;
    Ok(Json(calendars))
}

/// Create a new calendar
#[utoipa::path(
    post,
    path = "",
    tag = CALENDAR_TAG,
    request_body = CalendarCreate,
    responses(
        (status = 201, description = "Calendar created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_calendar(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CalendarCreate>,
) -> Result<Json<String>, APIError> {
    let id = create(&state.db, &claims.sub, payload).await?;
    Ok(Json(id))
}

/// Delete a calendar
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Calendar ID")
    ),
    responses(
        (status = 204, description = "Calendar deleted successfully"),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_calendar(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    delete(&state.db, &id, &claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Update a calendar
#[utoipa::path(
    patch,
    path = "/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Calendar ID")
    ),
    request_body = CalendarUpdate,
    responses(
        (status = 204, description = "Calendar updated successfully"),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_calendar(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
    Json(payload): Json<CalendarUpdate>,
) -> Result<StatusCode, APIError> {
    update(&state.db, &id, &claims.sub, &payload).await?;
    Ok(StatusCode::NO_CONTENT)
}