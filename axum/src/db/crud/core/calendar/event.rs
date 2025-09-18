use sqlx::PgPool;

use crate::{
    db::error::DbError,
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

/// Finds all events for a calendar
pub async fn find_by_calendar_id(
    db: &PgPool,
    calendar_id: &str,
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
        WHERE calendar_id = $1 AND deleted_at IS NULL
        ORDER BY dtstart ASC
        "#,
        calendar_id
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
    uid: &str,
    calendar_id: &str,
    create: CalendarEventCreate,
) -> Result<String, DbError> {
    let attendee_name = sqlx::query_scalar!(
        r#"
        SELECT name FROM "user" WHERE id = $1
        "#,
        create.attendee
    )
    .fetch_one(db)
    .await?;

    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO calendar_events (
            id, calendar_id, uid, summary, dtstart, dtend
        )
        VALUES (
            $1, $2, $3, $4, $5, $6
        )
        RETURNING id
        "#,
        nanoid::nanoid!(),
        calendar_id,
        uid,
        attendee_name,
        create.dtstart,
        create.dtend,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}
