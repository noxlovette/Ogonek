use crate::types::NotificationPayload;
use a2::{
    Client, ClientConfig, DefaultNotificationBuilder, Endpoint, NotificationBuilder,
    NotificationOptions,
};
use anyhow::{Context, Result};
use std::sync::Arc;
use tracing::{debug, error, instrument};

#[derive(Debug, Clone)]
pub struct ApnsProvider {
    client: Option<Arc<Client>>,
    topic: String,
}

impl ApnsProvider {
    pub fn new() -> Result<Self> {
        let is_production = std::env::var("APP_ENV")
            .map(|env| env == "production")
            .unwrap_or(false);

        let endpoint = if is_production {
            Endpoint::Production
        } else {
            Endpoint::Sandbox
        };

        let key = std::env::var("APNS_KEY").context("APNS_KEY must be set")?;
        let key_id = std::env::var("APNS_KEY_ID").context("APNS_KEY_ID must be set")?;
        let team_id = std::env::var("APNS_TEAM_ID").context("APNS_TEAM_ID must be set")?;
        let topic = std::env::var("APNS_TOPIC").context("APNS_TOPIC must be set")?;

        let client = Client::token(
            std::io::Cursor::new(key),
            key_id,
            team_id,
            ClientConfig::new(endpoint),
        )
        .context("Failed to create APNS client with P8 token")?;

        Ok(Self {
            client: Some(Arc::new(client)), // Fixed: wrap in Some()
            topic,
        })
    }

    #[instrument(skip(self), fields(topic = %self.topic))]
    pub async fn send_notification(
        &self,
        device_token: &str,
        payload: NotificationPayload,
    ) -> Result<()> {
        // Early return if no client (test mode)
        let client = self
            .client
            .as_ref()
            .context("APNS client not available (test mode?)")?;

        debug!("Sending push notification to device");

        let mut builder = DefaultNotificationBuilder::new()
            .set_title(&payload.title)
            .set_body(&payload.body)
            .set_content_available();

        if let Some(badge) = payload.badge {
            builder = builder.set_badge(badge);
        }

        if let Some(sound) = &payload.sound {
            builder = builder.set_sound(sound);
        }

        let options = NotificationOptions {
            apns_topic: Some(&self.topic),
            ..Default::default()
        };

        let mut notification = builder.build(device_token, options);

        if let Some(data) = &payload.data {
            notification.add_custom_data("data", data)?;
        }

        match client.send(notification).await {
            Ok(response) => {
                if let Some(error) = response.error {
                    error!("APNS returned error response: {:?}", error);
                    anyhow::bail!("APNS error: {:?}", error);
                }
                debug!("Notification sent successfully");
                Ok(())
            }
            Err(e) => {
                error!("Failed to send notification: {}", e);
                Err(e.into())
            }
        }
    }
}

#[cfg(test)]
impl ApnsProvider {
    pub fn test() -> Self {
        Self {
            client: None, // No client in tests
            topic: "test.app.bundle".to_string(),
        }
    }
}
