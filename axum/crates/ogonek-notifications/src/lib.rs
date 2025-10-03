use ogonek_db::{
    core::account::{profile, student},
    notifications::read_device_tokens,
};

pub use crate::messages::NotificationType;
use crate::{apns::ApnsProvider, telegram::TelegramProvider};

use sqlx::PgPool;
use tracing::info;

mod apns;
mod error;
pub use error::NotificationError;
mod messages;
mod telegram;

#[derive(Debug, Clone)]
pub struct NotificationService {
    apns_provider: ApnsProvider,
    telegram_provider: TelegramProvider,
    db: PgPool,
}

impl NotificationService {
    pub fn new(db: PgPool) -> anyhow::Result<Self> {
        let telegram_provider = TelegramProvider::new()?;
        let apns_provider = ApnsProvider::new()?;

        Ok(Self {
            apns_provider,
            db,
            telegram_provider,
        })
    }

    pub async fn notify_student(
        &self,
        teacher_id: &str,
        student_id: &str,
        notification_type: NotificationType,
    ) -> Result<(), NotificationError> {
        info!(
            "Teacher {} notifying student {}: {:?}",
            teacher_id, student_id, notification_type
        );

        self.send_apns_notifications(student_id, &notification_type)
            .await?;

        if let Ok(Some(telegram_id)) =
            student::read_telegram_id(&self.db, teacher_id, student_id).await
        {
            self.telegram_provider
                .send_notification(&telegram_id, &notification_type)
                .await?;
        }

        Ok(())
    }

    pub async fn notify_teacher(
        &self,
        student_id: &str,
        notification_type: NotificationType,
    ) -> Result<(), NotificationError> {
        info!(
            "Student {} notifying teacher: {:?}",
            student_id, notification_type
        );

        if let Ok(Some(teacher_id)) = profile::read_teacher_user_id(&self.db, student_id).await {
            self.send_apns_notifications(&teacher_id, &notification_type)
                .await?;
        }

        if let Ok(Some(telegram_id)) = profile::read_teacher_telegram_id(&self.db, student_id).await
        {
            self.telegram_provider
                .send_notification(&telegram_id, &notification_type)
                .await?;
        }

        Ok(())
    }

    async fn send_apns_notifications(
        &self,
        recipient_id: &str,
        notification_type: &NotificationType,
    ) -> Result<(), NotificationError> {
        if let Ok(device_tokens) = read_device_tokens(&self.db, recipient_id).await {
            for token in device_tokens {
                let payload = notification_type.to_apns_payload();
                if let Err(e) = self.apns_provider.send_notification(&token, payload).await {
                    tracing::warn!("Failed to send APNS notification: {}", e);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl NotificationService {
    pub fn test(db: PgPool) -> anyhow::Result<Self> {
        let telegram_provider = TelegramProvider::test()?;
        let apns_provider = ApnsProvider::test();
        Ok(Self {
            apns_provider,
            db,
            telegram_provider,
        })
    }
}
