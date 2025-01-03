use crate::auth::{AuthError, Claims, KEYS};
use crate::db::error::DbError;
use crate::db::{AppState, DATABASE, NAMESPACE};
use crate::models::users::{AuthPayload, Credentials, SignUpCredentials, SignUpPayload, User};
use axum::extract::Json;
use axum::extract::State;
use axum::response::Response;
use jsonwebtoken::{encode, Header};
use surrealdb::opt::auth::Record;

pub async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>, // TODO adjust the error types to accomodate for 'not found disconnected blabla and other db errors'
) -> Result<Response, DbError> {
    tracing::info!("Attempting sign-in for user: {:?}", payload.username);

    let db = &state.db;

    let jwt = db
        .signin(Record {
            access: "user",
            namespace: &*NAMESPACE,
            database: &*DATABASE,
            params: Credentials {
                username: &payload.username,
                pass: &payload.pass,
            },
        })
        .await?;

    let token = jwt.as_insecure_token();

    tracing::info!("token: {:?}", token);

    let result: Vec<User> = db
        .query("SELECT * FROM user WHERE id = $auth.id")
        .await?
        .take(0)?;

    tracing::info!("result: {:?}", result);

    let user = result.into_iter().next();

    if let Some(user) = user {
        Ok(user.into_response(token.to_string()))
    } else {
        Err(DbError::Db)
    }
}

use argon2::{
    password_hash::{
        rand_core::OsRng, Error as Argon2Error, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};

use surrealdb::sql::Value;

trait FromQuery: Sized {
    fn from_query_result(result: Vec<Self>) -> Result<Self, DbError> {
        result.into_iter().next().ok_or(DbError::NotFound) // or whatever error type you prefer
    }
}

impl FromQuery for User {}

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
            created_at = time::now();
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

    dbg!("user", &created_user);

    let id = created_user
        .id
        .as_ref()
        .map(|record_id| record_id.to_string())
        .ok_or(AuthError::SignUpFail)?;

    use std::time::{SystemTime, UNIX_EPOCH};
    // In your signup function:
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + (60 * 60 * 24); // 24 hours from now

    let claims = Claims {
        name,
        username,
        email,
        role,
        id,
        exp,
    };

    dbg!("claims", &claims);

    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::RS256),
        &claims,
        &KEYS.encoding,
    )
    .map_err(|e| {
        eprintln!("Token creation error: {:?}", e); // Better error logging
        AuthError::TokenCreation
    })?;

    dbg!("token", &token);

    Ok(created_user.into_response(token))
}

fn hash_password(pass: &str) -> Result<String, PasswordHashError> {
    let pass_bytes = pass.as_bytes();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2.hash_password(&pass_bytes, &salt)?.to_string();
    let parsed_hash = PasswordHash::new(&hash)?;

    argon2
        .verify_password(pass_bytes, &parsed_hash)
        .map_err(|_| PasswordHashError::VerificationError)?;

    Ok(hash)
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordHashError {
    #[error("Failed to hash password: {0}")]
    HashingError(#[from] Argon2Error),
    #[error("Password verification failed after hashiong")]
    VerificationError,
}
