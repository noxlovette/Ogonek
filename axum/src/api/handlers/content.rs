use crate::api::error::APIError;
use crate::db::crud::content;
use crate::{openapi::CONTENT_TAG, schema::AppState, types::ContentPublic};
use axum::{
    Json,
    extract::{Path, State},
};

/// Fetches content by slug (public endpoint)
#[utoipa::path(
    get,
    path = "/{slug}",
    params(
        ("slug" = String, Path, description = "Content Slug")
    ),
    tag = CONTENT_TAG ,responses(
        (status = 200, description = "Content retrieved successfully", body = ContentPublic),
        (status = 404, description = "Content not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn fetch_content(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ContentPublic>, APIError> {
    let content = content::find_by_slug(&state.db, &slug).await?;

    Ok(Json(content))
}
