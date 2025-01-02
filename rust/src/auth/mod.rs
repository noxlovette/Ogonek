use jsonwebtoken::{ DecodingKey, EncodingKey };
use serde::{ Deserialize, Serialize };
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{ request::Parts, StatusCode },
    response::{ IntoResponse, Response },
    Json,
    RequestPartsExt,
};
use axum_extra::{ headers::{ authorization::Bearer, Authorization }, TypedHeader };
use serde_json::json;
use std::{ path::Display, sync::LazyLock };
use dotenvy::dotenv;
use thiserror::Error;
use axum::http::{ HeaderValue, header::{ self, HeaderMap } };
use surrealdb::sql::Thing;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(private_key: &[u8], public_key: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_rsa_pem(private_key).expect("Failed to create EncodingKey"),
            decoding: DecodingKey::from_rsa_pem(public_key).expect("Failed to create DecodingKey"),
        }
    }
}

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    dotenv().ok();
    let private_key = std::env::var("JWT_PRIVATE_KEY").expect("JWT_PRIVATE_KEY must be set");
    let public_key = std::env::var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY must be set");
    Keys::new(private_key.as_bytes(), public_key.as_bytes())
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub name: String,
    pub username: String,
    pub email: String,
    pub id: String,
    pub exp: i64,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub name: String,
    pub username: String,
    pub pass: String,
    pub email: String,
    pub id: String,
}

impl<'de> Deserialize<'de> for AuthBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        #[derive(Deserialize)]
        struct Helper {
            name: String,
            username: String,
            pass: String,
            email: String,
            id: Thing,
        }

        let helper = Helper::deserialize(deserializer)?;
        let id_string = format!("{}:{}", helper.id.tb, helper.id.id.to_string());

        Ok(AuthBody {
            name: helper.name,
            username: helper.username,
            pass: helper.pass,
            email: helper.email,
            id: id_string,
        })
    }
}

impl AuthBody {
    pub fn into_response(self, token: String) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).expect("Invalid Token Format")
        );

        (headers, Json(self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub email: String,
    pub pass: String,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
}

#[derive(Debug)]
pub struct Token(String);

impl Token {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Token where S: Send + Sync {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>().await
            .map_err(|_| AuthError::InvalidToken)?;

        let token = bearer.token().to_string();
        Ok(Token(token))
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
