use crate::api::handlers::core::calendar::*;
use crate::api::handlers::core::event::*;
use crate::api::handlers::core::event_attendee::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    fetch_calendar,
    delete_calendar,
    update_calendar,
    fetch_event,
    list_events,
    create_event,
    delete_event,
    update_event,
    delete_attendee,
    update_attendee,
))]
pub struct CalendarApi;
