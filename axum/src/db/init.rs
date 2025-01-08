use anyhow::Context;
use dotenvy::dotenv;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn init_db() -> anyhow::Result<PgPool> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    Ok(db)
}
