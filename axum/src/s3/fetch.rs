use crate::error::AppError;
use crate::schema::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::json;

pub async fn get_presigned_url(
    State(state): State<AppState>,
    Path(encoded_key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Decode the base64 key
    let key = BASE64
        .decode(encoded_key)
        .map_err(|_| AppError::BadRequest("Invalid base64 encoding".into()))?;

    let key_str = String::from_utf8(key)
        .map_err(|_| AppError::BadRequest("Invalid UTF-8 in decoded key".into()))?;
    let filename = key_str.split('/').last().unwrap_or("download");
    // Create a presigned request that expires in 15 minutes
    let presigned_req = state
        .s3
        .get_object()
        .bucket(state.bucket_name)
        .key(&key_str)
        .response_content_disposition(format!("attachment; filename=\"{}\"", filename))
        .presigned(aws_sdk_s3::presigning::PresigningConfig::expires_in(
            std::time::Duration::from_secs(15 * 60),
        )?)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create presigned URL: {}", e)))?;

    let presigned_url = presigned_req.uri().to_string();

    Ok((StatusCode::OK, Json(json!({ "url": presigned_url }))))
}

pub async fn check_s3_connection(State(state): State<AppState>) -> Result<StatusCode, AppError> {
    let result = state.s3.list_buckets().send().await.map_err(|e| {
        tracing::error!("S3 connection test failed: {:?}", e);
        AppError::Internal(format!("S3 connection test failed: {}", e))
    })?;

    let bucket_count = result.buckets().len();
    let bucket_names: Vec<&str> = result.buckets().iter().filter_map(|b| b.name()).collect();

    tracing::info!(
        "Successfully connected to S3. Found {} buckets: {:?}",
        bucket_count,
        bucket_names
    );

    Ok(StatusCode::OK)
}
