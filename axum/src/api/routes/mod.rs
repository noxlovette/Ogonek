pub mod admin_routes;
pub mod auth_routes;
pub mod content_routes;
pub mod core_routes;
pub mod file_routes;
pub mod notification_routes;
pub mod user_routes;
pub use admin_routes::*;
pub use auth_routes::*;
use axum::http::{Method, StatusCode};
pub use content_routes::*;
pub use core_routes::*;
pub use file_routes::file_routes;
pub use notification_routes::notification_routes;
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

use crate::{schema::AppState, tools::extractors::REQUEST_ID_HEADER};

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
        .nest("/content", content_routes())
        .nest("/admin", admin_routes())
}

fn public_routes() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

pub fn root(state: AppState, cors: String) -> Result<Router, anyhow::Error> {
    let router = Router::new()
        .nest("/api/v1", api_routes())
        .merge(public_routes())
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
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    async fn app() -> Router {
        let state = AppState::test().await.unwrap();
        let cors = std::env::var("CORS").expect("CORS needs to be set");

        root(state, cors).unwrap()
    }

    #[tokio::test]
    async fn hello_world() {
        let app = app().await;

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
    #[tokio::test]
    async fn not_found() {
        let app = app().await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();

        assert_eq!(&body[..], b"The requested endpoint was not found");
    }
}
