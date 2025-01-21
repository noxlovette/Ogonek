use axum::Json;
use axum::{
    extract::{Multipart, Path},
    http::{header, StatusCode},
    response::IntoResponse,
};
use std::path::PathBuf;
use tokio::fs;
use tracing::error;
use tracing::info;

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

pub async fn upload_handler(mut multipart: Multipart) -> Result<impl IntoResponse, StatusCode> {
    let upload_path = std::env::var("UPLOAD_PATH").unwrap_or_else(|_| "./uploads".to_string());

    if !PathBuf::from(&upload_path).exists() {
        fs::create_dir_all(&upload_path).await.map_err(|err| {
            error!("Failed to create upload directory: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    let mut unique_filename = String::new();

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

        let safe_filename = sanitize_filename::sanitize(&filename);
        unique_filename = format!("{}-{}", uuid::Uuid::new_v4(), safe_filename);

        let data = field.bytes().await.map_err(|err| {
            error!("Failed to read field data: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if data.len() as u64 > MAX_FILE_SIZE {
            error!("Payload too large:");
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }

        let file_path = PathBuf::from(&upload_path).join(&unique_filename);

        fs::write(&file_path, &data).await.map_err(|err| {
            error!("Failed to write file {:?}: {:?}", file_path, err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        info!("Uploaded file: {}", safe_filename);
    }

    Ok(Json(unique_filename))
}

pub async fn download_handler(
    Path(filename): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let upload_path = std::env::var("UPLOAD_PATH").unwrap_or_else(|_| "./uploads".to_string());
    let file_path = PathBuf::from(&upload_path).join(&filename);

    // Basic path traversal protection
    if !file_path.starts_with(&upload_path) {
        error!("Path traversal attempt detected");
        return Err(StatusCode::BAD_REQUEST);
    }

    // Read the file
    let data = fs::read(&file_path).await.map_err(|err| {
        error!("Failed to read file {:?}: {:?}", file_path, err);
        StatusCode::NOT_FOUND
    })?;

    // Guess MIME type from filename
    let mime_type = mime_guess::from_path(&file_path)
        .first_or_octet_stream()
        .to_string();

    // Build response with proper headers
    let headers = [
        (header::CONTENT_TYPE, mime_type),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        ),
    ];

    Ok((headers, data))
}
