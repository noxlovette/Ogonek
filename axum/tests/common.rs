use axum::Router;
use ogonek::schema::AppState;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

pub async fn create_test_app_state(db: PgPool) -> AppState {
    // Mock S3 client for tests - you might want to use localstack here
    let s3_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("us-east-1"))
        .load()
        .await;
    let s3_client = aws_sdk_s3::Client::new(&s3_config);

    AppState {
        db,
        s3: s3_client,
        bucket_name: "test-bucket".to_string(),
    }
}

pub async fn create_test_app(state: AppState) -> Router {
    use axum::routing::get;
    use ogonek::api::routes;
    use ogonek::tools::middleware::api_key::validate_api_key;

    let protected_routes = Router::new()
        .nest("/lesson", routes::lesson_routes::lesson_routes())
        .nest("/user", routes::user_routes::user_routes())
        .nest("/task", routes::task_routes::task_routes())
        .nest("/student", routes::student_routes::student_routes())
        .nest("/auth", routes::auth_routes::auth_routes())
        .nest("/profile", routes::profile_routes::profile_routes())
        .nest("/deck", routes::deck_routes::deck_routes())
        .nest("/s3", routes::s3_routes::s3_routes())
        .nest("/file", routes::file_routes::file_routes())
        .layer(axum::middleware::from_fn(validate_api_key));

    Router::new()
        .merge(protected_routes)
        .route("/health", get(|| async { "OK" }))
        .with_state(state)
        .layer(CorsLayer::permissive())
}
