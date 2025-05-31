use crate::db::init::init_db;
use crate::s3::init::init_s3;
use aws_sdk_s3::Client as S3Client;
use sqlx::postgres::PgPool;
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
    pub s3: S3Client,
    pub bucket_name: String,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let bucket_name = std::env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME needs to be set");

        let db = init_db().await?;
        let s3 = init_s3().await?;

        Ok(Self {
            db: db.clone(),
            s3,
            bucket_name,
        })
    }
}
