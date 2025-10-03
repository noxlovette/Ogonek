use ogonek_types::LessonCreate;
use sqlx::PgPool;

use crate::DbError;

pub async fn create(db: &PgPool, user_id: &str, create: LessonCreate) -> Result<String, DbError> {
    let mut assignee = user_id;

    if create.assignee.is_some() {
        assignee = create.assignee.as_ref().unwrap();
    }

    let id = sqlx::query_scalar!(
        "INSERT INTO lessons (id, title, topic, markdown, created_by, assignee)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id",
        nanoid::nanoid!(),
        create.title,
        create.topic,
        create.markdown,
        user_id,
        assignee
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}

/// Takes user preferences to define defaults (well, it currently doesn't but you get the point)
pub async fn create_with_defaults(db: &PgPool, user_id: &str) -> Result<String, DbError> {
    let id = sqlx::query_scalar!(
        "INSERT INTO lessons (id, title, topic, markdown, created_by, assignee)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id",
        nanoid::nanoid!(),
        "Default Title",
        "Default Topic",
        "# Default Lesson",
        user_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(id)
}
