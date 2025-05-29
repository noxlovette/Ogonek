use axum::{
    http::{HeaderName, HeaderValue, Method, Request, StatusCode},
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
    trace::TraceLayer,
};
use tracing::{error, info_span};

const REQUEST_ID_HEADER: &str = "x-request-id";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cors = std::env::var("CORS").expect("CORS needs to be set");
    init_logging().await;

    let state = AppState::new().await?;
    let cleanup_state = state.clone();
    tokio::spawn(async move {
        daily_cleanup(cleanup_state).await;
    });

    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").expect("SENTRY_DSN must be set"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
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

    let app = Router::new()
        .merge(protected_routes)
        .route("/health", get(health_check))
        .fallback(handler_404)
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    HeaderName::from_static(REQUEST_ID_HEADER).clone(),
                    MakeRequestUuid,
                ))
                .layer(
                    TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                        let request_id = request.headers().get(REQUEST_ID_HEADER);
                        match request_id {
                            Some(request_id) => info_span!(
                                "http_request",
                                request_id = ?request_id
                            ),
                            None => {
                                error!("could not extract request_id");
                                info_span!("http_request")
                            }
                        }
                    }),
                )
                .layer(PropagateRequestIdLayer::new(HeaderName::from_static(
                    REQUEST_ID_HEADER,
                )))
                .layer(TimeoutLayer::new(std::time::Duration::from_secs(30)))
                .layer(axum::extract::DefaultBodyLimit::max(100 * 1024 * 1024))
                .layer(
                    CorsLayer::new()
                        .allow_origin(cors.parse::<HeaderValue>().unwrap())
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::DELETE,
                            Method::PUT,
                            Method::PATCH,
                        ]),
                ),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here")
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
