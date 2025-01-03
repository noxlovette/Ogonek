// this module will interact with the database directly. if there is a connection involved, it should be here
pub mod error;

// initiate the DB and set it as static
use std::sync::Arc;
use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    Surreal,
};

use dotenvy::dotenv;
use std::sync::LazyLock;

use std::fs;

pub static NAMESPACE: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    std::env::var("NAMESPACE").expect("NAMESPACE must be set")
});

pub static DATABASE: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    std::env::var("DATABASE").expect("DATABASE must be set")
});

// this is the state that will be passed to the router. DB
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Surreal<Client>>,
}

pub async fn init_db() -> Result<Arc<Surreal<Client>>, Box<dyn std::error::Error>> {
    dotenv().ok();
    let username: String = std::env::var("DB_USERNAME").expect("DB USERNAME must be set");
    let password: String = std::env::var("DB_PASSWORD").expect("DB PASSWORD must be set");
    let url: String = std::env::var("DB_URL").expect("DB URL must be set");

    let db: Arc<Surreal<Client>> = Arc::new(Surreal::new::<Wss>(url).await?);

    db.signin(Root {
        username: &username,
        password: &password,
    })
    .await?;

    db.use_ns(&*NAMESPACE).use_db(&*DATABASE).await?;

    // let auth_query = fs::read_to_string("src/db/queries/auth.surql").expect("file not found");

    // db.query(auth_query).await?;

    tracing::info!("DB initialized");
    Ok(db)
}
