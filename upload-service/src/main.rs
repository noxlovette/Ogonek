use axum::{
    extract::DefaultBodyLimit, http::StatusCode, response::IntoResponse, routing::{get, post}, Router
};
use tower_http::cors::CorsLayer;
use upload_service::api::file::{download_handler, upload_handler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env vars
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Get config from env
    let upload_path = std::env::var("UPLOAD_PATH").unwrap_or_else(|_| "./uploads".to_string());

    // Ensure upload directory exists
    tokio::fs::create_dir_all(&upload_path)
        .await
        .expect("Failed to create upload directory");

    // Build our application
    let app = Router::new()
        .route("/upload", post(upload_handler))
        .route("/download/{filename}", get(download_handler))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive()) //TODO PROD CHECK
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        .layer(axum::middleware::from_fn(
            upload_service::api::key::validate_api_key,
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;

    tracing::info!("🚀 Upload service listening on 0.0.0.0:3001");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
