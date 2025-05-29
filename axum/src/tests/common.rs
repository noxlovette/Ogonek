// tests/common/mod.rs
use axum::Router;
use ogonek::schema::AppState;
use sqlx::PgPool;
use std::sync::Once;
use tower_http::cors::CorsLayer;

static INIT: Once = Once::new();

pub fn init_test_env() {
    INIT.call_once(|| {
        std::env::set_var("API_KEY", "test-key");
        std::env::set_var(
            "JWT_PRIVATE_KEY",
            include_str!("../fixtures/test_private_key.pem"),
        );
        std::env::set_var(
            "JWT_PUBLIC_KEY",
            include_str!("../fixtures/test_public_key.pem"),
        );
        std::env::set_var("S3_REGION", "fr-par");
        std::env::set_var("S3_URL", "https://s3.fr-par.scw.cloud");
        std::env::set_var("S3_ACCESS_KEY", "test-access-key");
        std::env::set_var("S3_SECRET_KEY", "test-secret-key");
        std::env::set_var("S3_BUCKET_NAME", "test-bucket");
    });
}

pub async fn create_test_app_state(db: PgPool) -> AppState {
    init_test_env();

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

// Helper to create a test user and return their tokens
pub async fn create_authenticated_user(
    app: Router,
    username: &str,
    email: &str,
    password: &str,
) -> (String, String) {
    // (access_token, refresh_token)
    use axum::body::Body;
    use axum::http::Request;
    use serde_json::json;
    use tower::ServiceExt;

    let signup_payload = json!({
        "name": format!("Test {}", username),
        "username": username,
        "email": email,
        "pass": password,
        "role": "student"
    });

    let signup_request = Request::builder()
        .method("POST")
        .uri("/auth/signup")
        .header("content-type", "application/json")
        .header("x-api-key", "test-key")
        .body(Body::from(signup_payload.to_string()))
        .unwrap();

    let _signup_response = app.clone().oneshot(signup_request).await.unwrap();

    let signin_payload = json!({
        "username": username,
        "pass": password
    });

    let signin_request = Request::builder()
        .method("POST")
        .uri("/auth/signin")
        .header("content-type", "application/json")
        .header("x-api-key", "test-key")
        .body(Body::from(signin_payload.to_string()))
        .unwrap();

    let signin_response = app.oneshot(signin_request).await.unwrap();
    let signin_body = hyper::body::to_bytes(signin_response.into_body())
        .await
        .unwrap();
    let signin_result: serde_json::Value = serde_json::from_slice(&signin_body).unwrap();

    let access_token = signin_result["accessToken"]["token"]
        .as_str()
        .unwrap()
        .to_string();
    let refresh_token = signin_result["refreshToken"]["token"]
        .as_str()
        .unwrap()
        .to_string();

    (access_token, refresh_token)
}
