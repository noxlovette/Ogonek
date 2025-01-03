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
    pub id: RecordId,
    #[serde(rename = "in")]
    pub user: RecordId,
    #[serde(rename = "out")]
    pub lesson: RecordId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub added_at: DateTime<Utc>,
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

pub async fn create_bookmark(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<BookMarkBody>,
    token: Token,
) -> Result<Json<BookMarkBody>, DbError> {
    tracing::info!("Attempting to create bookmark");

    let db = &state.db;
    db.authenticate(token.as_str()).await?;

    let result: Vec<BookMarkBody> = db
        .query(
            "RELATE user:$user_id->bookmark->lesson:$lesson_id CONTENT {
        notes: $notes}",
        )
        .bind(("user_id", "1"))
        .bind(("lesson_id", id.clone()))
        .bind(("notes", "notes"))
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
