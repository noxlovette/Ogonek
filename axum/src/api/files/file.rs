use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::files::file;
use crate::models::files::{File, FileListParams, FileUpdate};
use crate::s3::get_presigned_url;
use crate::s3::post::delete_s3;
use crate::schema::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde_json::json;

pub async fn fetch_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
) -> Result<Json<File>, APIError> {
    let file = file::find_by_id(&state.db, &file_id, &claims.sub).await?;

    Ok(Json(file))
}

pub async fn fetch_presigned_url(
    State(state): State<AppState>,
    claims: Claims,
    Path(encoded_key): Path<String>,
) -> Result<impl IntoResponse, APIError> {
    tracing::debug!("Reached the presign url endpoint");
    let key = BASE64
        .decode(encoded_key)
        .map_err(|_| APIError::BadRequest("Invalid base64 encoding".into()))?;
    tracing::debug!("Decyphered base64 into url");
    let key_str = String::from_utf8(key)
        .map_err(|_| APIError::BadRequest("Invalid UTF-8 in decoded key".into()))?;

    let file_id = key_str
        .rsplit("/")
        .next()
        .unwrap()
        .split(".")
        .next()
        .unwrap();
    tracing::debug!("File ID decyphered");
    tracing::debug!(file_id);
    let file = file::find_by_id(&state.db, file_id, &claims.sub).await?;
    let presigned_url =
        get_presigned_url(&state.bucket_name, &state.s3, key_str, file.name).await?;

    Ok((StatusCode::OK, Json(json!({ "url": presigned_url }))))
}

pub async fn list_files(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<FileListParams>,
) -> Result<Json<Vec<File>>, APIError> {
    let files = file::find_all(&state.db, params, &claims.sub).await?;

    Ok(Json(files))
}

pub async fn update_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
    Json(payload): Json<FileUpdate>,
) -> Result<StatusCode, APIError> {
    file::update(&state.db, &file_id, &claims.sub, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
) -> Result<StatusCode, APIError> {
    let file = file::delete(&state.db, &file_id, &claims.sub).await?;
    if let Some(key) = &file.s3_key {
        delete_s3(key, &state).await?;
    } else {
        return Err(APIError::NotFound("S3 Object not Found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}
