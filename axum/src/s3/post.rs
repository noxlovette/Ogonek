use crate::error::AppError;
use crate::schema::AppState;
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_smithy_types::byte_stream::ByteStream;
use bytes::Bytes;
use serde::Serialize;
use sqlx::types::time::OffsetDateTime;

const CHUNK_SIZE: usize = 1024 * 1024 * 5;
const MAX_CHUNKS: usize = 10000;

#[derive(Debug, Serialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub s3_key: String,
    pub path: String,
    pub mime_type: Option<String>,
    pub size: i64,
    pub is_folder: bool,
    pub parent_id: Option<String>,
    pub owner_id: String,
    pub visibility: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
async fn upload_s3_simple(
    s3_key: &String,
    state: &AppState,
    file_data: &[u8],
    mime_type: &String,
) -> Result<(), AppError> {
    if file_data.is_empty() {
        return Err(AppError::BadRequest("File is empty".to_string()));
    }

    tracing::info!(
        "Uploading small file to S3: key={}, size={}",
        s3_key,
        file_data.len()
    );

    state
        .s3
        .put_object()
        .bucket(&state.bucket_name)
        .key(s3_key)
        .body(ByteStream::from(Bytes::copy_from_slice(file_data)))
        .content_type(mime_type)
        .send()
        .await
        .map_err(|e| {
            tracing::error!(
                error = %e,
                s3_key = %s3_key,
                "S3 upload failed"
            );
            AppError::Internal(format!("S3 upload failed: {}", e))
        })?;

    Ok(())
}

pub async fn delete_s3(s3_key: &String, state: &AppState) -> Result<(), AppError> {
    state
        .s3
        .delete_object()
        .bucket(&state.bucket_name)
        .key(s3_key)
        .send()
        .await
        .map_err(|err| {
            tracing::error!(
                error = %err,
                s3_key = %s3_key,
                "Failed to delete object from S3"
            );
            AppError::from(err)
        })?;

    tracing::info!(s3_key = %s3_key, "File deletion completed successfully");
    Ok(())
}

pub async fn multipart_upload(
    s3_key: &String,
    state: &AppState,
    file_data: &[u8],
    mime_type: &String,
) -> Result<(), AppError> {
    // Check if the file is large enough to warrant multipart upload
    if file_data.len() < CHUNK_SIZE {
        // For small files, use regular upload
        return upload_s3_simple(s3_key, state, file_data, mime_type).await;
    }

    // Start multipart upload
    let multipart_upload_res = state
        .s3
        .create_multipart_upload()
        .bucket(&state.bucket_name)
        .key(s3_key)
        .content_type(mime_type)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create multipart upload: {}", e)))?;

    let upload_id = multipart_upload_res
        .upload_id()
        .ok_or(AppError::BadRequest("Missing upload ID".into()))?;

    // Calculate chunks
    let file_size = file_data.len();
    let mut chunk_count = (file_size / CHUNK_SIZE) + if file_size % CHUNK_SIZE > 0 { 1 } else { 0 };

    if chunk_count > MAX_CHUNKS {
        return Err(AppError::BadRequest(
            "File too large, exceeds maximum chunks".into(),
        ));
    }

    // Track completed parts
    let mut upload_parts: Vec<CompletedPart> = Vec::with_capacity(chunk_count);

    // Upload each chunk
    for chunk_index in 0..chunk_count {
        let start_pos = chunk_index * CHUNK_SIZE;
        let end_pos = std::cmp::min(start_pos + CHUNK_SIZE, file_size);
        let chunk_size = end_pos - start_pos;

        // Create chunk from file data
        let chunk_data = &file_data[start_pos..end_pos];
        let stream = ByteStream::from(Bytes::copy_from_slice(chunk_data));

        // Part numbers start at 1
        let part_number = (chunk_index as i32) + 1;

        let upload_part_res = state
            .s3
            .upload_part()
            .key(s3_key)
            .bucket(&state.bucket_name)
            .upload_id(upload_id)
            .body(stream)
            .part_number(part_number)
            .send()
            .await
            .map_err(|e| {
                AppError::Internal(format!("Failed to upload part {}: {}", part_number, e))
            })?;

        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_res.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );

        tracing::debug!(
            "Uploaded part {} of {} for file {}, size: {} bytes",
            part_number,
            chunk_count,
            s3_key,
            chunk_size
        );
    }

    // Complete the multipart upload
    let completed_multipart_upload = CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts))
        .build();

    let _complete_multipart_upload_res = state
        .s3
        .complete_multipart_upload()
        .bucket(&state.bucket_name)
        .key(s3_key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to complete multipart upload: {}", e)))?;

    tracing::info!("Successfully completed multipart upload for {}", s3_key);
    Ok(())
}
// Modified upload_s3 that decides whether to use multipart or simple upload
pub async fn upload_s3(
    file_name: &String,
    file_data: &[u8],
    s3_key: &String,
    mime_type: &String,
    state: &AppState,
) -> Result<(), AppError> {
    let file_size = file_data.len();

    if file_size >= CHUNK_SIZE {
        tracing::info!(
            "Using multipart upload for file: name={}, key={}, size={}MB",
            file_name,
            s3_key,
            file_size / (1024 * 1024)
        );
        multipart_upload(s3_key, state, file_data, mime_type).await
    } else {
        tracing::info!(
            "Using simple upload for file: name={}, key={}, size={}KB",
            file_name,
            s3_key,
            file_size / 1024
        );
        upload_s3_simple(s3_key, state, file_data, mime_type).await
    }
}
