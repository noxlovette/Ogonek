use crate::auth::jwt::Claims;
use crate::schema::AppState;
use crate::models::lessons::{LessonBodySmall, LessonBodyWithStudent, LessonCreateBody, LessonUpdate, PaginationParams};
use axum::extract::{Json, Query};
use axum::extract::Path;
use axum::extract::State;
use hyper::StatusCode;
use super::error::APIError;
use crate::models::meta::{CreationId, PaginatedResponse};

pub async fn fetch_recent_lessons(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<LessonBodySmall>>, APIError> {
    let tasks = sqlx::query_as!(
        LessonBodySmall,
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
        claims.sub
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(tasks))
}

pub async fn fetch_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<Json<Option<LessonBodyWithStudent>>, APIError> {
    let lesson = sqlx::query_as!(
        LessonBodyWithStudent,
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
        id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?;

    Ok(Json(lesson))
}
pub async fn list_lessons(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
    claims: Claims,
) -> Result<Json<PaginatedResponse<LessonBodyWithStudent>>, APIError> {

    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT l.id, l.title, l.topic, l.markdown, l.assignee, l.created_by, l.created_at, l.updated_at, u.name as assignee_name 
         FROM lessons l 
         LEFT JOIN \"user\" u ON l.assignee = u.id 
         WHERE (l.assignee = ");

    query_builder.push_bind(&claims.sub);
    query_builder.push(" OR l.created_by = ");
    query_builder.push_bind(&claims.sub);
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
        .build_query_as::<LessonBodyWithStudent>()
        .fetch_all(&state.db)
        .await?;
    // Count query - build a similar query without LIMIT/OFFSET but with COUNT
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM lessons l WHERE (l.assignee = ");
    
    count_query_builder.push_bind(&claims.sub);
    count_query_builder.push(" OR l.created_by = ");
    count_query_builder.push_bind(&claims.sub);
    count_query_builder.push(")");
    
    // Add the same filters as the main query
    if let Some(search) = &params.search {
        count_query_builder.push(" AND (l.title ILIKE ");
        count_query_builder.push_bind(format!("%{}%", search));
        count_query_builder.push(" OR l.topic ILIKE ");
        count_query_builder.push_bind(format!("%{}%", search));
        count_query_builder.push(" OR l.markdown ILIKE ");
        count_query_builder.push_bind(format!("%{}%", search));
        count_query_builder.push(")");
    }
    
    if let Some(assignee) = &params.assignee {
        count_query_builder.push(" AND l.assignee = ");
        count_query_builder.push_bind(assignee);
    }
    
    let count: i64 = count_query_builder
        .build_query_scalar()
        .fetch_one(&state.db)
        .await?;
    
    Ok(Json(PaginatedResponse {
        data: lessons,
        total: count,
        page: params.page(),
        per_page: params.limit(),
    }))
}


pub async fn create_lesson(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<LessonCreateBody>,
) -> Result<Json<CreationId>, APIError> {
    let mut assignee = &claims.sub;

    if payload.assignee.is_some() {
        assignee = payload.assignee.as_ref().unwrap();
    }

    let id = sqlx::query_as!(
        CreationId,
        "INSERT INTO lessons (id, title, topic, markdown, created_by, assignee) 
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id",
        nanoid::nanoid!(),
        payload.title,
        payload.topic,
        payload.markdown,
        claims.sub,
        assignee
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(id))
}

pub async fn delete_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    sqlx::query!(
        "DELETE FROM lessons WHERE id = $1 AND created_by = $2",
        id,
        claims.sub
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
    Json(payload): Json<LessonUpdate>,
) -> Result<StatusCode, APIError> {
    sqlx::query!(
        "UPDATE lessons 
         SET 
            title = COALESCE($1, title),
            topic =COALESCE($2, topic), 
            markdown = COALESCE($3, markdown),
            assignee = COALESCE($4, assignee)
         WHERE id = $5 AND created_by = $6
",
        payload.title,
        payload.topic,
        payload.markdown,
        payload.assignee,
        id,
        claims.sub
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
