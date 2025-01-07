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
        .nest("/auth", rust::routes::auth_routes::auth_routes())
        .nest("/lesson", rust::routes::lesson_routes::lesson_routes())
        .nest("/user", rust::routes::user_routes::user_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
