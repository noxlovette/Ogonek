use crate::{
    AppState, Claims,
    api::{CALENDAR_TAG, error::APIError},
    services::calendar::extract_id_and_occurence,
};
use ogonek_db::core::calendar::{
    event::{read_all, read_one},
    event_attendee::find_by_event_id,
};

use axum::extract::{Json, Path, Query, State};
use ogonek_types::{CalendarQuery, CalendarRole, EventSmall, EventWithAttendees};

/// Retrieves a single event by its unique identifier
///
/// Returns the event details along with attendee information.
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

    // Extract master event ID for attendee lookup (virtual events use master's attendees)
    let (master_id, _) = extract_id_and_occurence(id);
    let attendees = find_by_event_id(&state.db, &master_id).await?;

    Ok(Json(EventWithAttendees { event, attendees }))
}
/// Lists events within a specified date range
///
/// Retrieves all events for the user within the given start and end dates, optionally filtered by role.
#[utoipa::path(
    get,
    path = "/events",
    tag = CALENDAR_TAG,
    params(
        ("start" = String, Query),
        ("end" = String, Query),
        ("role" = Option<CalendarRole>, Query)
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
    let calendar_events = read_all(
        &state.db,
        &claims.sub,
        query.start,
        query.end,
        query.role.unwrap_or_default(),
    )
    .await?;
    Ok(Json(calendar_events))
}
