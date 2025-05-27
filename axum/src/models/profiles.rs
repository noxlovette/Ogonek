use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: String,
    pub video_call_url: Option<String>,
    pub avatar_url: Option<String>,
    pub telegram_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileUpdate {
    pub video_call_url: Option<String>,
    pub avatar_url: Option<String>,
    pub telegram_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileWithTS {
    pub profile: Profile,
    pub teacher_data: Option<TeacherData>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherData {
    pub teacher_video_call_url: Option<String>,
    pub teacher_telegram_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileParams {
    pub is_student: String,
}
