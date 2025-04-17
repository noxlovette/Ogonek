use crate::api::error::APIError;
use crate::auth::jwt::Claims;
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
) -> Result<Json<Option<File>>, APIError> {
    let file = sqlx::query_as!(
        File,
        r#"
        SELECT * FROM files
        WHERE id = $1 AND (
            owner_id = $2
        )
        "#,
        file_id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?;

    Ok(Json(file))
}

pub async fn list_files(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<FileListParams>,
) -> Result<Json<Vec<File>>, APIError> {
    let files = if let Some(folder_id) = params.parent_id {
        let folder_exists = sqlx::query!(
            r#"
            SELECT 1 as "exists!: bool" FROM files
            WHERE id = $1 AND(
            owner_id = $2
            )
            "#,
            folder_id,
            claims.sub
        )
        .fetch_optional(&state.db)
        .await?
        .is_some();

        if !folder_exists {
            return Err(APIError::NotFound("Folder Not Found".into()));
        }
        sqlx::query_as!(
            File,
            r#"
            SELECT * FROM files
            WHERE parent_id = $1 AND owner_id = $2
            ORDER BY is_folder DESC, name ASC
            "#,
            folder_id,
            claims.sub
        )
        .fetch_all(&state.db)
        .await?
    } else {
        // Root folder contents
        sqlx::query_as!(
            File,
            r#"
            SELECT * FROM files
            WHERE parent_id IS NULL AND owner_id = $1
            ORDER BY is_folder DESC, name ASC
            "#,
            claims.sub
        )
        .fetch_all(&state.db)
        .await?
    };

    Ok(Json(files))
}

pub async fn update_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
    Json(payload): Json<FileUpdate>,
) -> Result<StatusCode, APIError> {
    let parent_id = if let Some(parent_id) = payload.parent_id {
        let folder_exists = sqlx::query!(
            r#"
            SELECT 1 as "exists!: bool" FROM files
            WHERE id = $1 AND (
                owner_id = $2 AND is_folder = true
            )
            "#,
            parent_id,
            claims.sub
        )
        .fetch_optional(&state.db)
        .await?
        .is_some();

        if !folder_exists {
            return Err(APIError::NotFound("Folder Not Found".into()));
        }
        Some(parent_id)
    } else {
        None
    };

    sqlx::query!(
        r#"
        UPDATE files
        SET
            name = COALESCE($3, name),
            path = COALESCE($4, path),
            parent_id = COALESCE($5, parent_id)
        WHERE id=$1 AND owner_id = $2
        "#,
        file_id,
        claims.sub,
        payload.name,
        payload.path,
        parent_id
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
) -> Result<StatusCode, APIError> {
    tracing::info!(user_id = %claims.sub, file_id = %file_id, "Attempting to delete file");

    let file = match sqlx::query_as!(
        S3KeyRecord,
        r#"
        DELETE FROM files
        WHERE id = $1 AND owner_id = $2
        RETURNING s3_key
        "#,
        file_id,
        claims.sub
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(file) => {
            tracing::debug!(file_id = %file_id, "File record successfully deleted from database");
            file
        }
        Err(err) => {
            tracing::error!(
                error = %err,
                file_id = %file_id,
                user_id = %claims.sub,
                "Failed to delete file record from database"
            );
            return Err(APIError::from(err));
        }
    };

    if let Some(key) = &file.s3_key {
        delete_s3(key, &state).await?;
    } else {
        tracing::warn!(file_id = %file_id, "File record exists but has no S3 key");
        return Err(APIError::NotFound("Object not found".into()));
    }

    tracing::info!(file_id = %file_id, "File deletion completed successfully");
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
