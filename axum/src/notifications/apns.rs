use a2::{
    Client, ClientConfig, DefaultNotificationBuilder, Endpoint, NotificationBuilder,
    NotificationOptions,
};
use anyhow::{Context, Result};
use std::sync::Arc;
use tracing::{debug, error, instrument};

use crate::types::NotificationPayload;

#[derive(Debug, Clone)]
pub struct ApnsProvider {
    client: Arc<Client>,
    topic: String,
}

impl ApnsProvider {
    pub fn new() -> Result<Self> {
        let is_production = match std::env::var("APP_ENV") {
            Ok(env) => env == "production",
            Err(_) => false,
        };

        let endpoint = if is_production {
            Endpoint::Production
        } else {
            Endpoint::Sandbox
        };

        let key = std::env::var("APNS_KEY").context("NO APNS KEY")?;

        let key_id = std::env::var("APNS_KEY_ID").context("APNS_KEY_ID needs to be set")?;
        let team_id = std::env::var("APNS_TEAM_ID").context("APNS_TEAM_ID needs to be set")?;

        let topic = std::env::var("APNS_TOPIC").context("APNS_TOPIC needs to be set")?;

        let client = Client::token(
            std::io::Cursor::new(key),
            key_id,
            team_id,
            ClientConfig::new(endpoint),
        )
        .context("Failed to create APNS client with P8 token")?;

        Ok(Self {
            client: Arc::new(client),
            topic,
        })
    }

    #[instrument(skip(self), fields(topic = %self.topic))]
    pub async fn send_notification(
        &self,
        device_token: &str,
        payload: NotificationPayload,
    ) -> Result<()> {
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

        match self.client.send(notification).await {
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
