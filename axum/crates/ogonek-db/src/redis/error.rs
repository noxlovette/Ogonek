use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("Redis Internal Error: {0}")]
    Internal(String),
}

impl From<redis::RedisError> for RedisError {
    fn from(err: redis::RedisError) -> Self {
        Self::Internal(err.to_string())
    }
}
