use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use ogonek::api::routes::*;
use ogonek::schema::AppState;
use ogonek::tools::daemons::task_cleanup::daily_cleanup;
use ogonek::tools::logging::init_logging;
use ogonek::tools::middleware::api_key::validate_api_key;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
};

const REQUEST_ID_HEADER: &str = "x-request-id";

async fn run_server() -> anyhow::Result<()> {
    let _ = init_logging().await;
    let state = AppState::new().await?;
    let cors = std::env::var("CORS").expect("CORS needs to be set");
    let cleanup_state = state.clone();

    // Spawn background tasks
    tokio::spawn(async move {
        daily_cleanup(cleanup_state).await;
    });

    let api_v1 = Router::new()
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
        .nest("/lessons", lesson_routes())
        .nest("/tasks", task_routes())
        .nest("/decks", deck_routes())
        .nest("/s3", s3_routes())
        .nest("/learn", learn_routes()) // assuming you have this
        .layer(axum::middleware::from_fn(validate_api_key));

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/", get(|| async { "Ogonek API v1.0.0" }));

    // Main app assembly
    let app = Router::new()
        .nest("/api/v1", api_v1)
        .merge(public_routes)
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("🚀 Server starting on http://0.0.0.0:3000");
    tracing::info!("📚 API docs available at http://0.0.0.0:3000/docs");

    axum::serve(listener, app).await?;
    Ok(())
}

fn main() {
    dotenvy::dotenv().ok();

    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").expect("SENTRY_DSN must be set"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: std::env::var("SENTRY_TRACE_RATE")
                .unwrap_or_else(|_| "1.0".to_string())
                .parse::<f32>()
                .unwrap_or(1.0),
            environment: Some(
                std::env::var("APP_ENV")
                    .expect("APP_ENV must be set")
                    .into(),
            ),
            send_default_pii: true,
            ..Default::default()
        },
    ));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime")
        .block_on(async {
            if let Err(e) = run_server().await {
                tracing::error!("Server error: {}", e);
                std::process::exit(1);
            }
        });
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::Json(serde_json::json!({
            "error": "Not Found",
            "message": "The requested resource was not found",
            "status": 404
        })),
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
