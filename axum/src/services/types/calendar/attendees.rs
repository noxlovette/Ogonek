use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventAttendee {
    pub id: String,
    pub event_id: String,
    pub email: String,
    pub name: Option<String>,
    pub role: EventAttendeeRole,
    pub status: EventAttendeeStatus,
    pub rsvp: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(ToSchema, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "kebab-case")]
pub enum EventAttendeeRole {
    ReqParticipant,
    Chair,
    OptParticipant,
    NonParticipant,
}

#[derive(ToSchema, Serialize, Deserialize, Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
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
    pub email: String,
    pub name: Option<String>,
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

impl fmt::Display for EventAttendeeRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventAttendeeRole::ReqParticipant => write!(f, "req-participant"),
            EventAttendeeRole::Chair => write!(f, "chair"),
            EventAttendeeRole::OptParticipant => write!(f, "opt-participant"),
            EventAttendeeRole::NonParticipant => write!(f, "non-participant"),
        }
    }
}

impl fmt::Display for EventAttendeeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NeedsAction => write!(f, "needs-action"),
            Self::Accepted => write!(f, "accepted"),
            Self::Declined => write!(f, "declined"),
            Self::Delegated => write!(f, "delegated"),
            Self::Tentative => write!(f, "tentative"),
        }
    }
}
