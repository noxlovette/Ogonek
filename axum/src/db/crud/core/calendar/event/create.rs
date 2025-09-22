use chrono::{DateTime, Utc};
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
    types::{EventAttendeeCreate, EventCreate, EventDBFull, EventUpdate},
};

/// Creates a master calendar event
pub async fn create(db: &PgPool, user_id: &str, create: EventCreate) -> Result<(), DbError> {
    let mut tx = db.begin().await?;
    let calendar_id = read_calendar_id(&mut *tx, user_id).await?;

    let attendee_name = get_name(&mut *tx, &create.attendee).await?;

    let video_call_url = get_call_url(&mut *tx, &user_id).await?;

    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO calendar_events (
            id, calendar_id, uid, summary, dtstart_time, dtend_time, location
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7
        )
        RETURNING id
        "#,
        nanoid::nanoid!(),
        calendar_id,
        nanoid::nanoid!(),
        attendee_name,
        create.dtstart_time,
        create.dtend_time,
        video_call_url,
    )
    .fetch_one(&mut *tx)
    .await?;

    let attendee_email = get_email(&mut *tx, &create.attendee).await?;

    let attendee_payload = EventAttendeeCreate {
        email: attendee_email,
        name: Some(attendee_name),
    };

    event_attendee::create(&mut *tx, &id, &create.attendee, attendee_payload).await?;

    tx.commit().await?;
    Ok(())
}

pub(super) async fn create_exception(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    master: &EventDBFull,
    update: &EventUpdate,
    occurrence_date: &DateTime<Utc>,
) -> Result<(), DbError> {
    let exception_id = sqlx::query_scalar!(
        r#"
            INSERT INTO calendar_events (
                id, 
                uid, 
                calendar_id, 
                summary, 
                description, 
                location,
                dtstart_time, 
                dtend_time, 
                recurrence_id
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            RETURNING id
            "#,
        nanoid::nanoid!(),
        master.uid,
        master.calendar_id,
        master.summary,
        master.description,
        update.location.as_ref().or(master.location.as_ref()),
        update.dtstart_time,
        update.dtend_time,
        occurrence_date
    )
    .fetch_one(&mut **tx)
    .await?;

    // Copy attendees from master event
    copy_attendees_to_event(tx, &master.id, &exception_id).await?;

    Ok(())
}

pub(super) async fn create_master(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    master: &EventDBFull,
    update: &EventUpdate,
    new_rrule: &str,
) -> Result<(), DbError> {
    let new_master_id = sqlx::query_scalar!(
        r#"
        INSERT INTO calendar_events (
                id, 
                uid, 
                calendar_id, 
                summary, 
                description, 
                location,
                dtstart_time, 
                dtend_time, 
                rrule
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            RETURNING id
        "#,
        nanoid::nanoid!(),
        nanoid::nanoid!(),
        master.calendar_id,
        master.summary,
        master.description,
        update.location.as_ref().or(master.location.as_ref()),
        update.dtstart_time,
        update.dtend_time,
        new_rrule
    )
    .fetch_one(&mut **tx)
    .await?;

    // Copy attendees from original master event
    copy_attendees_to_event(tx, &master.id, &new_master_id).await?;

    Ok(())
}

/// Copy all attendees from one event to another
async fn copy_attendees_to_event(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    source_event_id: &str,
    target_event_id: &str,
) -> Result<(), DbError> {
    // Get all attendees from the source event
    let attendees = sqlx::query!(
        r#"
        SELECT user_id, email, name 
        FROM event_attendees 
        WHERE event_id = $1
        "#,
        source_event_id
    )
    .fetch_all(&mut **tx)
    .await?;

    // Copy each attendee to the target event
    for attendee in attendees {
        let attendee_payload = EventAttendeeCreate {
            email: attendee.email,
            name: attendee.name,
        };

        event_attendee::create(
            &mut **tx,
            target_event_id,
            &attendee.user_id,
            attendee_payload,
        )
        .await?;
    }

    Ok(())
}
