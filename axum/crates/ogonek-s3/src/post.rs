use crate::{S3Provider, error::S3Error};

impl S3Provider {
    pub async fn delete_s3(&self, s3_key: &String) -> Result<(), S3Error> {
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
                S3Error::from(err)
            })?;

        tracing::info!(s3_key = %s3_key, "File deletion completed successfully");
        Ok(())
    }

    pub async fn upload_object(
        &self,
        s3_key: &str,
        body: Vec<u8>,
        content_type: Option<&str>,
    ) -> Result<(), S3Error> {
        let mut put_req = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(s3_key)
            .body(body.into());

        if let Some(ct) = content_type {
            put_req = put_req.content_type(ct);
        }

        put_req.send().await.map_err(|err| {
            tracing::error!(error = %err, s3_key = %s3_key, "Failed to upload to S3");
            S3Error::from(err)
        })?;

        tracing::info!(s3_key = %s3_key, "File upload successful");
        Ok(())
    }
}
