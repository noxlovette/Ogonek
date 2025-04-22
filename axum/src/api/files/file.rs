use crate::api::error::APIError;
use crate::auth::jwt::Claims;
use crate::db::crud::file;
use crate::models::files::{CreateFolderRequest, File, FileListParams, FileUpdate, S3KeyRecord};
use crate::s3::post::delete_s3;
use crate::schema::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn fetch_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
) -> Result<Json<File>, APIError> {
    let file = file::find_by_id(&state.db, &file_id, &claims.sub).await?;

    Ok(Json(file))
}

pub async fn list_files(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<FileListParams>,
) -> Result<Json<Vec<File>>, APIError> {
    let files = file::find_all(&state.db, params, &claims.sub).await?;

    Ok(Json(files))
}

pub async fn update_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
    Json(payload): Json<FileUpdate>,
) -> Result<StatusCode, APIError> {
    file::update(&state.db, &file_id, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
) -> Result<StatusCode, APIError> {
    let file = file::delete(&state.db, &file_id, &claims.sub).await?;
    if let Some(key) = &file.s3_key {
        delete_s3(key, &state).await?;
    } else {
        return Err(APIError::NotFound("S3 Object not Found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// FOLDERS
pub async fn create_folder(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<FileListParams>,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<StatusCode, APIError> {
    let parent_path = if let Some(parent_id) = &params.parent_id {
        sqlx::query_scalar!(
            "SELECT path FROM files WHERE id = $1 AND owner_id = $2 AND is_folder = true",
            parent_id,
            claims.sub
        )
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| APIError::NotFound("Parent folder not found".into()))?
    } else {
        "/".to_string()
    };

    let folder_path = if parent_path.ends_with('/') {
        format!("{}{}", parent_path, payload.name)
    } else {
        format!("{}/{}", parent_path, payload.name)
    };

    let folder_id = nanoid::nanoid!();
    sqlx::query!(
        r#"
        INSERT INTO files (id, name, s3_key, path, is_folder, parent_id, owner_id, size)
        VALUES ($1, $2, $3, $4, true, $5, $6, 0)
        "#,
        folder_id,
        payload.name,
        folder_path,
        folder_path,
        params.parent_id,
        claims.sub
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::CREATED)
}

pub async fn delete_folder(
    State(state): State<AppState>,
    claims: Claims,
    Path(folder_id): Path<String>,
) -> Result<impl IntoResponse, APIError> {
    let folder = sqlx::query!(
        "SELECT id, path FROM files WHERE id = $1 AND owner_id = $2 AND is_folder = true",
        folder_id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| APIError::NotFound("Folder not found".into()))?;

    let mut tx = state.db.begin().await?;
    let files = sqlx::query_as!(
        S3KeyRecord,
        r#"
        WITH RECURSIVE folder_contents AS (
            SELECT id, s3_key FROM files WHERE id = $1
            UNION ALL
            SELECT f.id, f.s3_key FROM files f
            JOIN folder_contents fc ON f.parent_id = fc.id
            WHERE f.is_folder = false
        )
        SELECT s3_key FROM folder_contents WHERE s3_key != $2
        "#,
        folder_id,
        folder.path // Skip the folder itself which doesn't have a real S3 object
    )
    .fetch_all(&mut *tx)
    .await?;

    for file in files {
        if let Some(key) = &file.s3_key {
            delete_s3(key, &state).await?;
        }
    }

    sqlx::query!(
        "DELETE FROM files WHERE id = $1 AND owner_id = $2",
        folder_id,
        claims.sub
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}
