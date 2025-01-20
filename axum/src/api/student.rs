use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
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
) -> Result<StatusCode, DbError> {
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
    .map_err(|e| {
        eprintln!("Failed to upsert student: {:?}", e);
        // You might want to handle unique constraint violations separately
        if e.to_string().contains("unique constraint") {
            return DbError::AlreadyExists;
        }
        DbError::Db
    })?;

    Ok(StatusCode::CREATED)
}

pub async fn remove_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, DbError> {
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
    .await
    .map_err(|e| {
        eprintln!("Failed to remove student: {:?}", e);
        DbError::Db
    })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateStudentRequest>,
) -> Result<StatusCode, DbError> {
    
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
    .await
    .map_err(|e| {
        eprintln!("Failed to modify student: {:?}", e);
        DbError::Db
    })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn fetch_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Student>, DbError> {
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
    .await
    .map_err(|e| {
        eprintln!("{:?}", e);
        DbError::Db
    })?;

    Ok(Json(student))
}

pub async fn list_students(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<Student>>, DbError> {
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
    .await
    .map_err(|e| {
        eprintln!("{:?}", e);
        DbError::Db
    })?;

    Ok(Json(students))
}
