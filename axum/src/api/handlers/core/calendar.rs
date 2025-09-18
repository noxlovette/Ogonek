use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::calendar::calendar::{delete, get_or_create, update};
use crate::schema::AppState;
use crate::types::{Calendar, CalendarUpdate};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;

/// Get the user's calendar
#[utoipa::path(
    get,
    path = "",
    tag = CALENDAR_TAG,
    responses(
        (status = 200, description = "Calendar retrieved successfully", body = Calendar),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_calendar(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Calendar>, APIError> {
    let calendar = get_or_create(&state.db, &claims.sub).await?;
    Ok(Json(calendar))
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
