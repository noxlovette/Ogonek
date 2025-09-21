use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;

use crate::{
    db::{
        crud::core::{
            account::{
                profile::get_call_url,
                user::{get_email, get_name},
            },
            calendar::{calendar::read_calendar_id, event_attendee},
        },
        error::DbError,
    },
    services::calendar::{RRule, extract_occurrence_from_id},
    types::{
        EditScope, EventAttendeeCreate, EventClass, EventCreate, EventDB, EventFull, EventStatus,
        EventTransp, EventUpdate,
    },
};
pub async fn update(db: &PgPool, id: &str, update: &EventUpdate) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    // Handle attendee updates and get the name if changed
    let attendee_name = handle_attendee_update(&mut tx, id, &update.attendee).await?;

    // Update the main event
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
        id,
        attendee_name,
        update.description,
        update.location,
        update.dtstart,
        update.dtend,
        update.timezone,
        update.rrule,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

async fn handle_attendee_update(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event_id: &str,
    new_attendee_id: &Option<String>,
) -> Result<Option<String>, DbError> {
    if let Some(new_attendee_id) = new_attendee_id {
        // Check if attendee is different from current one
        let current_attendee = sqlx::query_scalar!(
            "SELECT user_id FROM event_attendees WHERE event_id = $1",
            event_id,
        )
        .fetch_optional(&mut **tx)
        .await?;

        // Only update if attendee is different or none exists
        if current_attendee.as_ref() != Some(new_attendee_id) {
            // Delete existing attendee if any
            if current_attendee.is_some() {
                sqlx::query!("DELETE FROM event_attendees WHERE event_id = $1", event_id)
                    .execute(&mut **tx)
                    .await?;
            }

            // Create new attendee
            let attendee_email = get_email(&mut **tx, new_attendee_id).await?;
            let attendee_name = get_name(&mut **tx, new_attendee_id).await?;
            let attendee_payload = EventAttendeeCreate {
                email: attendee_email,
                name: Some(attendee_name.clone()),
            };

            event_attendee::create(&mut **tx, event_id, new_attendee_id, attendee_payload).await?;
            return Ok(Some(attendee_name));
        }
    }
    Ok(None)
}
/// Updates an event. Supports recursion with proper error handling
async fn edit_recurring_event(db: &PgPool, req: EventUpdate) -> Result<(), DbError> {
    let mut tx = db.begin().await?;
    let master = sqlx::query_as!(
        EventFull,
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
        WHERE uid = (
            SELECT uid FROM calendar_events WHERE id = $1
        ) AND recurrence_id IS NULL"#,
        req.event_id
    )
    .fetch_one(db)
    .await?;

    // Extract occurrence date - this should always exist for recurring events
    let occurrence_date =
        extract_occurrence_from_id(&req.event_id).ok_or(DbError::InvalidRecurrenceId)?;

    match req.edit_scope {
        EditScope::ThisOnly => {
            edit_single_occurrence(&master, occurrence_date, &req, &mut tx).await?;
        }
        EditScope::ThisAndFuture => {
            edit_this_and_future(&master, occurrence_date, &req, &mut tx).await?;
        }
        EditScope::AllEvents => {
            edit_all_events(&master, &req, &mut tx).await?;
        }
    }

    tx.commit().await?;
    Ok(())
}

async fn edit_single_occurrence(
    master: &EventFull,
    occurrence_date: DateTime<Utc>,
    updates: &EventUpdate,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), DbError> {
    // Check if exception already exists
    let existing_exception = sqlx::query_scalar!(
        "SELECT id FROM calendar_events 
         WHERE uid = $1 AND recurrence_id = $2",
        master.uid,
        occurrence_date
    )
    .fetch_optional(&mut **tx)
    .await?;

    // Calculate new times with better duration handling
    let new_start = updates.dtstart.unwrap_or(occurrence_date);
    let duration = master
        .dtend
        .map(|end| end - master.dtstart)
        .unwrap_or(Duration::hours(1)); // Default 1hr if no end time
    let new_end = updates.dtend.or_else(|| Some(new_start + duration));

    if let Some(exception_id) = existing_exception {
        // Update existing exception
        sqlx::query!(
            r#"
            UPDATE calendar_events 
            SET summary = COALESCE($1, summary),
                dtstart = COALESCE($2, dtstart),
                dtend = COALESCE($3, dtend),
                location = COALESCE($4, location),
                updated_at = NOW(),
                sequence = sequence + 1,
                etag = encode(sha256(random()::text::bytea), 'hex')
            WHERE id = $5
            "#,
            updates.summary,
            new_start,
            new_end,
            updates.location,
            exception_id
        )
        .execute(&mut **tx)
        .await?;
    } else {
        // Create new exception - this is the key part for iCal compliance
        sqlx::query!(
            r#"
            INSERT INTO calendar_events (
                id, uid, calendar_id, summary, description, location,
                dtstart, dtend, all_day, timezone, recurrence_id,
                status, organiser_email, organiser_name, sequence
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, 1
            )
            "#,
            nanoid::nanoid!(),
            master.uid,
            master.calendar_id,
            updates.summary.as_ref().unwrap_or(&master.summary),
            master.description.as_ref(),
            updates.location.as_ref().or(master.location.as_ref()),
            new_start,
            new_end,
            master.all_day,
            master.timezone.as_ref(),
            occurrence_date, // This makes it an exception!
            master.status.as_ref(),
            master.organiser_email.as_ref(),
            master.organiser_name.as_ref()
        )
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

/// Splits a recurring series at the given occurrence
async fn edit_this_and_future(
    master: &EventFull,
    occurrence_date: DateTime<Utc>,
    updates: &EventUpdate,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), DbError> {
    // Parse the RRULE to understand the recurrence pattern
    let rrule_str = master.rrule.as_ref().ok_or(DbError::NotRecurring)?;

    // 1. Truncate original master event with UNTIL
    let until_date = occurrence_date - Duration::seconds(1);
    let truncated_rrule = if rrule_str.contains("UNTIL=") {
        // Replace existing UNTIL
        let re = regex::Regex::new(r"UNTIL=[^;]*").unwrap();
        re.replace(
            rrule_str,
            &format!("UNTIL={}", until_date.format("%Y%m%dT%H%M%SZ")),
        )
        .to_string()
    } else {
        // Add UNTIL
        format!(
            "{};UNTIL={}",
            rrule_str,
            until_date.format("%Y%m%dT%H%M%SZ")
        )
    };

    sqlx::query!(
        r#"
        UPDATE calendar_events 
        SET rrule = $1, 
            updated_at = NOW(),
            sequence = sequence + 1,
            etag = encode(sha256(random()::text::bytea), 'hex')
        WHERE id = $2
        "#,
        truncated_rrule,
        master.id
    )
    .execute(&mut **tx)
    .await?;

    // 2. Create new master for this and future occurrences
    let new_start = updates.dtstart.unwrap_or(occurrence_date);
    let duration = master
        .dtend
        .map(|end| end - master.dtstart)
        .unwrap_or(Duration::hours(1));
    let new_end = updates.dtend.or_else(|| Some(new_start + duration));

    // Clean the RRULE - remove UNTIL since this is a new open-ended series
    let new_rrule = remove_until_from_rrule(rrule_str);

    sqlx::query!(
        r#"
        INSERT INTO calendar_events (
            id, uid, calendar_id, summary, description, location,
            dtstart, dtend, all_day, timezone, rrule,
            status, organiser_email, organiser_name, sequence
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, 1
        )
        "#,
        nanoid::nanoid!(),
        nanoid::nanoid!(), // New UID for the split series
        master.calendar_id,
        updates.summary.as_ref().unwrap_or(&master.summary),
        master.description.as_ref(),
        updates.location.as_ref().or(master.location.as_ref()),
        new_start,
        new_end,
        master.all_day,
        master.timezone.as_ref(),
        new_rrule,
        master.status.as_ref(),
        master.organiser_email.as_ref(),
        master.organiser_name.as_ref()
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn edit_all_events(
    master: &EventFull,
    updates: &EventUpdate,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), DbError> {
    // Update the master event - all instances inherit these changes
    sqlx::query!(
        r#"
        UPDATE calendar_events 
        SET summary = COALESCE($1, summary),
            location = COALESCE($2, location),
            updated_at = NOW(),
            sequence = sequence + 1,
            etag = encode(sha256(random()::text::bytea), 'hex')
        WHERE id = $3
        "#,
        updates.summary,
        updates.location,
        master.id
    )
    .execute(&mut **tx)
    .await?;

    // Also update any existing exceptions to maintain consistency
    if updates.summary.is_some() || updates.location.is_some() {
        sqlx::query!(
            r#"
            UPDATE calendar_events 
            SET summary = COALESCE($1, summary),
                location = COALESCE($2, location),
                updated_at = NOW(),
                sequence = sequence + 1,
                etag = encode(sha256(random()::text::bytea), 'hex')
            WHERE uid = $3 AND recurrence_id IS NOT NULL
            "#,
            updates.summary,
            updates.location,
            master.uid
        )
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}
