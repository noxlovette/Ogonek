use crate::{
    AppState, Claims,
    api::{CALENDAR_TAG, error::APIError},
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use ogonek_db::core::calendar::event::create;
use ogonek_types::EventCreate;
/// Creates a new calendar event for the authenticated user
///
/// Creates an event with automatic end time if not provided (defaults to 1 hour duration).
#[utoipa::path(
    post,
    path = "/events",
    tag = CALENDAR_TAG,
    request_body = EventCreate,
    responses(
        (status = 201, description = "Event created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_event(
    State(state): State<AppState>,
    claims: Claims,
    Json(mut payload): Json<EventCreate>,
) -> Result<StatusCode, APIError> {
    if payload.dtend_time.is_none() {
        payload.dtend_time = Some(payload.dtstart_time + chrono::Duration::hours(1));
    }

    create(&state.db, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
