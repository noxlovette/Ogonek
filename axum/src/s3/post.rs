use crate::schema::AppState;
use crate::error::AppError;
use serde::Serialize;
use aws_sdk_s3::primitives::ByteStream;
use sqlx::types::time::OffsetDateTime;


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


pub async fn upload_s3(
    file_name: &String,
    file_data: &Vec<u8>,
    s3_key: &String,
    mime_type: &String,
    state: &AppState,
) -> Result<(), AppError> {

    if file_data.is_empty() {
        return Err(AppError::BadRequest("File is empty".to_string()));
    }

    let bucket_name = std::env::var("SCW_BUCKET_NAME")
        .map_err(|err| {
            tracing::error!(error = %err, "Failed to get SCW_BUCKET_NAME from environment");
            AppError::Internal("Missing bucket configuration".into())
        })?;

    tracing::info!(
        "Uploading file to S3: name={}, key={}, size={}", 
        file_name, s3_key, file_data.len()
    );
    
    state.s3
        .put_object()
        .bucket(&bucket_name)
        .key(s3_key)
        .body(ByteStream::from(file_data.clone()))
        .content_type(mime_type)
        .send()
        .await
        .map_err(|e| {
            tracing::error!(
                error = %e,
                file_name = %file_name,
                s3_key = %s3_key,
                "S3 upload failed"
            );
            AppError::Internal(format!("S3 upload failed: {}", e))
        })?;
    
    Ok(())
}


pub async fn delete_s3(s3_key: &String, state: &AppState) -> Result<(), AppError> {
    let bucket_name = std::env::var("SCW_BUCKET_NAME")
        .map_err(|err| {
            tracing::error!(error = %err, "Failed to get SCW_BUCKET_NAME from environment");
            AppError::Internal("Missing bucket configuration".into())
        })?;

    state.s3
        .delete_object()
        .bucket(&bucket_name)
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