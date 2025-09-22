use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::crud::core::calendar::event::read_one;
use crate::db::crud::core::calendar::event::read_all;
use crate::db::crud::core::calendar::event_attendee::find_by_event_id;
use crate::schema::AppState;

use crate::types::{CalendarQuery, EventSmall, EventWithAttendees};
use axum::extract::{Json, Path, Query, State};

/// Get a single event by UID
#[utoipa::path(
    get,
    path = "/events/{id}",
    tag = CALENDAR_TAG,
    params(
        ("id" = String, Path, description = "Event UID")
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
    let event = read_one(&state.db, id.clone()).await?;
    let attendees = find_by_event_id(&state.db, &id).await?;
    Ok(Json(EventWithAttendees { event, attendees }))
}
/// Get events for a specific month
#[utoipa::path(
    get,
    path = "/events",
    tag = CALENDAR_TAG,
    params(
        ("start" = String, Query),
        ("end" = String, Query)
    ),
    responses(
        (status = 200, description = "Events retrieved successfully", body = Vec<EventSmall>),
        (status = 400, description = "Invalid year or month"),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_events(
    Query(query): Query<CalendarQuery>,
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<EventSmall>>, APIError> {
    let calendar_events = read_all(&state.db, &claims.sub, query.start, query.end).await?;
    Ok(Json(calendar_events))
}
