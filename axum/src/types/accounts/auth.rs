use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    #[validate(length(min = 3, max = 16))]
    #[schema(min_length = 3, max_length = 16, example = "john_doe")]
    pub username: String,

    #[validate(length(min = 8, max = 32))]
    #[schema(min_length = 8, max_length = 32, example = "MyPassword123")]
    pub pass: String,
}

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SignUpPayload {
    #[validate(length(min = 3))]
    #[schema(min_length = 3, max_length = 100, example = "John Doe")]
    pub name: String,

    #[validate(email)]
    #[schema(format = "email", example = "john@example.com")]
    pub email: String,

    #[validate(length(min = 8))]
    #[schema(min_length = 8, max_length = 128, example = "MyPassword123")]
    pub pass: String,

    #[validate(length(min = 2))]
    #[schema(min_length = 2, max_length = 50, example = "johndoe")]
    pub username: String,

    #[schema(example = "student")]
    pub role: String,
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
#[derive(Serialize, Deserialize, ToSchema)]
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

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InviterQuery {
    pub invite: String,
}
