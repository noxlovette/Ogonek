use sqlx::PgPool;

use crate::{
    db::error::DbError,
    types::{EventAttendee, EventAttendeeCreate, EventAttendeeUpdate},
};

/// Finds an event attendee by id
pub async fn find_by_id(db: &PgPool, attendee_id: &str) -> Result<EventAttendee, DbError> {
    let attendee = sqlx::query_as!(
        EventAttendee,
        r#"
        SELECT 
            id,
            event_id,
            email,
            name,
            role as "role: _",
            status as "status: _",
            rsvp,
            created_at,
            updated_at
        FROM event_attendees
        WHERE id = $1
        "#,
        attendee_id
    )
    .fetch_one(db)
    .await?;

    Ok(attendee)
}

/// Finds all attendees for an event
pub async fn find_by_event_id(db: &PgPool, event_id: &str) -> Result<Vec<EventAttendee>, DbError> {
    let attendees = sqlx::query_as!(
        EventAttendee,
        r#"
        SELECT 
            id,
            event_id,
            email,
            name,
            role as "role: _",
            status as "status: _",
            rsvp,
            created_at,
            updated_at
        FROM event_attendees
        WHERE event_id = $1
        "#,
        event_id
    )
    .fetch_all(db)
    .await?;

    Ok(attendees)
}

/// Deletes an event attendee
pub async fn delete(db: &PgPool, attendee_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM event_attendees
        WHERE id = $1
        "#,
        attendee_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Updates an event attendee
pub async fn update(
    db: &PgPool,
    attendee_id: &str,
    update: &EventAttendeeUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        UPDATE event_attendees 
        SET
            email = COALESCE($1, email),
            name = COALESCE($2, name),
            role = COALESCE($3, role),
            status = COALESCE($4, status),
            rsvp = COALESCE($5, rsvp)
        WHERE id = $6
        "#,
        update.email,
        update.name,
        update.role.as_ref().map(|r| serde_json::to_string(r).unwrap().trim_matches('"').to_string()),
        update.status.as_ref().map(|s| serde_json::to_string(s).unwrap().trim_matches('"').to_string()),
        update.rsvp,
        attendee_id
    )
    .execute(db)
    .await?;

    Ok(())
}

/// Creates an event attendee
pub async fn create(db: &PgPool, create: EventAttendeeCreate) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO event_attendees (id, event_id, email, name, role, status, rsvp)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
        nanoid::nanoid!(),
        create.event_id,
        create.email,
        create.name,
        serde_json::to_string(&create.role).unwrap().trim_matches('"'),
        serde_json::to_string(&create.status).unwrap().trim_matches('"'),
        create.rsvp,
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}