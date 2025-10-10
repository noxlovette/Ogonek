use crate::{RedisClient, RedisError};

impl RedisClient {
    /// Store email verification token with TTL (default 1 hour)
    pub async fn set_verification_token(
        &mut self,
        email: &str,
        token: &str,
        ttl_seconds: Option<u64>,
    ) -> Result<(), RedisError> {
        let key = format!("email_verify:{}", token);
        let ttl = ttl_seconds.unwrap_or(3600); // 1 hour default

        redis::cmd("SETEX")
            .arg(&key)
            .arg(ttl)
            .arg(email)
            .query_async(&mut self.con)
            .await
            .map_err(Into::into)
    }

    /// Get and delete verification token (atomic operation)
    /// Returns Some(email) if token exists, None if expired/invalid
    pub async fn verify_and_consume_token(
        &mut self,
        token: &str,
    ) -> Result<Option<String>, RedisError> {
        let key = format!("email_verify:{}", token);

        let email: Option<String> = redis::cmd("GETDEL")
            .arg(&key)
            .query_async(&mut self.con)
            .await?;

        Ok(email)
    }
    // Check if verification token exists (doesn't consume it)
    pub async fn token_exists(&mut self, token: &str) -> Result<bool, RedisError> {
        let key = format!("email_verify:{}", token);

        let exists: bool = redis::cmd("EXISTS")
            .arg(&key)
            .query_async(&mut self.con)
            .await?;

        Ok(exists)
    }

    /// Delete token without getting value (for cleanup/invalidation)
    pub async fn invalidate_token(&mut self, token: &str) -> Result<(), RedisError> {
        let key = format!("email_verify:{}", token);

        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut self.con)
            .await?;

        Ok(())
    }
}
