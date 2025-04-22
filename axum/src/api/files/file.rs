use crate::api::error::APIError;
use crate::auth::Claims;
use crate::db::crud::files::file;
use crate::models::files::{File, FileListParams, FileUpdate};
use crate::s3::post::delete_s3;
use crate::schema::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

pub async fn fetch_file(
    State(state): State<AppState>,
    claims: Claims,
    Path(file_id): Path<String>,
) -> Result<Json<File>, APIError> {
    let file = file::find_by_id(&state.db, &file_id, &claims.sub).await?;

    Ok(Json(file))
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
