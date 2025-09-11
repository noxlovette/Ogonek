use crate::api::error::APIError;
use crate::api::{ADMIN_TAG, LESSON_TAG};
use crate::auth::Claims;
use crate::db::crud::content;
use crate::schema::AppState;
use crate::types::{Content, UpdateContent};
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;

/// Fetches content by id (admin interface)
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Content ID")
    ),
    tag = ADMIN_TAG ,responses(
        (status = 200, description = "Content retrieved successfully", body = Content),
        (status = 404, description = "Content not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn fetch_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Content>, APIError> {
    let content = content::find_by_id(&state.db, &id).await?;

    Ok(Json(content))
}
/// All content from the website
#[utoipa::path(
    get,
    path = "",
    tag = ADMIN_TAG, responses(
        (status = 200, description = "Content retrieved successfully", body = Vec<Content>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")

    )
)]
pub async fn list_content(State(state): State<AppState>) -> Result<Json<Vec<Content>>, APIError> {
    let content = content::find_all(&state.db).await?;

    Ok(Json(content))
}

/// Creates new content
#[utoipa::path(
    post,
    path = "",
    tag = ADMIN_TAG, responses(
        (status = 201, description = "Content created successfully", body = String),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_content(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<String>, APIError> {
    let id = content::create(&state.db, &claims.sub).await?;
    Ok(Json(id))
}
/// Deletes content
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Content ID")
    ),
    tag = LESSON_TAG,responses(
        (status = 204, description = "Content deleted successfully"),
        (status = 404, description = "Content not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    content::delete(&state.db, &id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Updates content
#[utoipa::path(
    patch,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "content ID")
    ),
    request_body = UpdateContent,
    tag = LESSON_TAG,responses(
        (status = 204, description = "content updated successfully"),
        (status = 404, description = "content not found"),
        (status = 403, description = "Forbidden"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
    Json(payload): Json<UpdateContent>,
) -> Result<StatusCode, APIError> {
    content::update(&state.db, &id, &claims.sub, &payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
