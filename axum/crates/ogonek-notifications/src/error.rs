use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "type", content = "details")]
pub enum NotificationError {
    #[error("Chat not found: {chat_id}")]
    ChatNotFound { chat_id: String },

    #[error("Bot was blocked by user: {chat_id}")]
    BotBlocked { chat_id: String },

    #[error("Invalid bot token")]
    InvalidBotToken,

    #[error("Telegram rate limit exceeded")]
    RateLimited,

    #[error("Telegram API error {code}: {description}")]
    TelegramApiError {
        code: u32,
        description: String,
        chat_id: String,
    },

    #[error("Network error: {message}")]
    NetworkError { message: String },

    #[error("Timeout sending notification")]
    Timeout,
}

impl NotificationError {
    pub fn error_code(&self) -> u16 {
        match self {
            Self::ChatNotFound { .. } => 4001,
            Self::BotBlocked { .. } => 4002,
            Self::InvalidBotToken => 4003,
            Self::RateLimited => 4029,
            Self::TelegramApiError { .. } => 4000,
            Self::NetworkError { .. } => 5001,
            Self::Timeout => 5002,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TelegramApiResponse {
    pub ok: bool,
    pub error_code: Option<u32>,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct TelegramError {
    pub error_code: u32,
    pub description: String,
}
