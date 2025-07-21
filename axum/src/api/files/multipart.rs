use crate::auth::Claims;
use crate::db::crud::files::file;
use crate::error::AppError;
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
) -> Result<Json<MultipartUploadInit>, AppError> {
    if let Some(ref parent_id) = payload.parent_id {
        file::check_file_exists(&state.db, parent_id, &claims.sub).await?;
        Some(parent_id)
    } else {
        None
    };

    let file_id = nanoid::nanoid!();
    let file_extension = payload.file_name.split('.').next_back().unwrap_or("");
    let s3_key = if payload.task_id.is_some() {
        format!("tasks/{}/{}.{}", claims.sub, file_id, file_extension)
    } else {
        format!("user-files/{}/{}.{}", claims.sub, file_id, file_extension)
    };

    let path = format!("/{}", payload.file_name);

    let mut tx = state.db.begin().await?;

    sqlx::query!(
        r#"
        INSERT INTO files (
            id, name, s3_key, path, mime_type, size, is_folder, parent_id, owner_id, visibility, upload_status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#,
        file_id,
        payload.file_name,
        s3_key,
        path,
        Some(payload.content_type.clone()),
        payload.file_size,
        false,
        payload.parent_id,
        claims.sub,
        "private",
        "pending" // Mark as pending until upload is complete
    )
    .execute(&mut *tx)
    .await?;

    if let Some(task_id) = payload.task_id {
        sqlx::query!(
            r#"
            INSERT INTO task_files (task_id, file_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            task_id,
            file_id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

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
) -> Result<StatusCode, AppError> {
    file::check_file_exists(&state.db, &payload.file_id, &claims.sub).await?;
    complete_multipart_s3(&state, &payload.s3_key, &payload.upload_id, payload.parts).await?;

    sqlx::query!(
        r#"
        UPDATE files
        SET upload_status = 'complete'
        WHERE id = $1 AND owner_id = $2
        "#,
        payload.file_id,
        claims.sub
    )
    .execute(&state.db)
    .await?;

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
) -> Result<StatusCode, AppError> {
    file::check_file_exists(&state.db, &payload.file_id, &claims.sub).await?;
    abort_multipart_s3(&state, &payload.s3_key, &payload.upload_id).await?;
    file::delete(&state.db, &payload.file_id, &claims.sub).await?;

    Ok(StatusCode::OK)
}
