use crate::db::init_db;
use crate::notifications::NotificationService;
use crate::s3::S3Provider;
use sqlx::postgres::PgPool;
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
    pub s3: S3Provider,
    pub notification_service: NotificationService,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let db = init_db().await?;
        let s3 = S3Provider::new().await?;

        let notification_service = NotificationService::new(db.clone())?;

        Ok(Self {
            db: db.clone(),
            s3,
            notification_service,
        })
    }
}

#[cfg(test)]
use crate::tests::integration::test_db;

impl AppState {
    #[cfg(test)]
    pub async fn test() -> anyhow::Result<Self> {
        let db = test_db().await?;
        let s3 = S3Provider::test().await?;

        let notification_service = NotificationService::test(db.clone())?;

        Ok(Self {
            db: db.clone(),
            s3,
            notification_service,
        })
    }
}
