mod admin_routes;
mod auth_routes;
mod core_routes;
mod debug_routes;
mod file_routes;
mod notification_routes;
mod public_routes;
mod user_routes;

pub use admin_routes::*;
pub use auth_routes::*;
use axum::http::{Method, StatusCode};
pub use core_routes::*;
pub use file_routes::file_routes;
pub use notification_routes::notification_routes;
pub use public_routes::*;

use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
};
pub use user_routes::user_routes;

use axum::{
    Router,
    http::{HeaderName, HeaderValue},
    response::IntoResponse,
    routing::get,
};

use crate::{AppState, api::routes::debug_routes::debug_routes, services::REQUEST_ID_HEADER};

fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
        .nest("/lessons", lesson_routes())
        .nest("/tasks", task_routes())
        .nest("/decks", deck_routes())
        .nest("/files", file_routes())
        .nest("/learn", learn_routes())
        .nest("/notifications", notification_routes())
        .nest("/state", state_routes())
        .nest("/calendars", calendar_routes())
        .nest("/admin", admin_routes())
        .nest("/debug", debug_routes())
}

fn public_routes() -> Router<AppState> {
    Router::new().nest("/public", content_routes())
}

fn router() -> Router<AppState> {
    Router::new().merge(api_routes()).merge(public_routes())
}

pub fn root(state: AppState, cors: String) -> Result<Router, anyhow::Error> {
    let router = Router::new()
        .nest("/api/v1", router())
        .merge(public_routes())
        .route("/health", get(health_check))
        .fallback(handler_404)
        .with_state(state)
        .layer(SetRequestIdLayer::new(
            HeaderName::from_static(REQUEST_ID_HEADER),
            MakeRequestUuid,
        ))
        .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
            REQUEST_ID_HEADER,
        )))
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
        .layer(axum::extract::DefaultBodyLimit::max(100 * 1024 * 1024))
        .layer(
            CorsLayer::new()
                .allow_origin(
                    cors.parse::<HeaderValue>()
                        .map_err(|e| anyhow::anyhow!("CORS header parsing failed: {}", e))?,
                )
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                    Method::PATCH,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                    HeaderName::from_static("x-api-key"),
                ]),
        )
        .layer(sentry_tower::SentryHttpLayer::new().enable_transaction());

    Ok(router)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested endpoint was not found",
    )
}

async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::Json(serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now(),
            "version": env!("CARGO_PKG_VERSION")
        })),
    )
}
