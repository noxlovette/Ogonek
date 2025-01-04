use crate::auth::error::AuthError;
use crate::auth::helpers::{generate_token, hash_password};
use crate::db::helpers::FromQuery;
use crate::db::init::AppState;
use crate::models::users::{AuthPayload, SignUpPayload, User};
use axum::extract::Json;
use axum::extract::State;
use axum::response::Response;

pub async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>, // TODO adjust the error types to accomodate for 'not found disconnected blabla and other db errors'
) -> Result<Response, AuthError> {
    tracing::info!("Attempting sign-in for user: {:?}", payload.username);

    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    let db = &state.db;

    let username = payload.username.clone();
    let pass = payload.pass.clone();

    dbg!("hash", &pass);

    let result: Vec<User> = db
        .query("SELECT * FROM user WHERE username = $username AND crypto::argon2::compare(password, $pass)")
        .bind(("username", username))
        .bind(("pass", pass))
        .await?
        .take(0)?;

    let logged_in_user =
        User::from_query_result(result).map_err(|_| AuthError::WrongCredentials)?;

    let token = generate_token(&logged_in_user)?;

    Ok(logged_in_user.into_response(token))
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Response, AuthError> {
    tracing::info!("Creating user");
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let db = &state.db;
    let hashed_password = hash_password(&payload.pass)?;

    // Clone the values before the query since we'll need them later for Claims
    let name = payload.name.clone();
    let username = payload.username.clone();
    let email = payload.email.clone();
    let role = payload.role.clone();

    let result: Vec<User> = db
        .query(
            r#"
        CREATE user SET
            name = $name,
            username = $username,
            email = $email,
            password = $password,
            role = $role,
            joined_at = time::now();
        RETURN SELECT * FROM user WHERE username=$username;
        "#,
        )
        .bind(("name", name.clone()))
        .bind(("username", username.clone()))
        .bind(("email", email.clone()))
        .bind(("password", hashed_password.clone()))
        .bind(("role", role.clone()))
        .await?
        .take(0)?;

    let created_user = User::from_query_result(result).map_err(|_| AuthError::SignUpFail)?;

    let token = generate_token(&created_user)?;

    Ok(created_user.into_response(token))
}
