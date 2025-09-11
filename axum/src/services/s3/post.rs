use crate::error::AppError;
use crate::s3::S3Provider;

impl S3Provider {
    pub async fn delete_s3(&self, s3_key: &String) -> Result<(), AppError> {
        self.client
            .delete_object()
            .bucket(self.bucket_name.clone())
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
}
