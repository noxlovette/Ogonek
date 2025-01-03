use crate::auth::Token;
use crate::db::error::DbError;
use crate::db::AppState;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Debug)]
pub struct LessonTime {
    pub custom_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookMarkBody {
    pub id: Option<RecordId>,
    #[serde(rename = "in")]
    pub user: RecordId,
    #[serde(rename = "out")]
    pub lesson: RecordId,
    pub notes: Option<String>,
    pub added_at: Option<DateTime<Utc>>,
}

pub async fn list_bookmarks(
    State(state): State<AppState>,
    token: Token,
) -> Result<Json<Vec<BookMarkBody>>, DbError> {
    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let bookmarks: Vec<BookMarkBody> = db.select("bookmark").await?;

    tracing::info!("Fetch bookmarks successful");

    Ok(Json(bookmarks))
}

pub async fn get_bookmark(
    Path(id): Path<String>,
    State(state): State<AppState>,
    token: Token,
) -> Result<Json<Option<BookMarkBody>>, DbError> {
    tracing::info!("Attempting to fetch bookmark");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let bookmark = db.select(("bookmark", &*id)).await?;

    tracing::info!("Fetch bookmark successful");

    Ok(Json(bookmark))
}
// #[axum::debug_handler]
pub async fn create_bookmark(
    State(state): State<AppState>,
    token: Token,
    Json(payload): Json<BookMarkBody>,
) -> Result<Json<BookMarkBody>, DbError> {
    tracing::info!("Attempting to create bookmark");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let user_id = payload.user.clone();
    let notes = payload.notes.clone();
    let lesson_id = payload.lesson.clone();

    let result: Vec<BookMarkBody> = db
        .query(
            "RELATE $user_id->bookmark->$lesson_id CONTENT {
        notes: $notes}",
        )
        .bind(("user_id", user_id))
        .bind(("lesson_id", lesson_id))
        .bind(("notes", notes))
        .await?
        .take(0)?;

    let bookmark = result.into_iter().next();

    tracing::info!("Bookmark created");
    if let Some(bookmark) = bookmark {
        Ok(Json(bookmark))
    } else {
        Err(DbError::Db)
    }
}

pub async fn delete_bookmark(
    State(state): State<AppState>,
    token: Token,
    id: Path<String>,
) -> Result<Json<BookMarkBody>, DbError> {
    tracing::info!("Attempting to delete bookmark");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let lesson = db.delete(("bookmark", &*id)).await?;

    tracing::info!("bookmark deleted");
    if let Some(lesson) = lesson {
        Ok(Json(lesson))
    } else {
        Err(DbError::Db)
    }
}

pub async fn update_bookmark(
    State(state): State<AppState>,
    token: Token,
    id: Path<String>,
    Json(payload): Json<BookMarkBody>,
) -> Result<Json<BookMarkBody>, DbError> {
    tracing::info!("Attempting to create bookmark");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let bookmark = db.update(("bookmark", &*id)).content(payload).await?;

    tracing::info!("Bookmark updated");
    if let Some(bookmark) = bookmark {
        Ok(Json(bookmark))
    } else {
        Err(DbError::Db)
    }
}
