use crate::error::AppError;
use crate::notifications::messages::NotificationType;
use reqwest::Client;
use tracing::info;

pub mod error;
pub mod messages;
pub mod telegram;

pub async fn dispatch_notification(
    bot_token: &str,
    http_client: &Client,
    recipient_id: &str,
    notification_type: NotificationType,
) -> Result<(), AppError> {
    info!(
        "Dispatching {:?} notification to {}",
        notification_type, recipient_id
    );

    telegram::send_telegram_notification(http_client, bot_token, recipient_id, &notification_type)
        .await?;

    Ok(())
}
