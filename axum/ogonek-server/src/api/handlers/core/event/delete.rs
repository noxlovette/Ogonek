use crate::{
    AppState,
    api::{CALENDAR_TAG, error::APIError},
    Claims,
};
use ogonek_db::core::calendar::event::delete;
use ogonek_types::EventDelete;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

/// Delete an event
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
