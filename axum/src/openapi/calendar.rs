use crate::api::handlers::core::calendar::*;
use crate::api::handlers::core::event::*;
use crate::api::handlers::core::event_attendee::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    fetch_calendar,
    list_calendars,
    create_calendar,
    delete_calendar,
    update_calendar,
    fetch_event,
    list_events,
    create_event,
    delete_event,
    update_event,
    fetch_attendee,
    list_attendees,
    create_attendee,
    delete_attendee,
    update_attendee,
    list_events_day,
))]
pub struct CalendarApi;
