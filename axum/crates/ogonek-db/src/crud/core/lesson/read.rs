use crate::DbError;
use ogonek_types::{LessonFull, LessonPaginationParams, LessonSmall, PDFData};
use sqlx::PgPool;
/// Finds a list of mini-lessons (no markdown) according to passed Pagination params
pub async fn find_all(
    db: &PgPool,
    user_id: &str,
    params: &LessonPaginationParams, // <- Use specific params
) -> Result<Vec<LessonSmall>, DbError> {
    let mut query_builder = sqlx::QueryBuilder::new(
        r#"
        SELECT l.id, l.title, l.topic, l.created_at, l.updated_at,
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

    // Search filter
    if let Some(search) = &params.search {
        query_builder.push(" AND (l.title ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR l.topic ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(" OR l.markdown ILIKE ");
        query_builder.push_bind(format!("%{search}%"));
        query_builder.push(")");
    }

    // Topic filter (lesson-specific)
    if let Some(topic) = &params.topic {
        query_builder.push(" AND l.topic ILIKE ");
        query_builder.push_bind(format!("%{topic}%"));
    }

    // Assignee filter
    if let Some(assignee) = &params.assignee {
        query_builder.push(" AND l.assignee = ");
        query_builder.push_bind(assignee);
    }

    // Dynamic ordering
    query_builder.push(" ORDER BY ");
    query_builder.push(params.sort_by.to_lesson_column());
    query_builder.push(" ");
    query_builder.push(params.sort_order.to_sql());

    // Pagination
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
/// Finds one lesson by its id, will return not found if the user doesn't have access to the data
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
            u.name as "assignee_name?"
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
