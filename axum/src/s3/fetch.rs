use crate::schema::AppState;
use crate::error::AppError;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::Json;
use serde_json::json;
use bytes::BytesMut;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub async fn download_file(
    State(state): State<AppState>,
    Path(encoded_key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    
    // validate the incoming key
    let key = BASE64.decode(encoded_key)
    .map_err(|_| AppError::BadRequest("Invalid base64 encoding".into()))?;

    let key_str = String::from_utf8(key)
    .map_err(|_| AppError::BadRequest("Invalid UTF-8 in decoded key".into()))?;


    // Get the object from S3
    let mut object = state.s3
        .get_object()
        .bucket(state.bucket_name)
        .key(&key_str)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get object from S3: {}", e)))?;
    
    // Get content type if available
    let content_type = object.content_type()
        .unwrap_or("application/octet-stream")
        .to_string();
    
    // Get content length if available
    let content_length = object.content_length();
    
    // Create header map
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_str(&content_type).unwrap());
    
    if let Some(length) = content_length {
        headers.insert("Content-Length", HeaderValue::from(length as u64));
    }
    
    // Set filename for download
    let filename = key_str.split('/').last().unwrap_or("download");
    headers.insert(
        "Content-Disposition", 
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename)).unwrap()
    );
    
    // Collect the body into bytes
    let mut bytes = BytesMut::new();
    while let Some(chunk) = object.body.try_next().await
        .map_err(|e| AppError::Internal(format!("Failed to read object body: {}", e)))? {
        bytes.extend_from_slice(&chunk);
    }
    
    // Return the response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", filename))
        .body(axum::body::Body::from(bytes.freeze()))
        .map_err(|e| AppError::Internal(format!("Failed to build response: {}", e)))?)
}

pub async fn stream_file(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {

    // Get the object from S3
    let object = state.s3
        .get_object()
        .bucket(state.bucket_name)
        .key(&key)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to get object from S3: {}", e)))?;
    
    // Get content type and other metadata
    let content_type = object.content_type()
        .unwrap_or("application/octet-stream")
        .to_string();
    

    // one of the many fallbacks to the filename. it is stored in the db so this is technically dead code
    let filename = key.split('/').last().unwrap_or("download");
    
    // Create a stream from the body
    let stream = object.body.into_async_read();
    let body = axum::body::Body::from_stream(tokio_util::io::ReaderStream::new(stream));
    
    // Return the streaming response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .header("Content-Disposition", format!("attachment; filename=\"{}\"", filename))
        .body(body)
        .map_err(|e| AppError::Internal(format!("Failed to build response: {}", e)))?)
}


pub async fn get_presigned_url(
    State(state): State<AppState>,
    Path(encoded_key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Decode the base64 key
    let key = BASE64.decode(encoded_key)
        .map_err(|_| AppError::BadRequest("Invalid base64 encoding".into()))?;
    
    let key_str = String::from_utf8(key)
        .map_err(|_| AppError::BadRequest("Invalid UTF-8 in decoded key".into()))?;
    
    // Create a presigned request that expires in 15 minutes
    let presigned_req = state.s3
        .get_object()
        .bucket(state.bucket_name)
        .key(&key_str)
        .presigned(aws_sdk_s3::presigning::PresigningConfig::expires_in(
            std::time::Duration::from_secs(15 * 60)
        )?)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create presigned URL: {}", e)))?;
    
    let presigned_url = presigned_req.uri().to_string();
    
    Ok((StatusCode::OK, Json(json!({ "url": presigned_url }))))
}

pub async fn check_s3_connection(
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    let result = state.s3.list_buckets().send().await
        .map_err(|e| {
            tracing::error!("S3 connection test failed: {:?}", e);
            AppError::Internal(format!("S3 connection test failed: {}", e))
        })?;
    
    let bucket_count = result.buckets().len();
    let bucket_names: Vec<&str> = result.buckets()
        .iter()
        .filter_map(|b| b.name())
        .collect();
    
    tracing::info!("Successfully connected to S3. Found {} buckets: {:?}", 
        bucket_count, bucket_names);
    
    Ok(StatusCode::OK)
}