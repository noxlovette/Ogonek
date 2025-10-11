use crate::{
    AppState, Claims,
    api::{CALENDAR_TAG, error::APIError},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use ogonek_db::core::calendar::event::delete;
use ogonek_types::EventDelete;

/// Deletes a calendar event
///
/// Removes an event based on the provided deletion parameters.
#[utoipa::path(
    delete,
    path = "/events/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Event UID")
    ),
    responses(
        (status = 204, description = "Event deleted successfully"),
        (status = 404, description = "Event not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_event(
    State(state): State<AppState>,
    Path(id): Path<String>,
    _claims: Claims,
    Json(payload): Json<EventDelete>,
) -> Result<StatusCode, APIError> {
    delete(&state.db, id, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
