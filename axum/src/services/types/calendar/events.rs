use std::fmt;

use crate::types::{EventAttendee, datetime_serialization};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use utoipa::ToSchema;
use validator::Validate;

pub struct CalendarQuery {
    pub start: String,
    pub end: String,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSmall {
    pub id: String,
    pub uid: String,
    #[serde(alias = "title")]
    pub summary: String,
    pub location: Option<String>,
    #[serde(with = "datetime_serialization")]
    pub dtstart: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend: Option<DateTime<Utc>>,
    pub rrule: Option<String>,
}

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventFull {
    pub id: String,
    pub uid: String,
    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,

    #[serde(skip_serializing)]
    pub calendar_id: String,
    #[serde(alias = "title")]
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    #[serde(skip_serializing)]
    pub url: Option<String>,

    #[serde(with = "datetime_serialization")]
    pub dtstart: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend: Option<DateTime<Utc>>,

    pub all_day: bool,
    pub timezone: Option<String>,

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

    pub organiser_email: Option<String>,
    pub organiser_name: Option<String>,

    #[serde(skip_serializing)]
    pub sequence: i32,
    #[serde(skip_serializing)]
    pub dtstamp: DateTime<Utc>,

    #[serde(skip_serializing)]
    #[validate(length(equal = 64))]
    pub etag: String,

    #[serde(skip_serializing)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Tentative,
    Cancelled,
}

#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventClass {
    Public,
    Private,
    Confidential,
}

#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventTransp {
    Opaque,
    Transparent,
}

#[derive(Validate, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventCreate {
    pub attendee: String,
    #[serde(with = "datetime_serialization")]
    pub dtstart: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend: Option<DateTime<Utc>>,
}

#[derive(Validate, ToSchema, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventUpdate {
    pub description: Option<String>,
    pub location: Option<String>,
    #[serde(with = "datetime_serialization::option")]
    pub dtstart: Option<DateTime<Utc>>,
    #[serde(with = "datetime_serialization::option")]
    pub dtend: Option<DateTime<Utc>>,
    pub timezone: Option<String>,
    pub rrule: Option<String>,
    /// The invited student's id
    pub attendee: Option<String>,
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
            EventClass::Public => write!(f, "public"),
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

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventWithAttendees {
    #[serde(flatten)]
    pub event: EventFull,
    pub attendees: Vec<EventAttendee>,
}
