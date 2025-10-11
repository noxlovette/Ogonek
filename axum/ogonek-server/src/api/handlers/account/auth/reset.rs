use axum::extract::State;
use reqwest::StatusCode;
use ogonek_db::core::account::user::read_email;

use crate::{AppError, AppState, Claims, api::AUTH_TAG, services::generate_secure_token};

/// Initiates the password reset process for the authenticated user
///
/// Generates a secure token and stores it for password reset verification.
#[utoipa::path(
    post,
    path = "/reset_password",
    tag = AUTH_TAG,
    responses(
        (status = 200, description = "Password reset"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid token")
    )
)]
pub async fn reset_password(
    State(mut state): State<AppState>,
    claims: Claims,
) -> Result<StatusCode, AppError> {
    let token = generate_secure_token();
    let email = read_email(&state.db, &claims.sub).await.ok();

    match email {
        Some(email) => {
            state
                .redis
                .set_verification_token(&email, &token, None)
                .await?;
            Ok(StatusCode::ACCEPTED)
        }
        None => Ok(StatusCode::NOT_FOUND),
    }
}
