use crate::schema::AppState;
use crate::error::AppError;
use axum::extract::{Multipart, State, Query};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use serde_json::json;
use aws_sdk_s3::primitives::ByteStream;
use mime_guess::from_path;
use sqlx::types::time::OffsetDateTime;
use nanoid::nanoid;
use crate::models::files::FileListParams;

#[derive(Debug, Serialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub s3_key: String,
    pub path: String,
    pub mime_type: Option<String>,
    pub size: i64,
    pub is_folder: bool,
    pub parent_id: Option<String>,
    pub owner_id: String,
    pub visibility: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn upload_file(
    State(state): State<AppState>,
    claims: crate::auth::jwt::Claims,
    Query(params): Query<FileListParams>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    // Check if parent folder exists if parent_id is provided
    let parent_id = if let Some(parent_id) = params.parent_id {
        let folder_exists = sqlx::query!(
            r#"
            SELECT 1 as "exists!: bool" FROM files
            WHERE id = $1 AND (
                owner_id = $2 AND is_folder = true
            )
            "#,
            parent_id,
            claims.sub
        )
        .fetch_optional(&state.db)
        .await?
        .is_some();
        
        if !folder_exists {
            return Err(AppError::NotFound("Folder Not Found".into()));
        }
        Some(parent_id)
    } else {
        None
    };
    
    // Get the file field from multipart form
    let field = multipart.next_field().await
        .map_err(|e| AppError::BadRequest(format!("Failed to get form field: {}", e)))?
        .ok_or_else(|| AppError::BadRequest("No file field found in request".to_string()))?;
    
    // Extract file metadata
    let file_name = field.file_name()
        .ok_or_else(|| AppError::BadRequest("No filename provided".to_string()))?
        .to_string();
    
    // Get file content
    let file_data = field.bytes().await
        .map_err(|e| AppError::Internal(format!("Failed to read file data: {}", e)))?
        .to_vec();
    
    if file_data.is_empty() {
        return Err(AppError::BadRequest("File is empty".to_string()));
    }
    
    // Generate unique S3 key with user ID as prefix for basic organization
    let file_id = nanoid!();
    let file_extension = file_name.split('.').last().unwrap_or("");
    let s3_key = format!("user-files/{}/{}.{}", claims.sub, file_id, file_extension);
    
    // Get mime type from filename
    let mime_type = from_path(&file_name)
        .first_or_octet_stream()
        .to_string();
    
    // Create path for storage in DB
    let path = format!("/{}", file_name);
    
    // Log upload attempt
    tracing::info!("Uploading file to S3: name={}, key={}, size={}", file_name, s3_key, file_data.len());
    
    // Upload to S3
    state.s3
        .put_object()
        .bucket("ogonek-scaleway")
        .key(&s3_key)
        .body(ByteStream::from(file_data.clone()))
        .content_type(mime_type.clone())
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("S3 upload failed: {}", e)))?;
    
    // Create file record in the database
    let file_record = sqlx::query_as!(
        FileMetadata,
        r#"
        INSERT INTO files (
            id, name, s3_key, path, mime_type, size, is_folder, parent_id, owner_id, visibility
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, name, s3_key, path, mime_type, size, is_folder, parent_id, owner_id, visibility, created_at, updated_at
        "#,
        file_id,
        file_name,
        s3_key,
        path,
        Some(mime_type),
        file_data.len() as i64,
        false,
        parent_id,
        claims.sub,
        "private",
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| AppError::Database(e))?;
    
    // Return success response with file metadata
    Ok(Json(json!({
        "message": "File uploaded successfully",
        "file": file_record
    })))
}