use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SignUpPayload {
    #[validate(length(min = 3))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub pass: String,

    #[validate(length(min = 2))]
    pub username: String,
    pub role: String,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdate {
    pub name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub pass: Option<String>,
    pub role: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub pass: String,
    pub role: String,
}

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    #[validate(length(min = 3, max = 16))]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub pass: String,
}

#[derive(Serialize, ToSchema)]
pub struct TokenWithExpiry {
    pub token: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: u64,
}

#[derive(Serialize, ToSchema)]
pub struct RefreshTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: TokenWithExpiry,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshTokenPayload {
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct TokenPair {
    #[serde(rename = "accessToken")]
    pub access_token: TokenWithExpiry,
    #[serde(rename = "refreshToken")]
    pub refresh_token: TokenWithExpiry,
}

impl TokenPair {
    pub fn new(access_token: TokenWithExpiry, refresh_token: TokenWithExpiry) -> Self {
        Self {
            access_token: TokenWithExpiry {
                token: access_token.token,
                expires_at: access_token.expires_at,
            },
            refresh_token: TokenWithExpiry {
                token: refresh_token.token,
                expires_at: refresh_token.expires_at,
            },
        }
    }
}

// Simple struct to hold the invite data
#[derive(Serialize, Deserialize)]
pub struct InviteToken {
    pub teacher_id: String,
    pub created_at: DateTime<Utc>,
}

impl InviteToken {
    pub fn new(teacher_id: String) -> Self {
        Self {
            teacher_id,
            created_at: Utc::now(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BindPayload {
    pub student_id: String,
    pub invite_token: String,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InviteQuery {
    pub is_registered: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviterQuery {
    pub invite: String,
}
