use crate::types::datetime_serialization;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::ipnetwork::IpNetwork;

#[derive(Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub event_type: String,
    pub action: String,
    pub outcome: String,
    pub user_id: Option<String>,
    pub user_email: String,
    pub user_role: String,
    pub impersonated_by: Option<String>,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub resource_name: Option<String>,
    pub payload: serde_json::Value,
    pub session_id: Option<String>,
    pub ip_address: Option<IpNetwork>,
    pub user_agent: Option<String>,
    pub request_id: Option<String>,
    pub severity: String, // info, debug, error
    pub tags: Vec<String>,
    #[serde(with = "datetime_serialization")]
    pub occurred_at: DateTime<Utc>,
    #[serde(with = "datetime_serialization::option")]
    pub retention_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogCreate {
    pub event_type: String,
    pub action: String,
    pub outcome: Option<String>, // defaults to "success"

    pub user_id: Option<String>,
    pub user_email: String,
    pub user_role: String,
    pub impersonated_by: Option<String>,

    pub resource_type: String,
    pub resource_id: Option<String>,
    pub resource_name: Option<String>,

    pub payload: Option<serde_json::Value>, // defaults to {}

    pub session_id: Option<String>,
    pub ip_address: Option<IpNetwork>,
    pub user_agent: Option<String>,
    pub request_id: Option<String>,

    pub severity: Option<String>,  // defaults to "info"
    pub tags: Option<Vec<String>>, // defaults to []

    pub retention_until: Option<DateTime<Utc>>,
}
