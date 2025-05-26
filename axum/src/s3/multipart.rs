use crate::error::AppError;
use crate::models::{CompletedPart, MultipartInitResultS3, PartUploadUrl};
use crate::schema::AppState;
use aws_sdk_s3::presigning::PresigningConfig;
use std::error::Error;

pub async fn init_multipart_s3(
    state: &AppState,
    s3_key: &str,
    content_type: &str,
    total_parts: i32,
) -> Result<MultipartInitResultS3, AppError> {
    // Create multipart upload in S3
    let response = state
        .s3
        .create_multipart_upload()
        .bucket(&state.bucket_name)
        .key(s3_key)
        .content_type(content_type)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create multipart upload: {}", e)))?;

    let upload_id = response
        .upload_id()
        .ok_or(AppError::BadRequest("Missing upload ID".into()))?;

    // Generate presigned URLs for each part
    let mut presigned_urls = Vec::new();
    for part_number in 1..=total_parts {
        let presigned_req = state
            .s3
            .upload_part()
            .bucket(&state.bucket_name)
            .key(s3_key)
            .upload_id(upload_id)
            .part_number(part_number)
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

    Ok(MultipartInitResultS3 {
        upload_id: upload_id.to_string(),
        parts: presigned_urls,
    })
}

pub async fn complete_multipart_s3(
    state: &AppState,
    s3_key: &str,
    upload_id: &str,
    parts: Vec<CompletedPart>,
) -> Result<(), AppError> {
    let mut parts = parts;
    parts.sort_by_key(|p| p.part_number);

    let completed_parts: Vec<aws_sdk_s3::types::CompletedPart> = parts
        .iter()
        .map(|part| {
            aws_sdk_s3::types::CompletedPart::builder()
                .e_tag(part.etag.clone())
                .part_number(part.part_number)
                .build()
        })
        .collect();

    let completed_upload = aws_sdk_s3::types::CompletedMultipartUpload::builder()
        .set_parts(Some(completed_parts))
        .build();

    let resp = state
        .s3
        .complete_multipart_upload()
        .bucket(&state.bucket_name)
        .key(s3_key)
        .upload_id(upload_id)
        .multipart_upload(completed_upload)
        .send()
        .await;

    match resp {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Some(source) = e.source() {
                tracing::error!("Inner error: {:?}", source);
            }
            Err(AppError::Internal(format!(
                "Failed to complete multipart upload: {}",
                e
            )))
        }
    }
}

pub async fn abort_multipart_s3(
    state: &AppState,
    s3_key: &str,
    upload_id: &str,
) -> Result<(), AppError> {
    state
        .s3
        .abort_multipart_upload()
        .bucket(&state.bucket_name)
        .key(s3_key)
        .upload_id(upload_id)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to abort multipart upload: {}", e)))?;

    Ok(())
}
