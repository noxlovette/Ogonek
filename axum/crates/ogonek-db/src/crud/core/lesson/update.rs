use ogonek_types::LessonUpdate;
use sqlx::PgPool;

use crate::DbError;

pub async fn update(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
    update: &LessonUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE lessons
         SET
            title = COALESCE($1, title),
            topic = COALESCE($2, topic),
            markdown = COALESCE($3, markdown),
            assignee = CASE
            WHEN $7 = true THEN NULL
            ELSE 
            COALESCE($4, assignee)
            END
         WHERE id = $5 AND created_by = $6
",
        update.title,
        update.topic,
        update.markdown,
        update.assignee,
        lesson_id,
        user_id,
        update.unassign
    )
    .execute(db)
    .await?;

    Ok(())
}
