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
        format!("{parent_path}{payload.name}")
    } else {
        format!("{parent_path}/{payload.name}")
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
