use sqlx::PgPool;

use crate::{
    DbError,
    crud::core::calendar::event::{read::read_one_internal, update::truncate_master},
    helpers::extract_id_and_occurence,
};

use ogonek_types::{DeleteScope, EventDelete};
pub async fn delete(db: &PgPool, event_id: String, req: EventDelete) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

    let (master_id, occurrence_date) = extract_id_and_occurence(event_id);

    if let Some(occurrence) = occurrence_date {
        match req.scope {
            DeleteScope::ThisOnly => {
                sqlx::query!(
                    r#"
                UPDATE calendar_events 
                SET exdate = array_append(COALESCE(exdate, '{}'), $1),
                    updated_at = NOW()
                WHERE uid = (SELECT uid FROM calendar_events WHERE id = $2)
                  AND recurrence_id IS NULL
                "#,
                    occurrence.to_rfc3339(),
                    master_id
                )
                .execute(&mut *tx)
                .await?;
            }
            DeleteScope::ThisAndFuture => {
                let master = read_one_internal(&mut *tx, &master_id).await?;

                // DELETE ALL EXCEPTIONS TOO!
                sqlx::query!(
                    r#"
                    DELETE FROM calendar_events 
                    WHERE uid = $1 
                    AND recurrence_id IS NOT NULL 
                    AND recurrence_id >= $2
                    "#,
                    master.uid,
                    occurrence
                )
                .execute(&mut *tx)
                .await?;

                let _ = truncate_master(&mut tx, &master, &occurrence).await?;
            }
        }
    } else {
        let existing_exception = sqlx::query_scalar!(
            "SELECT id FROM calendar_events WHERE id = $1 AND recurrence_id IS NOT NULL",
            master_id.clone()
        )
        .fetch_optional(&mut *tx)
        .await?;
        if existing_exception.is_some() {
            // Get the exception's recurrence_id before deleting
            let exception_recurrence_id = sqlx::query_scalar!(
                "SELECT recurrence_id FROM calendar_events WHERE id = $1",
                master_id
            )
            .fetch_one(&mut *tx)
            .await?;

            // Delete the existing exception instance
            sqlx::query!("DELETE FROM calendar_events WHERE id = $1", master_id)
                .execute(&mut *tx)
                .await?;

            // Remove the occurrence from master's exdate array so it shows up again
            if let Some(recurrence_date) = exception_recurrence_id {
                sqlx::query!(
                    r#"
                    UPDATE calendar_events 
                    SET exdate = array_remove(COALESCE(exdate, '{}'), $1),
                        updated_at = NOW()
                    WHERE uid = (SELECT uid FROM calendar_events WHERE id = $2 AND recurrence_id IS NULL)
                    "#,
                    recurrence_date.to_rfc3339(),
                    master_id
                )
                .execute(&mut *tx)
                .await?;
            }
        }
        sqlx::query!(
            r#"
            UPDATE calendar_events
            SET deleted_at = NOW()
            WHERE uid = (SELECT uid FROM calendar_events WHERE id = $1 AND recurrence_id IS NULL) 
            "#,
            master_id
        )
        .execute(&mut *tx)
        .await?;
    };

    tx.commit().await?;
    Ok(())
}
