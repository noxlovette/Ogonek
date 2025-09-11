use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct DeviceTokenPayload {
    pub token: String,
    pub platform: String,
}
#[derive(Debug, Clone, Serialize)]
pub struct NotificationPayload {
    pub title: String,
    pub body: String,
    pub badge: Option<u32>,
    pub sound: Option<String>,
    pub data: Option<serde_json::Value>,
}

impl Default for NotificationPayload {
    fn default() -> Self {
        Self {
            title: "New Notification".to_string(),
            body: "You have a new notification".to_string(),
            badge: None,
            sound: Some("default".to_string()),
            data: None,
        }
    }
}
