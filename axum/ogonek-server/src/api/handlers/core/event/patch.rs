use crate::{
    AppState, Claims,
    api::{CALENDAR_TAG, error::APIError},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use ogonek_db::core::calendar::event::update;
use ogonek_types::{EventUpdate, EventUpdateRequest};

/// Update an event
#[utoipa::path(
    patch,
    path = "/events/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Event UID")
    ),
    request_body = EventUpdate,
    responses(
        (status = 204, description = "Event updated successfully"),
        (status = 404, description = "Event not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_event(
    State(state): State<AppState>,
    Path(id): Path<String>,
    _claims: Claims,
    Json(payload): Json<EventUpdateRequest>,
) -> Result<StatusCode, APIError> {
    update(&state.db, id, payload).await?;
    Ok(StatusCode::NO_CONTENT)
}
