use sqlx::PgPool;

use crate::{
    db::error::DbError,
    types::{Calendar, CalendarCreate, CalendarUpdate},
};
/// Finds the calendar by id
pub async fn find_by_id(
    db: &PgPool,
    calendar_id: &str,
    user_id: &str,
) -> Result<Calendar, DbError> {
    let calendar = sqlx::query_as!(
        Calendar,
        r#"
      SELECT *
      FROM calendars
      WHERE id = $1 AND owner_id = $2
      "#,
        calendar_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(calendar)
}

/// Finds all calendars for the user
pub async fn find_all(db: &PgPool, user_id: &str) -> Result<Vec<Calendar>, DbError> {
    let calendars = sqlx::query_as!(
        Calendar,
        r#"
      SELECT *
      FROM calendars
      WHERE owner_id = $1
      "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(calendars)
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

/// Creates a calendar entry. The initial request should contain the name for it
pub async fn create(db: &PgPool, user_id: &str, create: CalendarCreate) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO calendars (id, name, owner_id)
         VALUES ($1, $2, $3)
         RETURNING id",
        nanoid::nanoid!(),
        create.name,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}
