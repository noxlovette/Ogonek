use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::calendar::event_attendee::{create, delete, find_by_event_id, find_by_id, update};
use crate::schema::AppState;
use crate::types::{EventAttendee, EventAttendeeCreate, EventAttendeeUpdate};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;

/// Get a single event attendee by ID
#[utoipa::path(
    get,
    path = "/attendees/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Attendee ID")
    ),
    responses(
        (status = 200, description = "Attendee retrieved successfully", body = EventAttendee),
        (status = 404, description = "Attendee not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_attendee(
    State(state): State<AppState>,
    Path(id): Path<String>,
    _claims: Claims,
) -> Result<Json<EventAttendee>, APIError> {
    let attendee = find_by_id(&state.db, &id).await?;
    Ok(Json(attendee))
}

/// Get all attendees for an event
#[utoipa::path(
    get,
    path = "/events/{event_id}/attendees",
    tag = CALENDAR_TAG,
    params(
        ("event_id" = String, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Attendees retrieved successfully", body = Vec<EventAttendee>),
        (status = 404, description = "Event not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_attendees(
    State(state): State<AppState>,
    Path(event_id): Path<String>,
    _claims: Claims,
) -> Result<Json<Vec<EventAttendee>>, APIError> {
    let attendees = find_by_event_id(&state.db, &event_id).await?;
    Ok(Json(attendees))
}

/// Create a new event attendee
#[utoipa::path(
    post,
    path = "/events/{event_id}/attendees",
    tag = CALENDAR_TAG,
    params(
        ("event_id" = String, Path, description = "Event ID")
    ),
    request_body = EventAttendeeCreate,
    responses(
        (status = 201, description = "Attendee created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_attendee(
    State(state): State<AppState>,
    Path(event_id): Path<String>,
    _claims: Claims,
    Json(payload): Json<EventAttendeeCreate>,
) -> Result<Json<String>, APIError> {
    let id = create(&state.db, &event_id, payload).await?;
    Ok(Json(id))
}

/// Delete an event attendee
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