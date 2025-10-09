use axum::extract::State;
use ogonek_db::core::account::{self};

use crate::{AppError, AppState, Claims};

pub async fn send_welcome_email(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<(), AppError> {
    let user = account::user::read_by_id(&state.db, &claims.sub).await?;
    state
        .ses
        .send_confirm_email(&user.email, &user.name, &claims.role.to_string())
        .await?;

    Ok(())
}
