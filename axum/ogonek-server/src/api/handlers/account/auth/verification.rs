use axum::extract::Query;
use ogonek_db::core::account::{self, user::read_email};
use ogonek_types::EmailVerificationQuery;
use reqwest::StatusCode;

use crate::{AppError, AppState, Claims, api::AUTH_TAG, services::generate_secure_token};

/// Generates the invite link for the teacher
#[utoipa::path(
    get,
    path = "/resend",
    tag = AUTH_TAG,
    responses(
        (status = 202, description = "Confirmation link resent"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid token")
    )
)]
pub async fn resend_verification(
    mut state: AppState,
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

/// Generates the invite link for the teacher
#[utoipa::path(
    get,
    path = "/confirm-email",
    tag = AUTH_TAG,
    params(
        ("token" = String, Query, description = "The confirmation token")
    ),
    responses(
        (status = 202, description = "Confirmation Successful"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid token")
    )
)]
pub async fn confirm_email(
    mut state: AppState,
    query: Query<EmailVerificationQuery>,
) -> Result<StatusCode, AppError> {
    let email = state.redis.verify_and_consume_token(&query.token).await?;

    match email {
        Some(email) => {
            account::auth::verify_email(&state.db, &email).await?;
            Ok(StatusCode::ACCEPTED)
        }
        None => Ok(StatusCode::NOT_FOUND),
    }
}
