use super::error::{NotificationError, TelegramApiResponse};
use super::messages::NotificationType;
use reqwest::Client;
use serde_json::json;

pub async fn send_telegram_notification(
    client: &Client,
    bot_token: &str,
    telegram_id: &str,
    notification: &NotificationType,
) -> Result<(), NotificationError> {
    let message = notification.to_message();
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let response = client
        .post(&url)
        .json(&json!({
            "chat_id": telegram_id,
            "text": message,
            "parse_mode": "MarkdownV2"
        }))
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                NotificationError::Timeout
            } else {
                NotificationError::NetworkError {
                    message: e.to_string(),
                }
            }
        })?;

    let response_text = response
        .text()
        .await
        .map_err(|e| NotificationError::NetworkError {
            message: format!("Failed to read response: {}", e),
        })?;

    // Parse the Telegram API response
    let api_response: TelegramApiResponse =
        serde_json::from_str(&response_text).map_err(|_| NotificationError::NetworkError {
            message: "Invalid JSON response from Telegram".to_string(),
        })?;

    if !api_response.ok {
        let error_code = api_response.error_code.unwrap_or(0);
        let description = api_response
            .description
            .unwrap_or("Unknown error".to_string());

        // Map common Telegram errors to specific variants
        return Err(match error_code {
            400 if description.contains("chat not found") => NotificationError::ChatNotFound {
                chat_id: telegram_id.to_string(),
            },
            403 if description.contains("blocked") => NotificationError::BotBlocked {
                chat_id: telegram_id.to_string(),
            },
            401 => NotificationError::InvalidBotToken,
            429 => NotificationError::RateLimited,
            _ => NotificationError::TelegramApiError {
                code: error_code,
                description,
                chat_id: telegram_id.to_string(),
            },
        });
    }

    Ok(())
}
