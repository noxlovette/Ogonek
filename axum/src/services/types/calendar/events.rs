use crate::types::{
    EditScope, EventAttendee, EventClass, EventStatus, EventTransp, RecurrenceRange,
    datetime_serialization,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CalendarQuery {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSmall {
    #[serde(flatten)]
    pub db_data: EventDB,
    pub master_id: Option<String>,
    pub is_recurring: bool,
    pub is_exception: bool,
}

/// Internal Struct used to query the DB
#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDB {
    pub id: String,
    pub uid: String,
    #[serde(rename(serialize = "title"))]
    pub summary: String,
    pub location: Option<String>,
    #[serde(with = "datetime_serialization")]
    pub dtstart_time: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend_time: Option<DateTime<Utc>>,
    pub rrule: Option<String>,
    pub recurrence_id: Option<DateTime<Utc>>,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventFull {
    // Basic data
    pub id: String,
    pub uid: String,
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub calendar_id: String,

    // Secondary data
    #[serde(alias = "title")]
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub url: Option<String>,

    // Time data
    #[serde(with = "datetime_serialization::option")]
    pub dtstart_time: Option<DateTime<Utc>>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend_time: Option<DateTime<Utc>>,
    pub dtend_tz: Option<String>,
    pub dtstart_tz: String,
    pub dtstart_date: Option<NaiveDate>,
    pub dtend_date: Option<NaiveDate>,
    pub all_day: bool,

    // Recurrence data
    pub rrule: Option<String>,
    #[serde(skip_serializing)]
    pub rdate: Option<Vec<String>>,
    #[serde(skip_serializing)]
    pub exdate: Option<Vec<String>>,
    #[serde(with = "datetime_serialization::option")]
    #[serde(skip_serializing)]
    pub recurrence_id: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub recurrence_range: Option<RecurrenceRange>,

    pub status: EventStatus,
    #[serde(skip_serializing)]
    pub class: EventClass,
    #[serde(skip_serializing)]
    pub transp: EventTransp,

    #[serde(skip_serializing)]
    #[validate(range(min = 0, max = 9))]
    pub priority: Option<i32>,

    pub categories: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub sequence: i32,
    #[serde(skip_serializing)]
    pub dtstamp: DateTime<Utc>,

    #[serde(skip_serializing)]
    #[validate(length(equal = 64))]
    pub etag: String,

    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,

    #[serde(skip_serializing)]
    pub caldav_href: Option<String>,
    #[serde(skip_serializing)]
    pub content_type: Option<String>,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventCreate {
    pub attendee: String,
    #[serde(with = "datetime_serialization")]
    pub dtstart_time: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend_time: Option<DateTime<Utc>>,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventUpdateRequest {
    pub edit_scope: EditScope,
    pub uid: String,
    /// This is gonna be the master/regular event or a recurrence instance if there is an id_timestamp in it
    pub event_id: String,
    #[serde(flatten)]
    pub updates: EventUpdate,
}
#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventUpdate {
    pub description: Option<String>,
    pub attendee: Option<String>,
    pub location: Option<String>,
    #[serde(with = "datetime_serialization::option")]
    pub dtstart_time: Option<DateTime<Utc>>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend_time: Option<DateTime<Utc>>,
    pub dtstart_tz: Option<String>,
    pub dtend_tz: Option<String>,
    pub rrule: Option<String>,
    pub dtstart_date: Option<NaiveDate>,
    pub dtend_date: Option<NaiveDate>,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventWithAttendees {
    #[serde(flatten)]
    pub event: EventFull,
    pub attendees: Vec<EventAttendee>,
}
