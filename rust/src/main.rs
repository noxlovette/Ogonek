use axum::routing::{get, post};
use axum::Router;
use rust::db::init::init_db;
use rust::db::init::AppState;
use rust::tools::logging::init_logging;

// use rust::api::user::{delete_user, fetch_user, list_users, update_user};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging().await;
    let state = AppState {
        db: init_db().await?,
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/signup", post(rust::api::auth::signup))
        .route("/signin", post(rust::api::auth::authorize))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
