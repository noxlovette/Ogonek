// src/db.rs
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::r2d2;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

// Initialize the pool in your main function or at program start
pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// Helper function to get a connection from the pool
pub fn get_conn(pool: &DbPool) -> Result<DbConn, r2d2::PoolError> {
    pool.get()
}