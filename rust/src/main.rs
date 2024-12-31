use axum::routing::get;
use axum::Router;
use rust::api::user::test_user_endpoint;
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
        .route("/test", get(test_user_endpoint))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
