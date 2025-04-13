use crate::auth::jwt::Claims;
use crate::error::AppError;

use crate::schema::AppState;
use aws_sdk_s3::presigning::PresigningConfig;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

// Request to initialize a multipart upload
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitUploadRequest {
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub total_parts: i32,
    pub parent_id: Option<String>,
    pub task_id: Option<String>,
}

// Info about a part's upload URL
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PartUploadUrl {
    pub part_number: i32,
    pub url: String,
}

// Response after initializing a multipart upload
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipartUploadInit {
    pub upload_id: String,
    pub file_id: String,
    pub s3_key: String,
    pub parts: Vec<PartUploadUrl>,
}

// Request to complete a multipart upload
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteMultipartRequest {
    pub upload_id: String,
    pub file_id: String,
    pub s3_key: String,
    pub parts: Vec<CompletedPart>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbortMultipartRequest {
    pub upload_id: String,
    pub file_id: String,
    pub s3_key: String,
}
// Info about a completed part
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedPart {
    pub part_number: i32,
    pub etag: String,
}

// API endpoints implementation

// 1. Initialize a multipart upload
pub async fn init_multipart_upload(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<InitUploadRequest>,
) -> Result<Json<MultipartUploadInit>, AppError> {
    // Check parent folder if provided
    if let Some(ref parent_id) = payload.parent_id {
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
            return Err(AppError::NotFound("Folder Not Found".into()));
        }
    }

    // Generate file ID and S3 key
    let file_id = nanoid!();
    let file_extension = payload.file_name.split('.').last().unwrap_or("");
    let s3_key = if payload.task_id.is_some() {
        format!("tasks/{}/{}.{}", claims.sub, file_id, file_extension)
    } else {
        format!("user-files/{}/{}.{}", claims.sub, file_id, file_extension)
    };

    // Create the file entry in the database with pending status
    let path = format!("/{}", payload.file_name);

    // Create a files record that will be updated later
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

    // Associate with task if provided
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

    // Create multipart upload in S3
    let response = state
        .s3
        .create_multipart_upload()
        .bucket(&state.bucket_name)
        .key(&s3_key)
        .content_type(payload.content_type)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create multipart upload: {}", e)))?;

    let upload_id = response
        .upload_id()
        .ok_or(AppError::BadRequest("Missing upload ID".into()))?;

    // Generate presigned URLs for each part
    let mut presigned_urls = Vec::new();
    for part_number in 1..=payload.total_parts {
        let presigned_req = state
            .s3
            .upload_part()
            .bucket(&state.bucket_name)
            .key(&s3_key)
            .upload_id(upload_id)
            .part_number(part_number as i32)
            .presigned(PresigningConfig::expires_in(
                std::time::Duration::from_secs(3600),
            )?)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to generate presigned URL: {}", e)))?;

        presigned_urls.push(PartUploadUrl {
            part_number,
            url: presigned_req.uri().to_string(),
        });
    }

    Ok(Json(MultipartUploadInit {
        upload_id: upload_id.to_string(),
        file_id,
        s3_key,
        parts: presigned_urls,
    }))
}

// 2. Complete a multipart upload
pub async fn complete_multipart_upload(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CompleteMultipartRequest>,
) -> Result<StatusCode, AppError> {
    // Verify ownership of the file
    let _file = sqlx::query!(
        r#"
        SELECT id, name, mime_type
        FROM files
        WHERE id = $1 AND owner_id = $2 AND upload_status = 'pending'
        "#,
        payload.file_id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("File not found or not owned by user".into()))?;

    // Convert completed parts to AWS format
    let completed_parts: Vec<aws_sdk_s3::types::CompletedPart> = payload
        .parts
        .iter()
        .map(|part| {
            aws_sdk_s3::types::CompletedPart::builder()
                .e_tag(part.etag.clone())
                .part_number(part.part_number)
                .build()
        })
        .collect();

    // Complete the multipart upload in S3
    let completed_upload = aws_sdk_s3::types::CompletedMultipartUpload::builder()
        .set_parts(Some(completed_parts))
        .build();

    state
        .s3
        .complete_multipart_upload()
        .bucket(&state.bucket_name)
        .key(&payload.s3_key)
        .upload_id(payload.upload_id)
        .multipart_upload(completed_upload)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to complete multipart upload: {}", e)))?;

    // Update file status in the database
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

    // Return the completed file info
    Ok(StatusCode::CREATED)
}

// 3. Abort a multipart upload if something goes wrong
pub async fn abort_multipart_upload(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<AbortMultipartRequest>,
) -> Result<StatusCode, AppError> {
    // Verify ownership of the file
    let file_exists = sqlx::query!(
        r#"
        SELECT 1 as "exists!: bool"
        FROM files
        WHERE id = $1 AND owner_id = $2 AND upload_status = 'pending'
        "#,
        payload.file_id,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await?
    .is_some();

    if !file_exists {
        return Err(AppError::NotFound(
            "File not found or not owned by user".into(),
        ));
    }

    // Abort the multipart upload in S3
    state
        .s3
        .abort_multipart_upload()
        .bucket(&state.bucket_name)
        .key(&payload.s3_key)
        .upload_id(payload.upload_id)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to abort multipart upload: {}", e)))?;

    // Delete the file record from the database
    sqlx::query!(
        r#"
        DELETE FROM files
        WHERE id = $1 AND owner_id = $2
        "#,
        payload.file_id,
        claims.sub
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::OK)
}
