use aws_credential_types::provider::error::CredentialsError;
use aws_sdk_s3::{error::SdkError, presigning::PresigningConfigError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum S3Error {
    #[error("Access denied")]
    AccessDenied,

    // Resource errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    // General server errors
    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),
}

// Generic handler for all S3 SDK errors
impl<E> From<SdkError<E>> for S3Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(err: SdkError<E>) -> Self {
        match &err {
            SdkError::ConstructionFailure(err) => {
                tracing::error!("S3 client construction error: {err:?}");
                Self::Internal("Failed to construct S3 client".into())
            }
            SdkError::DispatchFailure(err) => {
                tracing::error!("S3 dispatch error: {err:?}");
                Self::Internal("Failed to dispatch S3 request".into())
            }
            SdkError::ResponseError(err) => {
                tracing::error!("S3 response error: {err:?}");
                Self::Internal("S3 response error".into())
            }
            SdkError::ServiceError(service_err) => {
                // Log the full error for debugging
                tracing::error!("S3 service error: {service_err:?}");

                // Check error type using string matching as a fallback
                let err_str = format!("{service_err:?}");

                if err_str.contains("NoSuchKey") || err_str.contains("NoSuchBucket") {
                    Self::NotFound("S3 resource not found".into())
                } else if err_str.contains("AccessDenied") {
                    Self::AccessDenied
                } else if err_str.contains("InvalidBucketName") {
                    Self::Validation("Invalid S3 bucket name".into())
                } else if err_str.contains("BucketAlreadyExists")
                    || err_str.contains("BucketAlreadyOwnedByYou")
                {
                    Self::AlreadyExists("S3 bucket already exists".into())
                } else {
                    Self::Internal(format!("S3 service error: {service_err:?}"))
                }
            }
            SdkError::TimeoutError(_) => Self::Internal("S3 request timed out".into()),
            _ => Self::Internal(format!("Unknown S3 error: {err:?}")),
        }
    }
}

// Handle Presigning-specific errors
impl From<PresigningConfigError> for S3Error {
    fn from(err: PresigningConfigError) -> Self {
        tracing::error!("S3 presigning error: {:?}", err);
        Self::Internal(format!("Failed to generate presigned URL: {err}"))
    }
}

// Handle credential errors
impl From<CredentialsError> for S3Error {
    fn from(err: CredentialsError) -> Self {
        tracing::error!("AWS credentials error: {:?}", err);
        Self::Internal("Failed to authenticate with AWS".into())
    }
}
