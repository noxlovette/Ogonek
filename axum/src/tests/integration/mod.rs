use sqlx::PgPool;
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};

pub mod user;

pub async fn test_db() -> anyhow::Result<PgPool> {
    let container = postgres::Postgres::default().start().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let connection_string =
        &format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres",);

    let pool = PgPool::connect(connection_string)
        .await
        .expect("Failed to connect to test database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    Ok(pool)
}
