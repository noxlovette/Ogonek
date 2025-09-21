use chrono_tz::TZ_VARIANTS;
use sqlx::PgPool;

async fn seed_all_timezones(pool: &PgPool) -> Result<(), sqlx::Error> {
    println!(
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
        .await?;
    }

    println!("✅ Timezone seeding completed!");
    Ok(())
}

fn format_timezone_display(tzid: &str) -> String {
    tzid.replace("_", " ")
        .split('/')
        .collect::<Vec<&str>>()
        .join(" - ")
}
