use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize)]
struct APNsPayload {
    pub aps: APSData,
    #[serde(flatten)]
    pub custom_data: serde_json::Value,
}

#[derive(Serialize)]
struct APSData {
    pub alert: AlertData,
    pub badge: Option<u32>,
    pub sound: Option<String>,
    #[serde(rename = "content-available")]
    pub content_available: Option<u8>,
}

#[derive(Serialize)]
struct AlertData {
    pub title: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
}

#[derive(Serialize)]
struct TaskNotificationData {
    #[serde(rename = "type")]
    pub notification_type: String, // "new_task", "lesson_added", etc.
    pub task_id: String,
    pub assignee: String,
}

#[derive(Deserialize, ToSchema)]
pub struct DeviceTokenPayload {
    pub token: String,
    pub platform: String,
}
