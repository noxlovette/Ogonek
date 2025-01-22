use axum::Json;
use axum::{
    extract::{Multipart, Path},
    http::{header, StatusCode},
    response::IntoResponse,
};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::error;
use tracing::info;

pub async fn upload_handler(mut multipart: Multipart) -> Result<impl IntoResponse, StatusCode> {
    let upload_path = std::env::var("UPLOAD_PATH").unwrap_or_else(|_| "./uploads".to_string());
    if !PathBuf::from(&upload_path).exists() {
        fs::create_dir_all(&upload_path).await.map_err(|err| {
            error!("Failed to create upload directory: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    let mut unique_filename = String::new();

    while let Some(mut field) = multipart.next_field().await.map_err(|err| {
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
        let safe_filename = filename
            .trim()
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '.')
            .collect::<String>();
        unique_filename = format!("{}-{}", uuid::Uuid::new_v4(), safe_filename);

        let file_path = PathBuf::from(&upload_path).join(&unique_filename);
        let mut file = tokio::fs::File::create(&file_path).await.map_err(|err| {
            error!("Failed to create file: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        while let Some(chunk) = field.chunk().await.map_err(|err| {
            error!("Failed to read chunk: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })? {
            file.write_all(&chunk).await.map_err(|err| {
                error!("Failed to write chunk: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }

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

    info!("guessed mime type");

    // Build response with proper headers
    let headers = [
        (header::CONTENT_TYPE, mime_type),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        ),
    ];

    info!("returning file {:?}", filename);
    Ok((headers, data))
}

//TODO ADD METHOD TRANSFORMER
