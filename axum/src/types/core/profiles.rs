use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The default profile struct
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: String,
    pub video_call_url: Option<String>,
    pub avatar_url: Option<String>,
    pub telegram_id: Option<String>,
}

/// The profile that gets decoded
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfileUpdate {
    pub video_call_url: Option<String>,
    pub avatar_url: Option<String>,
    pub telegram_id: Option<String>,
}
