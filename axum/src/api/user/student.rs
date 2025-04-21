use crate::api::error::APIError;
use crate::auth::jwt::Claims;
use crate::models::cards_decks::DeckBodySmall;
use crate::models::lessons::LessonBodySmall;
use crate::models::students::CompositeStudent;
use crate::models::students::Student;
use crate::models::students::UpdateStudentRequest;
use crate::models::tasks::TaskSmall;
use crate::schema::AppState;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn upsert_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    if claims.role != "teacher" {
        return Ok(StatusCode::UNAUTHORIZED);
    }
    sqlx::query!(
        r#"
        INSERT INTO teacher_student (teacher_id, student_id)
        VALUES ($1, $2)
        ON CONFLICT (teacher_id, student_id) DO UPDATE SET status = 'active'
        "#,
        claims.sub,
        id
    )
    .execute(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(dbe) if dbe.constraint() == Some("unique constraint") => {
            APIError::AlreadyExists("Already Exists".into())
        }
        _ => APIError::Database(e),
    })?;

    Ok(StatusCode::CREATED)
}

pub async fn remove_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    sqlx::query!(
        r#"
        UPDATE teacher_student
        SET status = 'inactive'
        WHERE teacher_id = $1 AND student_id = $2
        "#,
        claims.sub,
        id
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateStudentRequest>,
) -> Result<StatusCode, APIError> {
    sqlx::query!(
        r#"
        UPDATE teacher_student
        SET
            markdown = COALESCE($3, markdown),
            student_telegram_id = COALESCE($4, student_telegram_id)
        WHERE teacher_id = $1 AND student_id = $2
        "#,
        claims.sub,
        id,
        payload.markdown,
        payload.student_telegram_id
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn fetch_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CompositeStudent>, APIError> {
    let mut tx = state.db.begin().await?;

    let student = sqlx::query_as!(
        Student,
        r#"
        SELECT u.username, u.email, u.id, u.name, ts.markdown, ts.student_telegram_id
        FROM "user" u
        INNER JOIN teacher_student ts ON u.id = ts.student_id
        WHERE ts.teacher_id = $1 AND student_id = $2
        "#,
        claims.sub,
        id,
    )
    .fetch_one(&mut *tx)
    .await?;

    let decks = sqlx::query_as!(
        DeckBodySmall,
        r#"
        SELECT id, name, description FROM decks
        WHERE (created_by = $1 AND assignee = $2)
        LIMIT 2
        "#,
        claims.sub,
        student.id
    )
    .fetch_all(&mut *tx)
    .await?;

    let lessons = sqlx::query_as!(
        LessonBodySmall,
        r#"
        SELECT id, title, topic, markdown, created_at
        FROM lessons
        WHERE (created_by = $1 AND assignee = $2)
        ORDER BY created_at desc
        LIMIT 2
        "#,
        claims.sub,
        student.id,
    )
    .fetch_all(&mut *tx)
    .await?;

    let tasks = sqlx::query_as!(
        TaskSmall,
        r#"
        SELECT id, title, markdown, completed, due_date
        FROM tasks
        WHERE (created_by = $1 AND assignee = $2 AND completed = false)
        LIMIT 2
        "#,
        claims.sub,
        student.id,
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(Json(CompositeStudent {
        student,
        decks,
        lessons,
        tasks,
    }))
}

pub async fn list_students(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<Student>>, APIError> {
    let students = sqlx::query_as!(
        Student,
        r#"
        SELECT u.username, u.email, u.id, u.name, ts.markdown, ts.student_telegram_id
        FROM "user" u
        INNER JOIN teacher_student ts ON u.id = ts.student_id
        WHERE ts.teacher_id = $1 AND ts.status = 'active'
        ORDER BY u.name ASC
        "#,
        claims.sub
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(students))
}
