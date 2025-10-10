use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("Redis operation failed: {0}")]
    Redis(#[from] redis::RedisError),
}
