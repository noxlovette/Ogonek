use axum::routing::{get, post};
use axum::Router;
use rust::db::init_db;
use rust::db::AppState;
use rust::tools::logging::init_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let db = init_db().await?;
    let state = AppState {
        db: init_db().await?,
    };

    init_logging().await;

    let app = Router::new()
        .route("/", get(root))
        .route("/signup", post(rust::api::auth::signup))
        .route("/signin", post(rust::api::auth::authorize))
        .route("/user", get(rust::api::user::fetch_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
