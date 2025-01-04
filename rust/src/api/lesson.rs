// use crate::auth::Token;
use crate::db::error::DbError;
use crate::db::init::AppState;
use crate::models::lessons::LessonBody;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;

pub async fn fetch_lesson(
    State(state): State<AppState>,
    id: Path<String>,
) -> Result<Json<Option<LessonBody>>, DbError> {
    tracing::info!("Attempting to fetch user");

    let db = &state.db;

    let lesson = db.select(("lesson", &*id)).await?;

    dbg!(&lesson);
    tracing::info!("Fetch lesson successful");
    Ok(Json(lesson))
}

pub async fn list_lessons(State(state): State<AppState>) -> Result<Json<Vec<LessonBody>>, DbError> {
    let db = &state.db;

    let lessons: Vec<LessonBody> = db.select("lesson").await?;

    tracing::info!("Fetch lessons successful");

    Ok(Json(lessons))
}

pub async fn create_lesson(
    State(state): State<AppState>,

    Json(payload): Json<LessonBody>,
) -> Result<Json<LessonBody>, DbError> {
    tracing::info!("Attempting to create lesson");

    let db = &state.db;

    let lesson = db.create("lesson").content(payload).await?;

    tracing::info!("lesson created");
    if let Some(lesson) = lesson {
        Ok(Json(lesson))
    } else {
        Err(DbError::Db)
    }
}

pub async fn delete_lesson(
    State(state): State<AppState>,

    id: Path<String>,
) -> Result<Json<LessonBody>, DbError> {
    tracing::info!("Attempting to delete lesson");

    let db = &state.db;

    let lesson = db.delete(("lesson", &*id)).await?;

    tracing::info!("lesson deleted");
    if let Some(lesson) = lesson {
        Ok(Json(lesson))
    } else {
        Err(DbError::Db)
    }
}

pub async fn update_lesson(
    State(state): State<AppState>,

    id: Path<String>,
    Json(payload): Json<LessonBody>,
) -> Result<Json<LessonBody>, DbError> {
    tracing::info!("Attempting to update lesson");

    let db = &state.db;

    let lesson = db.update(("lesson", &*id)).content(payload).await?;

    tracing::info!("lesson created");
    if let Some(lesson) = lesson {
        Ok(Json(lesson))
    } else {
        Err(DbError::Db)
    }
}
