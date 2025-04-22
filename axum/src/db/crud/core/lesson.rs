use crate::db::error::DbError;
use crate::models::{
    CreationId, LessonCreate, LessonSmall, LessonUpdate, LessonWithStudent, PaginationParams,
};
use sqlx::PgPool;

pub async fn find_all(
    db: &PgPool,
    user_id: &str,
    params: &PaginationParams,
) -> Result<Vec<LessonWithStudent>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT l.id, l.title, l.topic, l.markdown, l.assignee, l.created_by, l.created_at, l.updated_at, u.name as assignee_name
         FROM lessons l
         LEFT JOIN \"user\" u ON l.assignee = u.id
         WHERE (l.assignee = ");

    query_builder.push_bind(user_id);
    query_builder.push(" OR l.created_by = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    if let Some(search) = &params.search {
        query_builder.push(" AND (l.title ILIKE ");
        query_builder.push_bind(format!("%{}%", search));
        query_builder.push(" OR l.topic ILIKE ");
        query_builder.push_bind(format!("%{}%", search));
        query_builder.push(" OR l.markdown ILIKE ");
        query_builder.push_bind(format!("%{}%", search));
        query_builder.push(")");
    }

    // search by assignee if provided
    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND l.assignee = ");
        query_builder.push_bind(assignee);
    }

    // ordering
    query_builder.push(" ORDER BY l.created_at DESC");

    query_builder.push(" LIMIT ");
    query_builder.push_bind(params.limit());
    query_builder.push(" OFFSET ");
    query_builder.push_bind(params.offset());

    let lessons = query_builder
        .build_query_as::<LessonWithStudent>()
        .fetch_all(db)
        .await?;

    Ok(lessons)
}

pub async fn find_recent(db: &PgPool, user_id: &str) -> Result<Vec<LessonSmall>, DbError> {
    let lessons = sqlx::query_as!(
        LessonSmall,
        r#"
        SELECT
            id,
            title,
            LEFT(markdown, 100) as "markdown!",
            topic,
            created_at
        FROM lessons
        WHERE (assignee = $1 OR created_by = $1)
        ORDER BY created_at DESC
        LIMIT 6
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(lessons)
}

pub async fn find_by_id(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
) -> Result<LessonWithStudent, DbError> {
    let lesson = sqlx::query_as!(
        LessonWithStudent,
        r#"
        SELECT
            l.id,
            l.title,
            l.topic,
            l.markdown,
            l.assignee,
            l.created_by,
            l.created_at,
            l.updated_at,
            u.name as assignee_name
        FROM lessons l
        LEFT JOIN "user" u ON l.assignee = u.id
        WHERE l.id = $1
        AND (l.assignee = $2 OR l.created_by = $2)
        "#,
        lesson_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(lesson)
}

pub async fn create(
    db: &PgPool,
    user_id: &str,
    create: LessonCreate,
) -> Result<CreationId, DbError> {
    let mut assignee = user_id;

    if create.assignee.is_some() {
        assignee = create.assignee.as_ref().unwrap();
    }

    let id = sqlx::query_as!(
        CreationId,
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

pub async fn delete(db: &PgPool, lesson_id: &str, user_id: &str) -> Result<(), DbError> {
    sqlx::query!(
        r#"
        DELETE FROM lessons
        WHERE id = $1 AND created_by = $2
        "#,
        lesson_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
    update: LessonUpdate,
) -> Result<(), DbError> {
    sqlx::query!(
        "UPDATE lessons
         SET
            title = COALESCE($1, title),
            topic =COALESCE($2, topic),
            markdown = COALESCE($3, markdown),
            assignee = COALESCE($4, assignee)
         WHERE id = $5 AND created_by = $6
",
        update.title,
        update.topic,
        update.markdown,
        update.assignee,
        lesson_id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}
pub async fn count(db: &PgPool, user_id: &str) -> Result<i64, DbError> {
    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM lessons WHERE
            (created_by = $1 OR assignee = $1)
            ",
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(count.unwrap_or(0))
}
