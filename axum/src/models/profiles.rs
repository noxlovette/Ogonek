use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    user_id: String,
    quizlet_url: Option<String>,
    bio: Option<String>,
    avatar_url: Option<String>,
    timezone: Option<String>
}
