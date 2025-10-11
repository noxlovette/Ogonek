use axum::extract::{Query, State};
use ogonek_db::core::account::{
    self,
    user::{read_by_id, read_email},
};
use ogonek_types::EmailVerificationQuery;
use reqwest::StatusCode;

use crate::{AppError, AppState, Claims, api::AUTH_TAG, services::generate_secure_token};

/// Generates the invite link for the teacher
#[utoipa::path(
    post,
    path = "/resend_email",
    tag = AUTH_TAG,
    responses(
        (status = 202, description = "Confirmation link resent"),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Invalid token")
    )
)]
pub async fn resend_verification(
    State(mut state): State<AppState>,
    claims: Claims,
) -> Result<StatusCode, AppError> {
    let token = generate_secure_token();
    let user = read_by_id(&state.db, &claims.sub).await.ok();

    match user {
        Some(user) => {
            state
                .redis
                .set_verification_token(&user.email, &token, None)
                .await?;

            tokio::spawn(async move {
                if let Err(e) = state
                    .ses
                    .send_confirm_email(&user.email, &user.name, &user.role.to_string(), &token)
                    .await
                {
                    tracing::error!("Error sending verification token: {e}")
                }
            });
            Ok(StatusCode::ACCEPTED)
        }
        None => Ok(StatusCode::NOT_FOUND),
    }
}

/// Confirms email verification using a token
#[utoipa::path(
    post,
    path = "/confirm_email",
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
    State(mut state): State<AppState>,
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
