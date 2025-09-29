mod crud;
mod error;
mod tests;
pub use crud::*;
pub use error::DbError;

mod helpers;
use anyhow::Context;
use chrono::Offset;
use chrono_tz::TZ_VARIANTS;
use dotenvy::dotenv;
use sqlx::{
    migrate::Migrator,
    postgres::{PgPool, PgPoolOptions},
};

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn init_db() -> anyhow::Result<PgPool> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .context("Failed to connect to Postgres")?;

    // Run migrations first
    MIGRATOR
        .run(&db)
        .await
        .context("Failed to run DB migrations")?;
    tracing::info!("Database migrations completed successfully");

    // Seed timezones after migrations
    seed_all_timezones(&db)
        .await
        .context("Failed to seed timezones")?;

    Ok(db)
}

async fn seed_all_timezones(pool: &PgPool) -> anyhow::Result<()> {
    tracing::info!(
        "Seeding {} timezones from IANA database...",
        TZ_VARIANTS.len()
    );

    for tz in TZ_VARIANTS.iter() {
        let tzid = tz.name();
        let display_name = format_timezone_display(tzid);

        // Get current UTC offset
        let now = chrono::Utc::now();
        let local_time = now.with_timezone(tz);
        let offset = local_time.offset().fix().local_minus_utc();

        sqlx::query!(
            r#"
            INSERT INTO timezones (tzid, display_name, utc_offset_std)
            VALUES ($1, $2, $3)
            ON CONFLICT (tzid) DO NOTHING
            "#,
            tzid,
            display_name,
            offset
        )
        .execute(pool)
        .await
        .context("Failed to insert timezone")?;
    }

    tracing::info!("✅ Timezone seeding completed successfully!");
    Ok(())
}

fn format_timezone_display(tzid: &str) -> String {
    tzid.replace("_", " ")
        .split('/')
        .collect::<Vec<&str>>()
        .join(" - ")
}

// Optional: Helper function for manual seeding if needed
pub async fn reseed_timezones(pool: &PgPool) -> anyhow::Result<()> {
    seed_all_timezones(pool).await
}
