use serde::{Deserialize, Serialize};

/// The default profile struct
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: String,
    pub video_call_url: Option<String>,
    pub avatar_url: Option<String>,
    pub telegram_id: Option<String>,
}

/// The profile that gets decoded
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileUpdate {
    pub video_call_url: Option<String>,
    pub avatar_url: Option<String>,
    pub telegram_id: Option<String>,
}

/// This is sent along with dashboard data to
/// include teacher video url and stuff if the user is a student
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProfileWithTS {
    pub profile: Profile,
    pub teacher_data: Option<TeacherData>,
}

/// Video Call URL and the teacher's TelegramID
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeacherData {
    pub teacher_video_call_url: Option<String>,
    pub teacher_telegram_id: Option<String>,
}
