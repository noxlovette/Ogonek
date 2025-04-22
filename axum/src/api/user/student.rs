use crate::api::error::APIError;
use crate::auth::jwt::Claims;
use crate::db::crud::user::student;

use crate::models::{CompositeStudent, Student, UpdateStudentRequest};
use crate::schema::AppState;
use axum::extract::{Json, Path, State};

use axum::http::StatusCode;

pub async fn upsert_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    if claims.role != "teacher" {
        return Ok(StatusCode::UNAUTHORIZED);
    }

    student::upsert(&state.db, &claims.sub, &id).await?;

    Ok(StatusCode::CREATED)
}

pub async fn remove_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    student::delete(&state.db, &id, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateStudentRequest>,
) -> Result<StatusCode, APIError> {
    student::update(&state.db, &id, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn fetch_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CompositeStudent>, APIError> {
    let student = student::find_by_id_and_data(&state.db, &id, &claims.sub).await?;

    Ok(Json(student))
}

pub async fn list_students(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<Student>>, APIError> {
    let students = student::find_all(&state.db, &claims.sub).await?;

    Ok(Json(students))
}
