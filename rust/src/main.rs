use axum::Router;
use rust::db::init::init_db;
use rust::db::init::AppState;
use rust::tools::logging::init_logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging().await;
    let state = AppState {
        db: init_db().await?,
    };

    // TODO implement rate limiting
    // TODO implement request signing
    // TODO API key

    let app = Router::new()
        .nest("/auth", rust::routes::auth_routes::auth_routes())
        .nest("/lesson", rust::routes::lesson_routes::lesson_routes())
        .nest("/user", rust::routes::user_routes::user_routes())
        .nest("/task", rust::routes::task_routes::task_routes())
        .nest("/notes", rust::routes::notes_routes::notes_routes())
        // .fallback(handler::not_found)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
