use crate::db::crud::core::account::profile::{get_teacher_telegram_id, get_telegram_id};
use crate::db::crud::notifications::get_device_tokens;
use crate::error::AppError;
use crate::notifications::apns::ApnsProvider;
use crate::notifications::messages::NotificationType;
use crate::notifications::telegram::TelegramProvider;

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

    pub async fn dispatch_notification(
        &self,
        sender_id: &str,
        notification_type: NotificationType,
        direction: NotificationDirection,
        db: &PgPool,
    ) -> Result<(), AppError> {
        let mut telegram_id: Option<String>;

        if direction = NotificationDirection::ToStudent {
            recipient_id = get_telegram_id(db, sender_id).await?;
        } else {
            recipient_id = get_teacher_telegram_id(db, sender_id).await?;
        }

        info!(
            "Dispatching {:?} notification to {}",
            notification_type, recipient_id
        );

        // Also send telegram if they have telegram ID
        if let Ok(telegram_id) = get_telegram_id(recipient_id).await {
            telegram::send_telegram_notification(
                &self.http_client,
                &self.bot_token,
                &telegram_id,
                &notification_type,
            )
            .await?;
        }

        Ok(())
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

        // Send APNS to student's devices
        self.send_apns_notifications(&student_id, &notification_type)
            .await?;

        // Send Telegram if student has it configured
        if let Ok(Some(telegram_id)) =
            student::get_telegram_id(&self.db, teacher_id, &student_id).await
        {
            telegram::send_telegram_notification(
                &self.http_client,
                &self.bot_token,
                &telegram_id,
                &notification_type,
            )
            .await?;
        }

        Ok(())
    }

    /// Student sends notification to their teacher
    pub async fn notify_teacher(
        &self,
        student_id: &str,
        notification_type: NotificationType,
    ) -> Result<(), AppError> {
        info!(
            "Student {} notifying teacher: {:?}",
            student_id, notification_type
        );

        // Lookup teacher
        let teacher_id = profile::get_teacher_id(&self.db, student_id).await?;

        // Send APNS to teacher's devices
        self.send_apns_notifications(&teacher_id, &notification_type)
            .await?;

        // Send Telegram if teacher has it configured
        if let Ok(Some(telegram_id)) = profile::get_teacher_telegram_id(&self.db, student_id).await
        {
            telegram::send_telegram_notification(
                &self.http_client,
                &self.bot_token,
                &telegram_id,
                &notification_type,
            )
            .await?;
        }

        Ok(())
    }

    async fn send_apns_notifications(
        &self,
        recipient_id: &str,
        notification_type: &NotificationType,
    ) -> Result<(), AppError> {
        if let Ok(device_tokens) = get_device_tokens(self.db, recipient_id).await {
            for token in device_tokens {
                let payload = notification_type.to_apns_payload();
                if let Err(e) = self.apns.send_notification(&token.token, payload).await {
                    tracing::warn!("Failed to send APNS notification: {}", e);
                }
            }
        }
    }

    async fn send_telegram_notification(
        &self,
        recipient_id: &str,
        notification_type: &NotificationType,
    ) -> Result<(), AppError> {
    }
}
