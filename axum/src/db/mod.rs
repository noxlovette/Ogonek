pub mod crud;
pub mod error;
pub mod helpers;

use anyhow::Context;
use dotenvy::dotenv;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn init_db() -> anyhow::Result<PgPool> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    MIGRATOR
        .run(&db)
        .await
        .context("Failed to run DB migrations")?;

    tracing::info!("Database migrations completed successfully");

    Ok(db)
}
