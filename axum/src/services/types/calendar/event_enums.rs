use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use std::fmt;
use utoipa::ToSchema;
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DeleteScope {
    ThisOnly,
    ThisAndFuture,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum EditScope {
    ThisOnly,
    ThisAndFuture,
}
// Event Status ENUM
#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "event_status")]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    #[sqlx(rename = "tentative")]
    Tentative,
    #[sqlx(rename = "confirmed")]
    Confirmed,
    #[sqlx(rename = "cancelled")]
    Cancelled,
}

// Event Class ENUM
#[derive(ToSchema, Serialize, PartialOrd, Deserialize, Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "event_class", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum EventClass {
    Public,
    Private,
    Confidential,
}

// Event Transparency ENUM
#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "event_transp")]
#[serde(rename_all = "lowercase")]
pub enum EventTransp {
    #[sqlx(rename = "opaque")]
    Opaque,
    #[sqlx(rename = "transparent")]
    Transparent,
}

// Attendee Role ENUM
#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "attendee_role")]
#[serde(rename_all = "kebab-case")]
pub enum AttendeeRole {
    #[sqlx(rename = "chair")]
    Chair,
    #[sqlx(rename = "req-participant")]
    ReqParticipant,
    #[sqlx(rename = "opt-participant")]
    OptParticipant,
    #[sqlx(rename = "non-participant")]
    NonParticipant,
}

// Attendee Status ENUM
#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "attendee_status")]
#[serde(rename_all = "kebab-case")]
pub enum AttendeeStatus {
    #[sqlx(rename = "needs-action")]
    NeedsAction,
    #[sqlx(rename = "accepted")]
    Accepted,
    #[sqlx(rename = "declined")]
    Declined,
    #[sqlx(rename = "tentative")]
    Tentative,
    #[sqlx(rename = "delegated")]
    Delegated,
}

// Alarm Action ENUM
#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "alarm_action")]
#[serde(rename_all = "lowercase")]
pub enum AlarmAction {
    #[sqlx(rename = "audio")]
    Audio,
    #[sqlx(rename = "display")]
    Display,
    #[sqlx(rename = "email")]
    Email,
    #[sqlx(rename = "procedure")]
    Procedure,
}

// Sync State ENUM
#[derive(ToSchema, Serialize, Deserialize, Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "sync_state")]
#[serde(rename_all = "lowercase")]
pub enum SyncState {
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "syncing")]
    Syncing,
    #[sqlx(rename = "error")]
    Error,
}

// Implement Display traits for better debugging/logging
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

impl fmt::Display for AttendeeRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttendeeRole::Chair => write!(f, "chair"),
            AttendeeRole::ReqParticipant => write!(f, "req-participant"),
            AttendeeRole::OptParticipant => write!(f, "opt-participant"),
            AttendeeRole::NonParticipant => write!(f, "non-participant"),
        }
    }
}

impl fmt::Display for AttendeeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttendeeStatus::NeedsAction => write!(f, "needs-action"),
            AttendeeStatus::Accepted => write!(f, "accepted"),
            AttendeeStatus::Declined => write!(f, "declined"),
            AttendeeStatus::Tentative => write!(f, "tentative"),
            AttendeeStatus::Delegated => write!(f, "delegated"),
        }
    }
}

// Default implementations for commonly used enums
impl Default for EventStatus {
    fn default() -> Self {
        EventStatus::Confirmed
    }
}

impl Default for EventClass {
    fn default() -> Self {
        EventClass::Public
    }
}

impl Default for EventTransp {
    fn default() -> Self {
        EventTransp::Opaque
    }
}

impl Default for AttendeeRole {
    fn default() -> Self {
        AttendeeRole::ReqParticipant
    }
}

impl Default for AttendeeStatus {
    fn default() -> Self {
        AttendeeStatus::NeedsAction
    }
}

impl Default for SyncState {
    fn default() -> Self {
        SyncState::Active
    }
}
