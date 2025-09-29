use ogonek_db::init_db;
use ogonek_notifications::NotificationService;
use ogonek_s3::S3Provider;
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
