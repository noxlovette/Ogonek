use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: String,
    pub quizlet_url: Option<String>,
    pub zoom_url: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileUpdate {
    pub quizlet_url: Option<String>,
    pub zoom_url: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>
}