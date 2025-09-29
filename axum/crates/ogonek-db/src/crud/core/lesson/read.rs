use crate::DbError;
use ogonek_types::{LessonFull, LessonSmall, PDFData, PaginationParams};
use sqlx::PgPool;

/// Finds a list of mini-lessons (no markdown) according to passed Pagination params
pub async fn find_all(
    db: &PgPool,
    user_id: &str,
    params: &PaginationParams,
) -> Result<Vec<LessonSmall>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT l.id, l.title, l.topic, l.created_at,
               u.name as assignee_name,
               COALESCE(s.seen_at IS NOT NULL, TRUE) as seen
        FROM lessons l
        LEFT JOIN "user" u ON l.assignee = u.id
        LEFT JOIN seen_status s ON s.model_id = l.id
            AND s.user_id = "#,
    );
    query_builder.push_bind(user_id);
    query_builder.push(" AND s.model_type = ");
    query_builder.push_bind("lesson");
    query_builder.push(" WHERE (l.assignee = ");
    query_builder.push_bind(user_id);
    query_builder.push(" OR l.created_by = ");
    query_builder.push_bind(user_id);
    query_builder.push(")");

    if let Some(search) = &params.search {
        query_builder.push(" AND (l.title ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR l.topic ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR l.markdown ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
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
        .build_query_as::<LessonSmall>()
        .fetch_all(db)
        .await?;

    Ok(lessons)
}

/// Returns three lessons in mini-format
pub async fn find_recent(db: &PgPool, user_id: &str) -> Result<Vec<LessonSmall>, DbError> {
    let lessons = sqlx::query_as!(
        LessonSmall,
        r#"
        SELECT l.id, l.title, l.topic, l.created_at,
               u.name as assignee_name,
               COALESCE(s.seen_at IS NOT NULL, TRUE) as seen
        FROM lessons l
        LEFT JOIN "user" u ON l.assignee = u.id
        LEFT JOIN seen_status s ON s.model_id = l.id AND s.user_id = $1
        WHERE (assignee = $1 OR created_by = $1)
        ORDER BY created_at DESC
        LIMIT 3
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(lessons)
}

/// Finds one lesson by its id, will return null if the user doesn't have access to the data
pub async fn find_by_id(
    db: &PgPool,
    lesson_id: &str,
    user_id: &str,
) -> Result<LessonFull, DbError> {
    let lesson = sqlx::query_as!(
        LessonFull,
        r#"
        SELECT
            l.id,
            l.title,
            l.topic,
            l.markdown,
            l.assignee,
            l.created_at,
            l.photo_id,
            l.updated_at,
            u.name as assignee_name
        FROM lessons l
        LEFT JOIN "user" u ON l.assignee = u.id
        LEFT JOIN photos p on l.photo_id = p.id
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

/// Finds the markdown and the topic
pub async fn read_for_pdf(db: &PgPool, task_id: &str, user_id: &str) -> Result<PDFData, DbError> {
    let data = sqlx::query_as!(
        PDFData,
        r#"
        SELECT markdown, topic AS "title!: String"
        FROM lessons
        WHERE id = $1
        AND (assignee = $2 OR created_by = $2)
        "#,
        task_id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(data)
}
