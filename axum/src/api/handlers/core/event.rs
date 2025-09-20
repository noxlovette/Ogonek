use std::collections::HashMap;

use crate::api::CALENDAR_TAG;
use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::calendar::event::{
    create, delete, find_by_calendar_id_in_scope, find_by_date, find_by_id, update,
};
use crate::db::crud::core::calendar::event_attendee::find_by_event_id;
use crate::schema::AppState;
use crate::tools::rrule::RRule;
use crate::types::{
    CalendarQuery, EventCreate, EventDB, EventSmall, EventUpdate, EventWithAttendees,
};
use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};

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
    let event = find_by_id(&state.db, &id).await?;
    let attendees = find_by_event_id(&state.db, &id).await?;
    Ok(Json(EventWithAttendees { event, attendees }))
}
/// Get events for a specific month
#[utoipa::path(
    get,
    path = "/events/month",
    tag = CALENDAR_TAG,
    params(
        ("start" = DateTime<Utc>, description = "Year (e.g., 2024)"),
        ("end" = DateTime<Utc>, Path, description = "Month (1-12)")
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
    let events =
        find_by_calendar_id_in_scope(&state.db, &claims.sub, query.start, query.end).await?;

    let masters: Vec<_> = events
        .iter()
        .filter(|e| e.rrule.is_some() && e.recurrence_id.is_none())
        .collect();

    let mut exceptions: HashMap<String, Vec<&EventDB>> = HashMap::new();
    for e in events.iter().filter(|e| e.recurrence_id.is_some()) {
        exceptions.entry(e.uid.clone()).or_default().push(e);
    }

    let mut calendar_events: Vec<EventSmall> = Vec::new();

    // Process recurring events
    for master in masters {
        if let Some(rrule) = RRule::parse(master.rrule.clone())
            .map_err(|_| APIError::BadRequest("rrule parsing failed".to_string()))?
        {
            let occurrences = rrule.generate_occurrences(master.dtstart, query.start, query.end);

            for occurrence in occurrences {
                // Check if this occurrence has an exception
                let has_exception = exceptions
                    .get(&master.uid)
                    .map(|excs| excs.iter().any(|exc| exc.recurrence_id == Some(occurrence)))
                    .unwrap_or(false);

                if !has_exception {
                    // Generate virtual instance
                    calendar_events.push(EventSmall {
                        id: format!("{}_{}", master.id, occurrence.timestamp()),
                        uid: master.uid.clone(),
                        master_id: Some(master.id.clone()),
                        summary: master.summary.clone(),
                        dtstart: occurrence,
                        location: master.location.clone(),
                        // Calculate dtend based on duration
                        dtend: master.dtend.map(|end| occurrence + (end - master.dtstart)),
                        is_recurring: true,
                        is_exception: false,
                    });
                }
            }
        }
    }

    // Add exception instances
    for (uid, exception_list) in exceptions {
        for exception in exception_list {
            if exception.dtstart >= query.start && exception.dtstart <= query.end {
                calendar_events.push(EventSmall {
                    id: exception.id.clone(),
                    uid: uid.clone(),
                    master_id: None, // Could find master if needed
                    summary: exception.summary.clone(),
                    location: exception.location.clone(),
                    dtstart: exception.dtstart,
                    dtend: exception.dtend,
                    is_recurring: true,
                    is_exception: true,
                });
            }
        }
    }

    // Add single events
    let single_events = events
        .iter()
        .filter(|e| e.rrule.is_none() && e.recurrence_id.is_none());

    for event in single_events {
        calendar_events.push(EventSmall {
            id: event.id.clone(),
            uid: event.uid.clone(),
            master_id: None,
            summary: event.summary.clone(),
            location: event.location.clone(),
            dtstart: event.dtstart,
            dtend: event.dtend,
            is_recurring: false,
            is_exception: false,
        });
    }

    Ok(Json(calendar_events))
}

/// Get all events for a given day
#[utoipa::path(
    get,
    path = "/events/day/{day}",
    tag = CALENDAR_TAG,
    params(
        ("day" = String, Path, description = "Day")
    ),
    responses(
        (status = 200, description = "Events retrieved successfully", body = Vec<EventSmall>),
        (status = 404, description = "Calendar not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_events_day(
    State(state): State<AppState>,
    Path(day): Path<String>,
    _claims: Claims,
) -> Result<Json<Vec<EventDB>>, APIError> {
    let naive_date = chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d")
        .map_err(|_| APIError::BadRequest("Invalid Date Format".to_string()))?;
    let events = find_by_date(&state.db, naive_date).await?;
    Ok(Json(events))
}

/// Create a new event
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
    if payload.dtend.is_none() {
        payload.dtend = Some(payload.dtstart + chrono::Duration::hours(1));
    }

    create(&state.db, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}

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
    Json(payload): Json<EventUpdate>,
) -> Result<StatusCode, APIError> {
    update(&state.db, &id, &payload).await?;
    Ok(StatusCode::NO_CONTENT)
}
