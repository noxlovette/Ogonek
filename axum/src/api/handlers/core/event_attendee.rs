use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::calendar::event_attendee::{delete, update};
use crate::schema::AppState;
use crate::types::EventAttendeeUpdate;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
// Delete an event attendee
#[utoipa::path(
    delete,
    path = "/attendees/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Attendee ID")
    ),
    responses(
        (status = 204, description = "Attendee deleted successfully"),
        (status = 404, description = "Attendee not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_attendee(
    State(state): State<AppState>,
    Path(id): Path<String>,
    _claims: Claims,
) -> Result<StatusCode, APIError> {
    delete(&state.db, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Update an event attendee
#[utoipa::path(
    patch,
    path = "/attendees/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Attendee ID")
    ),
    request_body = EventAttendeeUpdate,
    responses(
        (status = 204, description = "Attendee updated successfully"),
        (status = 404, description = "Attendee not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_attendee(
    State(state): State<AppState>,
    Path(id): Path<String>,
    _claims: Claims,
    Json(payload): Json<EventAttendeeUpdate>,
) -> Result<StatusCode, APIError> {
    update(&state.db, &id, &payload).await?;
    Ok(StatusCode::NO_CONTENT)
}
