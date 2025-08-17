use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::core::files::multipart::{FileCreateParams, FileLinkOptions};
use crate::db::crud::core::files::{file, multipart};
use crate::schema::AppState;
use crate::types::{
    AbortMultipartRequest, CompleteMultipartRequest, InitUploadRequest, MultipartInitResultS3,
    MultipartUploadInit,
};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use crate::api::TASK_TAG;

/// Multipart upload init endpoint
#[utoipa::path(
    post,
    path = "/init",
    request_body = InitUploadRequest,
    tag = TASK_TAG,
    responses(
        (status = 200, description = "Multipart upload initialized", body = MultipartUploadInit),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Parent folder not found")
    )
)]
pub async fn init_multipart_upload(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<InitUploadRequest>,
) -> Result<Json<MultipartUploadInit>, APIError> {
    // Validate parent folder if provided
    if let Some(ref parent_id) = payload.parent_id {
        file::check_file_exists(&state.db, parent_id, &claims.sub).await?;
    }

    let file_id = nanoid::nanoid!();
    let file_extension = payload.file_name.rsplit('.').next().unwrap_or("bin"); // More idiomatic than split().next_back()

    let s3_key = match payload.task_id.is_some() {
        true => format!("tasks/{}/{}.{}", claims.sub, file_id, file_extension),
        false => format!("user-files/{}/{}.{}", claims.sub, file_id, file_extension),
    };

    let file_params = FileCreateParams::new(file_id.clone(), payload.file_name, claims.sub)
        .with_s3_key(s3_key.clone())
        .with_content_type(payload.content_type.clone())
        .with_size(payload.file_size)
        .with_parent(payload.parent_id);

    let link_options = FileLinkOptions {
        task_id: payload.task_id,
    };

    multipart::create_multipart_file(&state.db, file_params, link_options).await?;

    let MultipartInitResultS3 { upload_id, parts } = state
        .s3
        .init_multipart_s3(&s3_key, &payload.content_type, payload.total_parts)
        .await?;

    Ok(Json(MultipartUploadInit {
        upload_id,
        file_id,
        s3_key,
        parts,
    }))
}
/// Complete a part of the upload
#[utoipa::path(
    post,
    path = "/complete",
    tag = TASK_TAG,
    request_body = CompleteMultipartRequest,
    responses(
        (status = 201, description = "Upload completed successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "File not found")
    )
)]
pub async fn complete_multipart_upload(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CompleteMultipartRequest>,
) -> Result<StatusCode, APIError> {
    // Verify file ownership first
    file::check_file_exists(&state.db, &payload.file_id, &claims.sub).await?;

    // Complete S3 upload
    state
        .s3
        .complete_multipart_s3(&payload.s3_key, &payload.upload_id, payload.parts)
        .await?;

    // Mark as complete in DB
    multipart::mark_upload_complete(&state.db, &payload.file_id, &claims.sub).await?;

    Ok(StatusCode::CREATED)
}
/// Cancel multipart upload
#[utoipa::path(
    post,
    tag = TASK_TAG,
    path = "/abort",
    request_body = AbortMultipartRequest,
    responses(
        (status = 200, description = "Upload aborted successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "File not found")
    )
)]
pub async fn abort_multipart_upload(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<AbortMultipartRequest>,
) -> Result<StatusCode, APIError> {
    // Verify file ownership
    file::check_file_exists(&state.db, &payload.file_id, &claims.sub).await?;

    // Abort S3 upload first (if this fails, we still want to clean up the DB record)
    if let Err(e) = state
        .s3
        .abort_multipart_s3(&payload.s3_key, &payload.upload_id)
        .await
    {
        // Log the S3 error but continue with DB cleanup
        tracing::warn!("Failed to abort S3 multipart upload: {:?}", e);
    }

    // Always clean up the DB record
    file::delete(&state.db, &payload.file_id, &claims.sub).await?;

    Ok(StatusCode::OK)
}
