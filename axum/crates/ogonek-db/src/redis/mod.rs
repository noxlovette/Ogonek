use redis::{AsyncCommands, aio::MultiplexedConnection};

pub use error::RedisError;
mod email;
mod error;

#[derive(Clone, Debug)]
pub struct RedisClient {
    con: MultiplexedConnection,
}

impl RedisClient {
    pub async fn new() -> Result<Self, error::RedisError> {
        let address = std::env::var("REDIS_URL").unwrap_or("redis://redis:6379/".to_string());

        let client = redis::Client::open(address)?;
        let con = client.get_multiplexed_async_connection().await?;

        Ok(Self { con })
    }

    // Add this so you can actually use it
    pub fn connection(&mut self) -> &mut MultiplexedConnection {
        &mut self.con
    }

    pub async fn test(mut self) -> Result<(), error::RedisError> {
        let _: () = self.con.set("key1", b"hello").await?;

        redis::cmd("SET")
            .arg(&["key2", "bar"])
            .exec_async(self.connection())
            .await?;

        let result: () = redis::cmd("MGET")
            .arg(&["key1", "key2"])
            .query_async(self.connection())
            .await?;

        println!("{:?}", result);

        Ok(())
    }
}
