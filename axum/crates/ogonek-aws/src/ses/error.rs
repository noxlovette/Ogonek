use aws_credential_types::provider::error::CredentialsError;
use aws_sdk_sesv2::{config::auth::BuildError, error::SdkError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SESError {
    #[error("Access denied")]
    AccessDenied,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),
}

// Generic handler for all SES SDK errors
impl<E> From<SdkError<E>> for SESError
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(err: SdkError<E>) -> Self {
        match &err {
            SdkError::ConstructionFailure(err) => {
                tracing::error!("SES client construction error: {err:?}");
                Self::Internal("Failed to construct SES client".into())
            }
            SdkError::DispatchFailure(err) => {
                tracing::error!("SES dispatch error: {err:?}");
                Self::Internal("Failed to dispatch SES request".into())
            }
            SdkError::ResponseError(err) => {
                tracing::error!("SES response error: {err:?}");
                Self::Internal("SES response error".into())
            }
            SdkError::ServiceError(service_err) => {
                // Log the full error for debugging
                tracing::error!("SES service error: {service_err:?}");

                // Check error type using string matching as a fallback
                let err_str = format!("{service_err:?}");

                if err_str.contains("NoSuchKey") || err_str.contains("NoSuchBucket") {
                    Self::NotFound("SES resource not found".into())
                } else if err_str.contains("AccessDenied") {
                    Self::AccessDenied
                } else if err_str.contains("InvalidBucketName") {
                    Self::Validation("Invalid SES bucket name".into())
                } else if err_str.contains("BucketAlreadyExists")
                    || err_str.contains("BucketAlreadyOwnedByYou")
                {
                    Self::AlreadyExists("SES bucket already exists".into())
                } else {
                    Self::Internal(format!("SES service error: {service_err:?}"))
                }
            }
            SdkError::TimeoutError(_) => Self::Internal("SES request timed out".into()),
            _ => Self::Internal(format!("Unknown SES error: {err:?}")),
        }
    }
}

impl From<CredentialsError> for SESError {
    fn from(err: CredentialsError) -> Self {
        tracing::error!("AWS credentials error: {:?}", err);
        Self::Internal("Failed to authenticate with AWS".into())
    }
}

impl From<BuildError> for SESError {
    fn from(err: BuildError) -> Self {
        tracing::error!("AWS SES client build error: {:?}", err);
        Self::Internal("Failed build the AWS SES client".into())
    }
}
// Handle credential errors
impl From<aws_sdk_s3::error::BuildError> for SESError {
    fn from(err: aws_sdk_s3::error::BuildError) -> Self {
        tracing::error!("AWS SES client build error: {:?}", err);
        Self::Internal("Failed build the AWS SES client".into())
    }
}
