use sqlx::PgPool;

use crate::{
    db::{
        crud::core::{
            account::{profile::get_call_url, user::get_name},
            calendar::calendar::get_calendar_id,
        },
        error::DbError,
    },
    types::{
        CalendarEvent, CalendarEventCreate, CalendarEventUpdate, EventClass, EventStatus,
        EventTransp,
    },
};

/// Finds a calendar event by uid
pub async fn find_by_uid(db: &PgPool, event_uid: &str) -> Result<CalendarEvent, DbError> {
    let event = sqlx::query_as!(
        CalendarEvent,
        r#"
        SELECT 
            id,
            uid,
            created_at,
            updated_at,
            calendar_id,
            summary,
            description,
            location,
            url,
            dtstart,
            dtend,
            all_day,
            timezone,
            rrule,
            rdate,
            exdate,
            recurrence_id,
            status as "status: EventStatus",
            class as "class: EventClass",
            transp as "transp: EventTransp",
            priority,
            categories,
            organiser_email,
            organiser_name,
            sequence,
            dtstamp,
            etag,
            deleted_at
        FROM calendar_events
        WHERE uid = $1 AND deleted_at IS NULL
        "#,
        event_uid
    )
    .fetch_one(db)
    .await?;

    Ok(event)
}

/// Finds all events for a calendar within a specific month
pub async fn find_by_calendar_id_and_month(
    db: &PgPool,
    calendar_id: &str,
    year: i32,
    month: u32,
) -> Result<Vec<CalendarEvent>, DbError> {
    let events = sqlx::query_as!(
        CalendarEvent,
        r#"
        SELECT 
            id,
            uid,
            created_at,
            updated_at,
            calendar_id,
            summary,
            description,
            location,
            url,
            dtstart,
            dtend,
            all_day,
            timezone,
            rrule,
            rdate,
            exdate,
            recurrence_id,
            status as "status: EventStatus",
            class as "class: EventClass",
            transp as "transp: EventTransp",
            priority,
            categories,
            organiser_email,
            organiser_name,
            sequence,
            dtstamp,
            etag,
            deleted_at
        FROM calendar_events
        WHERE calendar_id = $1 
            AND deleted_at IS NULL
            AND (
                -- Events that start in the target month
                (dtstart >= DATE_TRUNC('month', MAKE_DATE($2, $3, 1))
                AND dtstart < DATE_TRUNC('month', MAKE_DATE($2, $3, 1)) + INTERVAL '1 month')
                OR
                -- Multi-day events that overlap with the target month
                (dtend IS NOT NULL 
                AND dtstart < DATE_TRUNC('month', MAKE_DATE($2, $3, 1)) + INTERVAL '1 month'
                AND dtend >= DATE_TRUNC('month', MAKE_DATE($2, $3, 1)))
            )
        ORDER BY dtstart ASC
        "#,
        calendar_id,
        year,
        month as i32
    )
    .fetch_all(db)
    .await?;
    Ok(events)
}
pub async fn find_by_date(
    db: &PgPool,
    day: chrono::NaiveDate,
) -> Result<Vec<CalendarEvent>, DbError> {
    let events = sqlx::query_as!(
        CalendarEvent,
        r#"
        SELECT 
            id, uid, created_at, updated_at, calendar_id,
            summary, description, location, url,
            dtstart, dtend, all_day, timezone,
            rrule, rdate, exdate, recurrence_id,
            status as "status: EventStatus",
            class as "class: EventClass", 
            transp as "transp: EventTransp",
            priority, categories,
            organiser_email, organiser_name,
            sequence, dtstamp, etag, deleted_at
        FROM calendar_events
        WHERE deleted_at IS NULL
        AND (
            dtstart::date = $1 OR 
            dtend::date = $1 OR
            (dtstart::date <= $1 AND dtend::date >= $1)
        )
        ORDER BY dtstart ASC
        "#,
        day,
    )
    .fetch_all(db)
    .await?;
    Ok(events)
}

/// Soft deletes a calendar event (sets deleted_at timestamp)
pub async fn delete(db: &PgPool, event_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE calendar_events
        SET deleted_at = NOW()
        WHERE id = $1
        "#,
        event_id
    )
    .execute(db)
    .await?;

    Ok(())
}
pub async fn update(
    db: &PgPool,
    event_id: &str,
    update: &CalendarEventUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE calendar_events 
        SET
            summary = COALESCE($2, summary),
            description = COALESCE($3, description),
            location = COALESCE($4, location),
            dtstart = COALESCE($5, dtstart),
            dtend = COALESCE($6, dtend),
            timezone = COALESCE($7, timezone),
            rrule = COALESCE($8, rrule),
            updated_at = NOW()
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        event_id,
        update.summary,
        update.description,
        update.location,
        update.dtstart,
        update.dtend,
        update.timezone,
        update.rrule,
    )
    .execute(db)
    .await?;
    Ok(())
}

/// Creates a calendar event
pub async fn create(
    db: &PgPool,
    user_id: &str,
    create: CalendarEventCreate,
) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    let calendar_id = get_calendar_id(&mut *tx, user_id).await?;

    let attendee_name = get_name(&mut *tx, user_id).await?;

    let video_call_url = get_call_url(&mut *tx, &user_id).await?;

    sqlx::query!(
        r#"
        INSERT INTO calendar_events (
            id, calendar_id, uid, summary, dtstart, dtend, location
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7
        )
        "#,
        nanoid::nanoid!(),
        calendar_id,
        nanoid::nanoid!(),
        attendee_name,
        create.dtstart,
        create.dtend,
        video_call_url
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
