use crate::{
    AppState, Claims,
    api::{CALENDAR_TAG, error::APIError},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use ogonek_db::core::calendar::cal::{delete, get_or_create, update};
use ogonek_types::{CalendarFull, CalendarUpdate};

/// Retrieves or creates the user's calendar
///
/// Returns the user's calendar, creating it if it doesn't exist.
#[utoipa::path(
    get,
    path = "",
    tag = CALENDAR_TAG,
    responses(
        (status = 200, description = "Calendar retrieved successfully", body = CalendarFull),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_calendar(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<CalendarFull>, APIError> {
    let calendar = get_or_create(&state.db, &claims.sub).await?;
    Ok(Json(calendar))
}

/// Deletes a specific calendar owned by the user
///
/// Removes the calendar if the user has ownership permissions.
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

/// Updates a specific calendar owned by the user
///
/// Modifies calendar properties if the user has ownership permissions.
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
