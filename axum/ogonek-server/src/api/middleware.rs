use axum::{extract::Request, middleware::Next, response::Response};
use reqwest::StatusCode;

use crate::services::Claims;
pub async fn require_elevated_role(
    claims: Claims, // Custom extractors come after Request
    req: Request,   // Request should be first
    next: Next,
) -> Result<Response, StatusCode> {
    // Fix the logic - this was backwards
    if claims.role.hierarchy_level() < 2 {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(next.run(req).await)
}
