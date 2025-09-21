use sqlx::PgPool;

use crate::{
    db::{
        crud::core::calendar::event::{read_one, truncate_master},
        error::DbError,
    },
    types::{DeleteScope, EventDelete},
};

pub async fn delete(db: &PgPool, event_id: String, req: EventDelete) -> Result<(), DbError> {
    let mut tx = db.begin().await?;

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
                req.occurrence_date.to_rfc3339(),
                event_id
            )
            .execute(&mut *tx)
            .await?;
        }
        DeleteScope::ThisAndFuture => {
            let master = read_one(&mut *tx, &event_id).await?;

            let _ = truncate_master(&mut tx, &master, &req.occurrence_date).await?;
        }
    }
    Ok(())
}
