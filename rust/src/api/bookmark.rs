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
