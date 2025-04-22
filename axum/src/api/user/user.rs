use crate::api::error::APIError;
use crate::auth::password::hash_password;
use crate::auth::Claims;
use crate::schema::AppState;
use axum::extract::Json;
use axum::extract::State;

use crate::models::users::{User, UserUpdate};

pub async fn fetch_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<User>, APIError> {
    tracing::info!("Attempting to fetch user");
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT username, email, role, id, name, pass, verified
        FROM "user"
        WHERE id = $1
        "#,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error when fetching user: {:?}", e);
        APIError::Database(e)
    })?
    .ok_or_else(|| APIError::NotFound("User not found".into()))?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<User>, APIError> {
    let user = sqlx::query_as!(
        User,
        r#"
        DELETE FROM "user"
        WHERE id = $1
        RETURNING username, email, role, id, name, pass, verified
        "#,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error when deleting user: {:?}", e);
        APIError::Database(e)
    })?
    .ok_or_else(|| APIError::NotFound("User not found".into()))?;

    tracing::info!("User deleted successfully");
    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<UserUpdate>,
) -> Result<Json<User>, APIError> {
    tracing::info!("Attempting update for user");

    let hashed_pass = match payload.pass {
        Some(ref pass) => Some(hash_password(pass).map_err(|_| APIError::PasswordHash)?),
        None => None,
    };

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE "user"
        SET
            name = COALESCE($1, name),
            username = COALESCE($2, username),
            email = COALESCE($3, email),
            pass = COALESCE($4, pass),
            role = COALESCE($5, role),
            verified = COALESCE($6, verified)
        WHERE id = $7
        RETURNING username, email, role, id, name, pass, verified
        "#,
        payload.name,
        payload.username,
        payload.email,
        hashed_pass,
        payload.role,
        payload.verified,
        claims.sub
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        // Handle specific constraint violations for better error messages
        if let sqlx::Error::Database(dbe) = &e {
            if let Some(constraint) = dbe.constraint() {
                if constraint == "user_username_key" {
                    return APIError::AlreadyExists("Username already taken".into());
                }
                if constraint == "user_email_key" {
                    return APIError::AlreadyExists("Email already taken".into());
                }
            }
        }
        tracing::error!("Database error when updating user: {:?}", e);
        APIError::Database(e)
    })?
    .ok_or_else(|| APIError::NotFound("User not found".into()))?;

    tracing::info!("User update successful");
    Ok(Json(user))
}
