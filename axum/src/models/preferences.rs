// src/models/user_preferences.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserPreferences {
    pub user_id: String,
    pub auto_subscribe: bool,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub theme: String,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPreferencesUpdate {
    pub auto_subscribe: Option<bool>,
    pub email_notifications: Option<bool>,
    pub push_notifications: Option<bool>,
    pub theme: Option<String>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserPreferencesResponse {
    pub auto_subscribe: bool,
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub theme: String,
    pub language: String,
}
