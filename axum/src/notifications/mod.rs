use crate::error::AppError;
use crate::notifications::apns::ApnsProvider;
use crate::notifications::messages::NotificationType;
use crate::types::NotificationPayload;
use anyhow::Context;
use reqwest::Client;
use tracing::info;

pub mod apns;
pub mod error;
pub mod messages;
pub mod telegram;

#[derive(Debug, Clone)]
pub struct NotificationService {
    apns: ApnsProvider,
    http_client: Client,
    bot_token: String,
}

impl NotificationService {
    pub fn new() -> anyhow::Result<Self> {
        let http_client = Client::new();
        let bot_token = std::env::var("TELEGRAM_KEY").context("TELEGRAM_KEY needs to be set")?;
        let apns = ApnsProvider::new()?;

        Ok(Self {
            apns,
            http_client,
            bot_token,
        })
    }

    pub async fn notify_user(
        &self,
        user_device_token: &str,
        title: &str,
        message: &str,
        custom_data: Option<serde_json::Value>,
    ) -> Result<(), anyhow::Error> {
        let payload = NotificationPayload {
            title: title.to_string(),
            body: message.to_string(),
            badge: Some(1), // You might want to track this in your DB
            sound: Some("default".to_string()),
            data: custom_data,
        };

        self.apns
            .send_notification(user_device_token, payload)
            .await
    }

    pub async fn dispatch_notification(
        &self,
        recipient_id: &str,
        notification_type: NotificationType,
    ) -> Result<(), AppError> {
        info!(
            "Dispatching {:?} notification to {}",
            notification_type, recipient_id
        );

        telegram::send_telegram_notification(
            &self.http_client,
            &self.bot_token,
            recipient_id,
            &notification_type,
        )
        .await?;

        Ok(())
    }
}
