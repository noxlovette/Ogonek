use crate::types::{
    DeleteScope, EditScope, EventAttendee, EventClass, EventStatus, EventTransp,
    datetime_serialization,
};
use chrono::{DateTime, Utc};
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
    pub is_recurring: bool,
    pub is_exception: bool,
}

/// Internal Struct used to query the DB
#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDB {
    pub id: String,
    pub uid: String,

    #[serde(rename = "title")]
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(with = "datetime_serialization")]
    pub dtstart_time: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtend_time: Option<DateTime<Utc>>,

    pub status: EventStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rrule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdate: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exdate: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_id: Option<DateTime<Utc>>,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventFull {
    #[serde(flatten)]
    pub db_data: EventDBFull,
    pub is_recurring: bool,
    pub is_exception: bool,
}
impl From<EventDBFull> for EventFull {
    fn from(db_data: EventDBFull) -> Self {
        let is_recurring = db_data.rrule.is_some() && db_data.recurrence_id.is_none();
        let is_exception = db_data.recurrence_id.is_some();

        EventFull {
            db_data,
            is_recurring,
            is_exception,
        }
    }
}
#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDBFull {
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
    #[serde(rename = "title")]
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub url: Option<String>,

    // Time data
    #[serde(with = "datetime_serialization")]
    pub dtstart_time: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend_time: Option<DateTime<Utc>>,
    pub dtend_tz: Option<String>,
    pub dtstart_tz: Option<String>,

    // Recurrence data
    pub rrule: Option<String>,
    #[serde(skip_serializing)]
    pub rdate: Option<Vec<String>>,
    #[serde(skip_serializing)]
    pub exdate: Option<Vec<String>>,
    #[serde(with = "datetime_serialization::option")]
    #[serde(skip_serializing)]
    pub recurrence_id: Option<DateTime<Utc>>,
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
pub struct EventDelete {
    pub scope: DeleteScope,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventUpdateRequest {
    pub scope: EditScope,
    #[serde(flatten)]
    pub updates: EventUpdate,
}
#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventUpdate {
    pub description: Option<String>,
    pub attendee: Option<String>,
    pub location: Option<String>,
    pub dtstart_time: Option<DateTime<Utc>>,
    pub dtend_time: Option<DateTime<Utc>>,
    pub dtstart_tz: Option<String>,
    pub dtend_tz: Option<String>,
    pub rrule: Option<String>,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventWithAttendees {
    #[serde(flatten)]
    pub event: EventFull,
    pub attendees: Vec<EventAttendee>,
}
