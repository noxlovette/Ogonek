use axum::{
    http::{HeaderName, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use ogonek::schema::AppState;
use ogonek::tools::daemons::task_cleanup::daily_cleanup;
use ogonek::tools::logging::init_logging;
use ogonek::tools::middleware::api_key::validate_api_key;
use tower::ServiceBuilder;
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

    let protected_routes = Router::new()
        .nest(
            "/lesson",
            ogonek::api::routes::lesson_routes::lesson_routes(),
        )
        .nest("/user", ogonek::api::routes::user_routes::user_routes())
        .nest("/task", ogonek::api::routes::task_routes::task_routes())
        // .nest("/notes", ogonek::api::routes::notes_routes::notes_routes())
        .nest(
            "/student",
            ogonek::api::routes::student_routes::student_routes(),
        )
        .nest("/auth", ogonek::api::routes::auth_routes::auth_routes())
        .nest(
            "/profile",
            ogonek::api::routes::profile_routes::profile_routes(),
        )
        .nest("/deck", ogonek::api::routes::deck_routes::deck_routes())
        .nest("/s3", ogonek::api::routes::s3_routes::s3_routes())
        .nest("/file", ogonek::api::routes::file_routes::file_routes())
        .layer(axum::middleware::from_fn(validate_api_key));

    tokio::spawn(async move {
        daily_cleanup(cleanup_state).await;
    });

    let app = Router::new()
        .merge(protected_routes)
        .route("/health", get(health_check))
        .fallback(handler_404)
        .with_state(state)
        .layer(
            ServiceBuilder::new()
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
                                .expect("CORS header values parsing failed"),
                        )
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::DELETE,
                            Method::PUT,
                            Method::PATCH,
                        ]),
                )
                .layer(sentry_tower::SentryHttpLayer::new().enable_transaction()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn main() {
    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").expect("SENTRY_DSN must be set"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: std::env::var("SENTRY_TRACE_RATE")
                .unwrap_or(1.0.to_string())
                .parse::<f32>()
                .unwrap_or(1.0),
            environment: Some(
                std::env::var("APP_ENV")
                    .expect("APP_ENV must be set")
                    .into(),
            ),
            // Capture user IPs and potentially sensitive headers when using HTTP server integrations
            // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
            send_default_pii: true,
            ..Default::default()
        },
    ));

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Tokio build runtime failed")
        .block_on(async {
            let _ = run_server().await;
        });
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
