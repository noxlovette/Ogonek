use std::fmt;

use crate::types::datetime_serialization;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
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

    pub all_day: bool,
    pub timezone: Option<String>,

    pub rrule: Option<String>,
    pub rdate: Option<Vec<String>>,
    pub exdate: Option<Vec<String>>,
    #[serde(with = "datetime_serialization::option")]
    pub recurrence_id: Option<DateTime<Utc>>,

    pub status: EventStatus,
    pub class: EventClass,
    pub transp: EventTransp,

    #[validate(range(min = 0, max = 9))]
    pub priority: Option<i32>,

    pub categories: Option<Vec<String>>,

    pub organiser_email: Option<String>,
    pub organiser_name: Option<String>,

    pub sequence: i32,
    #[serde(skip_serializing)]
    pub dtstamp: DateTime<Utc>,

    #[validate(length(equal = 64))]
    pub etag: String,

    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(ToSchema, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Tentative,
    Cancelled,
}

#[derive(ToSchema, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventClass {
    Public,
    Private,
    Confidential,
}

#[derive(ToSchema, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventTransp {
    Opaque,
    Transparent,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventCreate {
    pub summary: String,
    #[serde(with = "datetime_serialization")]
    pub dtstart: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend: Option<DateTime<Utc>>,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventUpdate {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    #[serde(with = "datetime_serialization::option")]
    pub dtstart: Option<DateTime<Utc>>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend: Option<DateTime<Utc>>,
    pub all_day: Option<bool>,
    pub timezone: Option<String>,
    pub rrule: Option<String>,
    pub rdate: Option<Vec<String>>,
    pub exdate: Option<Vec<String>>,
    #[serde(with = "datetime_serialization::option")]
    pub recurrence_id: Option<DateTime<Utc>>,
    pub status: Option<EventStatus>,
    pub class: Option<EventClass>,
    pub transp: Option<EventTransp>,
    #[validate(range(min = 0, max = 9))]
    pub priority: Option<i32>,
    pub categories: Option<Vec<String>>,
    pub organiser_email: Option<String>,
    pub organiser_name: Option<String>,
    pub sequence: Option<i32>,
    #[serde(with = "datetime_serialization::option")]
    pub dtstamp: Option<DateTime<Utc>>,
    #[validate(length(equal = 64))]
    pub etag: Option<String>,
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

#[derive(ToSchema, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventAttendeeRole {
    ReqParticipant,
    Chair,
    OptParticipant,
    NonParticipant,
}

#[derive(ToSchema, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventAttendeeStatus {
    NeedsAction,
    Accepted,
    Declined,
    Tentative,
    Delegated,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendeeCreate {
    pub event_id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: EventAttendeeRole,
    pub status: EventAttendeeStatus,
    pub rsvp: bool,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendeeUpdate {
    pub email: Option<String>,
    pub name: Option<String>,
    pub role: Option<EventAttendeeRole>,
    pub status: Option<EventAttendeeStatus>,
    pub rsvp: Option<bool>,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAlarm {
    pub id: String,
    pub event_id: String,
    pub trigger_datetime: DateTime<Utc>,
    pub action: EventAlarmAction,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub attendee_email: Option<String>,
    pub attendee_telegram_id: Option<String>,
    pub repeat_count: i32,
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

impl fmt::Display for EventStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventStatus::Confirmed => write!(f, "confirmed"),
            EventStatus::Tentative => write!(f, "tentative"),
            EventStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl fmt::Display for EventClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventClass::Public => write!(f, "publicc"),
            EventClass::Private => write!(f, "private"),
            EventClass::Confidential => write!(f, "confidential"),
        }
    }
}

impl fmt::Display for EventTransp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventTransp::Opaque => write!(f, "opaque"),
            EventTransp::Transparent => write!(f, "transparent"),
        }
    }
}
