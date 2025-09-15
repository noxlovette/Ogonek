use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::calendar::event::{
    create, delete, find_by_calendar_id, find_by_date, find_by_uid, update,
};
use crate::db::crud::core::calendar::event_attendee::find_by_event_id;
use crate::schema::AppState;
use crate::types::{CalendarEvent, CalendarEventCreate, CalendarEventUpdate, EventWithAttendees};
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;

/// Get a single event by UID
#[utoipa::path(
    get,
    path = "/events/{uid}",
    tag = CALENDAR_TAG,
    params(
        ("uid" = String, Path, description = "Event UID")
    ),
    responses(
        (status = 200, description = "Event retrieved successfully", body = EventWithAttendees),
        (status = 404, description = "Event not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_event(
    State(state): State<AppState>,
    Path(id): Path<String>,
    _claims: Claims,
) -> Result<Json<EventWithAttendees>, APIError> {
    let event = find_by_uid(&state.db, &id).await?;
    let attendees = find_by_event_id(&state.db, &id).await?;
    Ok(Json(EventWithAttendees { event, attendees }))
}

/// Get all events for a calendar
#[utoipa::path(
    get,
    path = "/calendars/{calendar_id}/events",
    tag = CALENDAR_TAG,
    params(
        ("calendar_id" = String, Path, description = "Calendar ID")
    ),
    responses(
        (status = 200, description = "Events retrieved successfully", body = Vec<CalendarEvent>),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_events(
    State(state): State<AppState>,
    Path(calendar_id): Path<String>,
    _claims: Claims,
) -> Result<Json<Vec<CalendarEvent>>, APIError> {
    let events = find_by_calendar_id(&state.db, &calendar_id).await?;
    Ok(Json(events))
}

/// Get all events for a given day
#[utoipa::path(
    get,
    path = "/calendars/events/{day}",
    tag = CALENDAR_TAG,
    params(
        ("day" = String, Path, description = "Day")
    ),
    responses(
        (status = 200, description = "Events retrieved successfully", body = Vec<CalendarEvent>),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_events_day(
    State(state): State<AppState>,
    Path(day): Path<String>,
    _claims: Claims,
) -> Result<Json<Vec<CalendarEvent>>, APIError> {
    let naive_date = chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d")
        .map_err(|_| APIError::BadRequest("Invalid Date Format".to_string()))?;
    let events = find_by_date(&state.db, naive_date).await?;
    Ok(Json(events))
}

/// Create a new event
#[utoipa::path(
    post,
    path = "/calendars/{calendar_id}/events",
    tag = CALENDAR_TAG,
    params(
        ("calendar_id" = String, Path, description = "Calendar ID")
    ),
    request_body = CalendarEventCreate,
    responses(
        (status = 201, description = "Event created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn create_event(
    State(state): State<AppState>,
    Path(calendar_id): Path<String>,
    _claims: Claims,
    Json(payload): Json<CalendarEventCreate>,
) -> Result<Json<String>, APIError> {
    let uid = nanoid::nanoid!();
    let id = create(&state.db, &uid, &calendar_id, payload).await?;
    Ok(Json(id))
}

/// Delete an event
#[utoipa::path(
    delete,
    path = "/events/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Event ID")
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
) -> Result<StatusCode, APIError> {
    delete(&state.db, &id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Update an event
#[utoipa::path(
    patch,
    path = "/events/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    request_body = CalendarEventUpdate,
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
    Json(payload): Json<CalendarEventUpdate>,
) -> Result<StatusCode, APIError> {
    update(&state.db, &id, &payload).await?;
    Ok(StatusCode::NO_CONTENT)
}
