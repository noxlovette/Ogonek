use crate::error::AppError;
use crate::schema::AppState;

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
