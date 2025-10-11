use crate::{
    AppState, Claims,
    api::{TASK_TAG, error::APIError},
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use ogonek_db::core::file::{self, fetch_files_task};
use ogonek_types::{
    BatchPresignedUrlResponse, File, FileListParams, FileUpdate, PresignedFileUrl,
    PresignedUrlResponse,
};

/// Retrieves file metadata by file ID
///
/// Returns file information for the authenticated user.
pub async fn fetch_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
) -> Result<Json<File>, APIError> {
    let file = file::find_by_id(&state.db, &file_id, &claims.sub).await?;

    Ok(Json(file))
}

/// Generates a presigned URL for file access
///
/// Creates a temporary URL for accessing a file stored in S3 using base64 encoded key.
#[utoipa::path(
    get,
    path = "/presigned/{encoded_key}",
    params(
        ("encoded_key" = String, Path, description = "Base64 encoded file key")
    ),
    tag = TASK_TAG,
    responses(
        (status = 200, description = "Presigned URL generated successfully", body = PresignedUrlResponse),
        (status = 400, description = "Bad request - Invalid encoding or key"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "File not found")
    )
)]
pub async fn fetch_presigned_url(
    State(state): State<AppState>,
    Path(encoded_key): Path<String>,
) -> Result<impl IntoResponse, APIError> {
    let key = BASE64
        .decode(encoded_key)
        .map_err(|_| APIError::BadRequest("Invalid base64 encoding".into()))?;

    let key_str = String::from_utf8(key)
        .map_err(|_| APIError::BadRequest("Invalid UTF-8 in decoded key".into()))?;

    let file_id = key_str
        .rsplit("/")
        .next()
        .unwrap()
        .split(".")
        .next()
        .unwrap();

    let file = file::find_by_id_no_owner(&state.db, file_id).await?;
    let presigned_url = state.s3.get_presigned_url(key_str, file.name).await?;

    // Return the structured response instead of raw JSON
    Ok((
        StatusCode::OK,
        Json(PresignedUrlResponse { url: presigned_url }),
    ))
}

/// Generates presigned URLs for all files associated with a task
///
/// Returns batch presigned URLs for all files linked to the specified task.
#[utoipa::path(
    post,
    path = "/presigned/batch/{task_id}",
    params(
        ("task_id" = String, Path, description = "The DB id of the task the files belong to")
    ),
    tag = TASK_TAG,
    responses(
        (status = 200, description = "Presigned URLs generated successfully", body = BatchPresignedUrlResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_presigned_urls_batch(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<impl IntoResponse, APIError> {
    let mut presigned_urls = Vec::new();

    let files = fetch_files_task(&state.db, &task_id).await?;

    for file_id in files.iter().map(|task| task.id.clone()) {
        // Get file info
        let file = file::find_by_id_no_owner(&state.db, &file_id).await?;

        let presigned_url = state
            .s3
            .get_presigned_url(file.s3_key.clone(), file.name.clone())
            .await?;

        presigned_urls.push(PresignedFileUrl {
            file_id: file_id.clone(),
            url: presigned_url,
        });
    }

    Ok((
        StatusCode::OK,
        Json(BatchPresignedUrlResponse {
            urls: presigned_urls,
        }),
    ))
}

/// Lists files accessible to the authenticated user
///
/// Returns files based on query parameters and user permissions.
pub async fn list_files(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<FileListParams>,
) -> Result<Json<Vec<File>>, APIError> {
    let files = file::find_all(&state.db, params, &claims.sub).await?;

    Ok(Json(files))
}

/// Updates file metadata
///
/// Modifies file properties for the authenticated user.
pub async fn update_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
    Json(payload): Json<FileUpdate>,
) -> Result<StatusCode, APIError> {
    file::update(&state.db, &file_id, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
/// Deletes a file from database and S3 storage
///
/// Removes the file metadata and actual file from S3 bucket.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "File ID")
    ),
    tag = TASK_TAG,
    responses(
        (status = 204, description = "File deleted successfully"),
        (status = 404, description = "File not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn delete_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
) -> Result<StatusCode, APIError> {
    let file = file::delete(&state.db, &file_id, &claims.sub).await?;
    if let Some(key) = &file.s3_key {
        state.s3.delete_s3(key).await?;
    } else {
        return Err(APIError::NotFound("S3 Object not Found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}
