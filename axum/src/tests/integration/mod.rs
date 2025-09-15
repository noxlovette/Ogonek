use sqlx::PgPool;

pub mod user;

/// we might want to use testcontainers to make this work independently actually...
pub async fn test_db() -> anyhow::Result<PgPool> {
    dotenvy::dotenv().ok();
    // Use existing postgres container for tests
    let connection_string =
        std::env::var("DATABASE_URL").expect("DATABASE URL MUST BE SET FOR TESTS");

    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    Ok(pool)
}
