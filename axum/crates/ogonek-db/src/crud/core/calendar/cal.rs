use sqlx::PgPool;

use crate::DbError;

use ogonek_types::{CalendarFull, CalendarUpdate};
/// Finds the calendar by user id
pub async fn get_or_create(db: &PgPool, user_id: &str) -> Result<CalendarFull, DbError> {
    let calendar = sqlx::query_as!(
        CalendarFull,
        r#"
        INSERT INTO calendars (id, name, owner_id)
        VALUES ($1, $2, $3)
        ON CONFLICT (owner_id) 
        DO UPDATE SET 
            name = EXCLUDED.name,
            updated_at = CURRENT_TIMESTAMP
        RETURNING *
        "#,
        nanoid::nanoid!(),
        "My CalendarFull",
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(calendar)
}
pub async fn read_calendar_id(
    db: impl sqlx::Executor<'_, Database = sqlx::Postgres>,
    user_id: &str,
) -> Result<String, DbError> {
    let calendar_id = sqlx::query_scalar!(
        r#"
        INSERT INTO calendars (id, name, owner_id)
        VALUES ($1, $2, $3)
        ON CONFLICT (owner_id) 
        DO UPDATE SET owner_id = EXCLUDED.owner_id
        RETURNING id
        "#,
        nanoid::nanoid!(),
        "My CalendarFull",
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(calendar_id)
}

/// Deletes a cal
pub async fn delete(db: &PgPool, calendar_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
    DELETE
      FROM calendars
      WHERE owner_id = $2 AND id = $1
      "#,
        calendar_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Updates the calendar in the DB
pub async fn update(
    db: &PgPool,
    calendar_id: &str,
    user_id: &str,
    update: &CalendarUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE calendars 
         SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            colour = COALESCE($3, colour),
            timezone = COALESCE($4, timezone),
            caldav_url = COALESCE($5, caldav_url),
            sync_token = COALESCE($6, sync_token)
         WHERE id = $7 AND owner_id = $8
",
        update.name,
        update.description,
        update.colour,
        update.timezone,
        update.caldav_url,
        update.sync_token,
        calendar_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}
