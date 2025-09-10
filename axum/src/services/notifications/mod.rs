use crate::db::crud::core::account::profile;
use crate::db::crud::core::account::student;
use crate::db::crud::notifications::get_device_tokens;
use crate::notifications::messages::NotificationType;
use crate::notifications::telegram::TelegramProvider;
use crate::services::AppError;
use crate::services::notifications::apns::ApnsProvider;

use sqlx::PgPool;
use tracing::info;

pub mod apns;
pub mod error;
pub mod messages;
pub mod telegram;

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
    ) -> Result<(), AppError> {
        info!(
            "Teacher {} notifying student {}: {:?}",
            teacher_id, student_id, notification_type
        );

        self.send_apns_notifications(student_id, &notification_type)
            .await?;

        if let Ok(Some(telegram_id)) =
            student::get_telegram_id(&self.db, teacher_id, student_id).await
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
    ) -> Result<(), AppError> {
        info!(
            "Student {} notifying teacher: {:?}",
            student_id, notification_type
        );

        if let Ok(Some(teacher_id)) = profile::get_teacher_user_id(&self.db, student_id).await {
            self.send_apns_notifications(&teacher_id, &notification_type)
                .await?;
        }

        if let Ok(Some(telegram_id)) = profile::get_teacher_telegram_id(&self.db, student_id).await
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
    ) -> Result<(), AppError> {
        if let Ok(device_tokens) = get_device_tokens(&self.db, recipient_id).await {
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
