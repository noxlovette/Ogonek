use axum::{
    extract::Multipart,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::path::PathBuf;
use tokio::fs;
use tower_http::cors::CorsLayer;
use tracing::error;
use tracing::info;

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
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive()); // You'll want to configure this for production

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;

    axum::serve(listener, app).await?;
    info!("🚀 Upload service listening on 0.0.0.0:3001");

    Ok(())
}

pub async fn upload_handler(mut multipart: Multipart) -> Result<impl IntoResponse, StatusCode> {
    let upload_path = std::env::var("UPLOAD_PATH").unwrap_or_else(|_| "./uploads".to_string());

    // Ensure upload directory exists
    if !PathBuf::from(&upload_path).exists() {
        fs::create_dir_all(&upload_path).await.map_err(|err| {
            error!("Failed to create upload directory: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    while let Some(field) = multipart.next_field().await.map_err(|err| {
        error!("Error processing multipart field: {:?}", err);
        StatusCode::BAD_REQUEST
    })? {
        let filename = field
            .file_name()
            .ok_or_else(|| {
                error!("Field missing filename");
                StatusCode::BAD_REQUEST
            })?
            .to_string();

        let data = field.bytes().await.map_err(|err| {
            error!("Failed to read field data: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let file_path = PathBuf::from(&upload_path).join(&filename);

        fs::write(&file_path, &data).await.map_err(|err| {
            error!("Failed to write file {:?}: {:?}", file_path, err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        info!("Uploaded file: {}", filename);
    }

    Ok(StatusCode::OK)
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
