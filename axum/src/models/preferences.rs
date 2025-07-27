use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(ToSchema, Clone, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    #[serde(skip_serializing)]
    pub user_id: String,
    pub auto_subscribe: bool,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub theme: String,
    pub language: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferencesUpdate {
    pub auto_subscribe: Option<bool>,
    pub email_notifications: Option<bool>,
    pub push_notifications: Option<bool>,
    pub theme: Option<String>,
    pub language: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferencesResponse {
    pub auto_subscribe: bool,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub theme: String,
    pub language: String,
}
