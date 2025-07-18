use crate::error::AppError;
use crate::schema::AppState;
use aws_sdk_s3::Client as S3Client;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn get_presigned_url(
    bucket_name: &String,
    s3: &S3Client,
    key: String,
    filename: String,
) -> Result<String, AppError> {
    tracing::debug!("presigning url");
    let presigned_req = s3
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .response_content_disposition(format!("attachment; filename=\"{filename}\""))
        .presigned(aws_sdk_s3::presigning::PresigningConfig::expires_in(
            std::time::Duration::from_secs(15 * 60),
        )?)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create presigned URL: {e}")))?;

    let presigned_url = presigned_req.uri().to_string();

    Ok(presigned_url)
}

pub async fn check_s3_connection(State(state): State<AppState>) -> Result<StatusCode, AppError> {
    let result = state.s3.list_buckets().send().await.map_err(|e| {
        tracing::error!("S3 connection test failed: {e:?}");
        AppError::Internal(format!("S3 connection test failed: {e}"))
    })?;

    let bucket_count = result.buckets().len();
    let bucket_names: Vec<&str> = result.buckets().iter().filter_map(|b| b.name()).collect();

    tracing::info!("Successfully connected to S3. Found {bucket_count} buckets: {bucket_names:?}");

    Ok(StatusCode::OK)
}
