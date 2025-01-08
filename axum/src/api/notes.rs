use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
use crate::models::lessons::{StudentNote, StudentNoteUpdate};
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;

// servers as both patch and create endpoint
pub async fn upsert_student_note(
    State(state): State<AppState>,
    claims: Claims,
    Path(lesson_id): Path<String>,
    Json(payload): Json<StudentNoteUpdate>,
) -> Result<Json<StudentNote>, DbError> {
    let note = sqlx::query_as!(
        StudentNote,
        r#"
        INSERT INTO student_notes (id, lesson_id, user_id, is_bookmarked, notes)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (lesson_id, user_id) 
        DO UPDATE SET
            is_bookmarked = EXCLUDED.is_bookmarked,
            notes = EXCLUDED.notes,
            updated_at = CURRENT_TIMESTAMP
        RETURNING *
        "#,
        nanoid::nanoid!(),
        lesson_id,
        claims.sub,
        payload.is_bookmarked,
        payload.notes
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(note))
}

pub async fn fetch_student_note(
    State(state): State<AppState>,
    claims: Claims,
    Path(lesson_id): Path<String>,
) -> Result<Json<Option<StudentNote>>, DbError> {
    let note = sqlx::query_as!(
        StudentNote,
        r#"
        SELECT * FROM student_notes
        WHERE lesson_id = $1 AND user_id = $2
        "#,
        lesson_id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?;

    Ok(Json(note))
}

pub async fn list_student_notes(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<StudentNote>>, DbError> {
    let notes = sqlx::query_as!(
        StudentNote,
        r#"
        SELECT * FROM student_notes
        WHERE user_id = $1
        "#,
        claims.sub
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(notes))
}

pub async fn delete_student_notes(
    State(state): State<AppState>,
    claims: Claims,
    Path(lesson_id): Path<String>,
) -> Result<Json<StudentNote>, DbError> {
    let note = sqlx::query_as!(
        StudentNote,
        r#"
        DELETE FROM student_notes
        WHERE lesson_id = $1 AND user_id = $2
        RETURNING *
        "#,
        lesson_id,
        claims.sub
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(note))
}
