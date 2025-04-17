use anyhow::Context;
use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn init_db() -> anyhow::Result<PgPool> {
    dotenv().ok();
    let database_url = if std::env::var("APP_ENV").unwrap_or_default() != "development" {
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    } else {
        "postgres://postgres:changeme@postgres:5432/pg-ogonek-dev".to_string()
    };

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    Ok(db)
}
