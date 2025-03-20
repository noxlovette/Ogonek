use crate::auth::jwt::Claims;
use super::error::APIError;
use crate::schema::AppState;
use crate::models::students::UpdateStudentRequest;
use crate::models::students::Student;
use axum::extract::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::extract::Path;

pub async fn upsert_student(
    claims: Claims, // from auth middleware
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    // Insert the relationship

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
    }
    )?;


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
            telegram_id = COALESCE($4, telegram_id)
        WHERE teacher_id = $1 AND student_id = $2
        "#,
        claims.sub,
        id,
        payload.markdown,
        payload.telegram_id
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn fetch_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Student>, APIError> {
    let student = sqlx::query_as!(
        Student,
        r#"
        SELECT u.username, u.email, u.id, u.name, ts.markdown, ts.telegram_id
        FROM "user" u
        INNER JOIN teacher_student ts ON u.id = ts.student_id
        WHERE ts.teacher_id = $1 AND student_id = $2
        "#,
        claims.sub,
        id,
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(student))
}

pub async fn list_students(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<Student>>, APIError> {
    let students = sqlx::query_as!(
        Student,
        r#"
        SELECT u.username, u.email, u.id, u.name, ts.markdown, ts.telegram_id
        FROM "user" u
        INNER JOIN teacher_student ts ON u.id = ts.student_id
        WHERE ts.teacher_id = $1 AND ts.status = 'active'
        "#,
        claims.sub
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(students))
}
