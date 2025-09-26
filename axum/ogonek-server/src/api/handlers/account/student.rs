use crate::{
    api::{USER_TAG, error::APIError},
    app::AppState,
    services::Claims,
};
use axum::extract::{Json, Path, State};

use axum::http::StatusCode;
use ogonek_db::core::{account::student, flashcards::deck, lesson, task};
use ogonek_types::{CompositeStudent, Student, UpdateStudentRequest};
#[utoipa::path(
    post,
    path = "/student/{id}",
    params(
        ("id" = String, Path, description = "Student ID")
    ),
    tag = USER_TAG,
    responses(
        (status = 201, description = "Student relationship created"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn upsert_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    student::upsert(&state.db, &claims.sub, &id).await?;

    Ok(StatusCode::CREATED)
}
#[utoipa::path(
    delete,
    path = "/student/{id}",
    params(
        ("id" = String, Path, description = "Student ID")
    ),
    tag = USER_TAG,
    responses(
        (status = 204, description = "Student relationship removed"),
        (status = 404, description = "Student not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn remove_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    student::delete(&state.db, &id, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}
#[utoipa::path(
    patch,
    path = "/student/{id}",
    params(
        ("id" = String, Path, description = "Student ID")
    ),
    request_body = UpdateStudentRequest,
    tag = USER_TAG,
    responses(
        (status = 204, description = "Student updated successfully"),
        (status = 404, description = "Student not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateStudentRequest>,
) -> Result<StatusCode, APIError> {
    student::update(&state.db, &id, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
#[utoipa::path(
    get,
    path = "/student/{id}",
    params(
        ("id" = String, Path, description = "Student ID")
    ),
    tag = USER_TAG,
    responses(
        (status = 200, description = "Student details retrieved", body = CompositeStudent),
        (status = 404, description = "Student not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_student(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CompositeStudent>, APIError> {
    let student = student::find_by_id(&state.db, &id, &claims.sub).await?;
    let decks = deck::find_recent(&state.db, &student.id).await?;
    let lessons = lesson::find_recent(&state.db, &student.id).await?;
    let tasks = task::read_recent(&state.db, &student.id).await?;

    Ok(Json(CompositeStudent {
        student,
        decks,
        lessons,
        tasks,
    }))
}
#[utoipa::path(
    get,
    path = "/student",
    tag = USER_TAG,
    responses(
        (status = 200, description = "Students list retrieved", body = Vec<Student>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_students(
    claims: Claims,
    State(state): State<AppState>,
) -> Result<Json<Vec<Student>>, APIError> {
    let students = student::find_all(&state.db, &claims.sub).await?;

    Ok(Json(students))
}
