use crate::types::datetime_serialization;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub id: String,
    pub name: String,
    pub description: Option<String>,

    #[validate(length(equal = 7))]
    pub colour: String,
    #[serde(skip_serializing)]
    pub timezone: String,
    #[serde(skip_serializing)]
    pub owner_id: String,

    #[serde(skip_serializing)]
    pub caldav_url: Option<String>,
    #[serde(skip_serializing)]
    pub sync_token: Option<String>,

    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub colour: Option<String>,
    pub timezone: Option<String>,
    pub caldav_url: Option<String>,
    pub sync_token: Option<String>,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarCreate {
    pub name: String,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    #[serde(skip_serializing)]
    pub id: String,
    pub uid: String,
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,

    #[serde(skip_serializing)]
    pub calendar_id: String,

    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,

    #[serde(with = "datetime_serialization")]
    pub dtstart: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend: Option<DateTime<Utc>>,

    pub duration: Option<chrono::Duration>,
    pub all_day: bool,
    pub timezome: Option<String>,

    pub rrule: Option<String>,
    pub rdate: Option<Vec<String>>,
    pub exdate: Option<Vec<String>>,
    #[serde(with = "datetime_serialization::option")]
    pub recurrence_id: Option<DateTime<Utc>>,

    pub status: EventStatus,
    pub class: EventClass,
    pub transp: EventTransp,

    #[validate(range(min = 0, max = 9))]
    pub priority: Option<i64>,

    pub categories: Option<Vec<String>>,

    pub organiser_email: Option<String>,
    pub organiser_name: Option<String>,

    pub sequence: i64,
    #[serde(skip_serializing)]
    pub dtstamp: DateTime<Utc>,

    #[validate(length(equal = 64))]
    pub etag: String,

    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Tentative,
    Cancelled,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventClass {
    Public,
    Private,
    Confidential,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventTransp {
    Opaque,
    Transparent,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendee {
    id: String,
    event_id: String,
    email: String,
    name: Option<String>,
    role: EventAttendeeRole,
    status: EventAttendeeStatus,
    rsvp: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventAttendeeRole {
    ReqParticipant,
    Chair,
    OptParticipant,
    NonParticipant,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventAttendeeStatus {
    NeedsAction,
    Accepted,
    Declined,
    Tentative,
    Delegated,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAlarm {
    pub id: String,
    pub event_id: String,
    pub trigger_offset: Option<chrono::Duration>,
    pub trigger_datetime: DateTime<Utc>,
    pub action: EventAlarmAction,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub attendee_email: Option<String>,
    pub attendee_telegram_id: Option<String>,
    pub repeat_count: i64,
    pub repeat_duration: Option<chrono::Duration>,
    pub created_at: DateTime<Utc>,
}

#[derive(ToSchema, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventAlarmAction {
    Display,
    Audio,
    Email,
    Procedure,
}
