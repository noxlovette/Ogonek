use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::files::{file, multipart};
use crate::models::{
    AbortMultipartRequest, CompleteMultipartRequest, InitUploadRequest, MultipartInitResultS3,
    MultipartUploadInit,
};
use crate::s3::{abort_multipart_s3, complete_multipart_s3, init_multipart_s3};
use crate::schema::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

#[utoipa::path(
    post,
    path = "/s3/multipart/init",
    request_body = InitUploadRequest,
    responses(
        (status = 200, description = "Multipart upload initialized", body = MultipartUploadInit),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Parent folder not found")
    ),
    security(
        ("api_key" = [])
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

    // Use the new CRUD function
    multipart::create_multipart_file(
        &state.db,
        &file_id,
        &payload.file_name,
        &s3_key,
        &payload.content_type,
        payload.file_size,
        payload.parent_id.as_deref(),
        &claims.sub,
        payload.task_id.as_deref(),
    )
    .await?;

    let MultipartInitResultS3 { upload_id, parts } =
        init_multipart_s3(&state, &s3_key, &payload.content_type, payload.total_parts).await?;

    Ok(Json(MultipartUploadInit {
        upload_id,
        file_id,
        s3_key,
        parts,
    }))
}

#[utoipa::path(
    post,
    path = "/s3/multipart/complete",
    request_body = CompleteMultipartRequest,
    responses(
        (status = 201, description = "Upload completed successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "File not found")
    ),
    security(
        ("api_key" = [])
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
    complete_multipart_s3(&state, &payload.s3_key, &payload.upload_id, payload.parts).await?;

    // Mark as complete in DB
    multipart::mark_upload_complete(&state.db, &payload.file_id, &claims.sub).await?;

    Ok(StatusCode::CREATED)
}

#[utoipa::path(
    post,
    path = "/s3/multipart/abort",
    request_body = AbortMultipartRequest,
    responses(
        (status = 200, description = "Upload aborted successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "File not found")
    ),
    security(
        ("api_key" = [])
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
    if let Err(e) = abort_multipart_s3(&state, &payload.s3_key, &payload.upload_id).await {
        // Log the S3 error but continue with DB cleanup
        tracing::warn!("Failed to abort S3 multipart upload: {:?}", e);
    }

    // Always clean up the DB record
    file::delete(&state.db, &payload.file_id, &claims.sub).await?;

    Ok(StatusCode::OK)
}
