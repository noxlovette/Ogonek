use axum::{
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    Router,
};
use rust::db::init::{init_db, AppState};
use rust::tools::logging::init_logging;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};

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
        .fallback(handler_404)
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(TimeoutLayer::new(std::time::Duration::from_secs(10)))
                .layer(
                    CorsLayer::new()
                        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::DELETE,
                            Method::PUT,
                            Method::PATCH,
                        ]),
                ),
        );

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}
