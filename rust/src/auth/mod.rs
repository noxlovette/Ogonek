use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde_json::json;
use surrealdb::RecordId;
use thiserror::Error;

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::api::auth::PasswordHashError;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Failed to Sign Up")]
    SignUpFail,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

impl From<surrealdb::Error> for AuthError {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::SignUpFail
    }
}

impl From<PasswordHashError> for AuthError {
    fn from(error: PasswordHashError) -> Self {
        eprintln!("{error}");
        Self::WrongCredentials
    }
}

#[derive(Debug)]
pub struct Token(String);

impl Token {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    dotenv().ok();
    let private_key = std::env::var("JWT_PRIVATE_KEY").expect("JWT PRIVATE KEY NOT SET");
    let public_key = std::env::var("JWT_PUBLIC_KEY").expect("JWT PUBLIC KEY NOT SET");
    Keys::new(private_key.as_bytes(), public_key.as_bytes())
});

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::SignUpFail => (StatusCode::BAD_REQUEST, "Failed to sign up"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(private_key: &[u8], public_key: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_rsa_pem(private_key).expect("Invalid Private Key"),
            decoding: DecodingKey::from_rsa_pem(public_key).expect("Invalid Public Key"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub name: String,
    pub username: String,
    pub role: String,
    pub email: String,
    pub exp: usize,
}
