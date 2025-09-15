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
