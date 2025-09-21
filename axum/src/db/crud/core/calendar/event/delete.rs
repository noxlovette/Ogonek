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
    types::{
        EventAttendeeCreate, EventClass, EventCreate, EventDB, EventFull, EventStatus, EventTransp,
        EventUpdate,
    },
};
#[derive(Deserialize)]
enum DeleteScope {
    ThisOnly,
    ThisAndFuture,
    AllEvents,
}

pub async fn delete(
    event_id: String,
    occurrence_date: DateTime<Utc>,
    scope: DeleteScope,
    pool: &PgPool,
) -> Result<(), DbError> {
    match scope {
        DeleteScope::ThisOnly => {
            // Add to exdate array
            sqlx::query!(
                r#"
                UPDATE calendar_events 
                SET exdate = array_append(COALESCE(exdate, '{}'), $1),
                    updated_at = NOW()
                WHERE uid = (SELECT uid FROM calendar_events WHERE id = $2)
                  AND recurrence_id IS NULL
                "#,
                occurrence_date.to_rfc3339(),
                event_id
            )
            .execute(pool)
            .await?;
        }
        DeleteScope::ThisAndFuture => {
            // Same logic as edit_this_and_future but without creating new master
            let master = get_master_event(&event_id, pool).await?;
            let truncated_rrule = format!(
                "{};UNTIL={}",
                master.rrule.unwrap(),
                (occurrence_date - Duration::seconds(1)).format("%Y%m%dT%H%M%SZ")
            );

            sqlx::query!(
                "UPDATE calendar_events SET rrule = $1 WHERE id = $2",
                truncated_rrule,
                master.id
            )
            .execute(pool)
            .await?;
        }
        DeleteScope::AllEvents => {
            // Soft delete master
            sqlx::query!(
                "UPDATE calendar_events SET deleted_at = NOW() WHERE id = $1",
                event_id
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
