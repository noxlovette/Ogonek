use crate::{
    api::{ADMIN_TAG, error::APIError},
    app::AppState,
    services::{AuditBuilder, Claims, RequestMetadata},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use ogonek_db::{content, core::account::user, tracking::audit};
use ogonek_types::{Content, UpdateContent};

/// Retrieves content by ID for admin interface
///
/// Returns content details for administrative access.
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
    let content = content::read_by_id(&state.db, &id).await?;

    Ok(Json(content))
}
/// Lists all content for admin management
///
/// Returns all website content for administrative purposes.
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
    let content = content::read_all(&state.db).await?;

    Ok(Json(content))
}

/// Creates new content with audit logging
///
/// Creates a new content entry and logs the creation for audit purposes.
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
    metadata: RequestMetadata,
) -> Result<Json<String>, APIError> {
    let id = content::create(&state.db, &claims.sub).await?;
    let email = user::read_email(&state.db, &claims.sub).await?;
    let audit = AuditBuilder::new("content.operation", "CREATE", &claims, email)
        .resource_id(id.clone())
        .with_metadata(&metadata)
        .tag("content")
        .build();

    audit::create(&state.db, &audit).await?;
    Ok(Json(id))
}
/// Deletes content with audit logging
///
/// Removes content and creates an audit log entry for the deletion.
#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Content ID")
    ),
    tag = ADMIN_TAG,responses(
        (status = 204, description = "Content deleted successfully"),
        (status = 404, description = "Content not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn delete_content(
    State(state): State<AppState>,
    metadata: RequestMetadata,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<StatusCode, APIError> {
    content::delete(&state.db, &id).await?;

    let email = user::read_email(&state.db, &claims.sub).await?;
    let audit = AuditBuilder::new("content.operation", "DELETE", &claims, email)
        .resource_id(id.clone())
        .with_metadata(&metadata)
        .tag("content")
        .build();

    audit::create(&state.db, &audit).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Updates content with audit logging
///
/// Modifies content properties and logs the update operation.
#[utoipa::path(
    patch,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "content ID")
    ),
    request_body = UpdateContent,
    tag = ADMIN_TAG, responses(
        (status = 204, description = "content updated successfully"),
        (status = 404, description = "content not found"),
        (status = 403, description = "Forbidden"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn update_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
    metadata: RequestMetadata,
    claims: Claims,
    Json(payload): Json<UpdateContent>,
) -> Result<StatusCode, APIError> {
    content::update(&state.db, &id, &claims.sub, &payload).await?;
    let email = user::read_email(&state.db, &claims.sub).await?;
    let audit = AuditBuilder::new("content.operation", "UPDATE", &claims, email)
        .resource_id(id.clone())
        .with_metadata(&metadata)
        .tag("content")
        .build();

    audit::create(&state.db, &audit).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Publishes content
///
/// Marks content as published and available to users.
#[utoipa::path(
    put,
    path = "/{id}/publish",
    params(
        ("id" = String, Path, description = "content ID")
    ),
    tag = ADMIN_TAG, responses(
        (status = 204, description = "content published successfully"),
        (status = 404, description = "content not found"),
        (status = 403, description = "Forbidden"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn publish_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    content::publish(&state.db, &id, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Unpublishes content
///
/// Marks content as unpublished and removes it from public access.
#[utoipa::path(
    delete,
    path = "/{id}/publish",
    params(
        ("id" = String, Path, description = "content ID")
    ),
    tag = ADMIN_TAG, responses(
        (status = 204, description = "content published successfully"),
        (status = 404, description = "content not found"),
        (status = 403, description = "Forbidden"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn unpublish_content(
    State(state): State<AppState>,
    Path(id): Path<String>,
    claims: Claims,
) -> Result<StatusCode, APIError> {
    content::unpublish(&state.db, &id, &claims.sub).await?;

    Ok(StatusCode::NO_CONTENT)
}
