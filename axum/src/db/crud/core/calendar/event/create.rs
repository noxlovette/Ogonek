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
    types::{EventAttendeeCreate, EventCreate, EventFull, EventUpdate},
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

pub async fn create_exception(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    master: &EventFull,
    update: &EventUpdate,
    occurrence_date: &DateTime<Utc>,
) -> Result<(), DbError> {
    sqlx::query!(
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
                all_day,
                dtstart_date,
                dtend_date, 
                recurrence_id
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
            "#,
        nanoid::nanoid!(),
        master.uid,
        master.calendar_id,
        master.summary,
        master.description,
        update.location.as_ref().or(master.location.as_ref()),
        update.dtstart_time,
        update.dtend_time,
        master.all_day,
        master.dtstart_date.as_ref(),
        master.dtend_date.as_ref(),
        occurrence_date
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn create_master(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    master: &EventFull,
    update: &EventUpdate,
    new_rrule: &str,
) -> Result<(), DbError> {
    sqlx::query!(
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
                all_day,
                dtstart_date,
                dtend_date ,
                rrule
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
        "#,
        nanoid::nanoid!(),
        nanoid::nanoid!(),
        master.calendar_id,
        master.summary,
        master.description,
        update.location.as_ref().or(master.location.as_ref()),
        update.dtstart_time,
        update.dtend_time,
        master.all_day,
        master.dtstart_date.as_ref(),
        master.dtend_date.as_ref(),
        new_rrule
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
