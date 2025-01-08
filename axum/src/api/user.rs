use crate::auth::helpers::hash_password;
use crate::auth::jwt::Claims;
use crate::db::error::DbError;
use crate::db::init::AppState;
use axum::extract::Json;
use axum::extract::State;

use crate::models::users::{User, UserUpdate};

pub async fn fetch_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<User>, DbError> {
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
        eprintln!("{:?}", e);
        DbError::Db
    })?
    .ok_or(DbError::NotFound)?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<User>, DbError> {
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
        eprintln!("{:?}", e);
        DbError::Db
    })?
    .ok_or(DbError::NotFound)?;

    tracing::info!("Deleted");
    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<UserUpdate>,
) -> Result<Json<User>, DbError> {
    tracing::info!("Attempting update for user");

    let hashed_pass = match payload.pass {
        Some(ref pass) => Some(hash_password(pass)?),
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
        eprintln!("{:?}", e);
        DbError::Db
    })?
    .ok_or(DbError::NotFound)?;

    tracing::info!("User update successful");
    Ok(Json(user))
}
