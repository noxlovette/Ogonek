use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize)]
pub struct APNsPayload {
    pub aps: APSData,
    #[serde(flatten)]
    pub custom_data: serde_json::Value,
}

#[derive(Serialize)]
pub struct APSData {
    pub alert: AlertData,
    pub badge: Option<u32>,
    pub sound: Option<String>,
    #[serde(rename = "content-available")]
    pub content_available: Option<u8>,
}

#[derive(Serialize)]
pub struct AlertData {
    pub title: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
}

#[derive(Serialize)]
pub struct TaskNotificationData {
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

pub enum NotificationType {
    NewTask,
    NewLesson,
    NewDeck,
    TaskCompleted,
}

impl NotificationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NewDeck => "deck",
            Self::NewLesson => "lesson",
            Self::NewTask => "task",
            Self::TaskCompleted => "task_completed",
        }
    }
}
