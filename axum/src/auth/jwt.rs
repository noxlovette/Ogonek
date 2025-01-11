use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization, Cookie},
    TypedHeader,
};

use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Validation};

use crate::auth::error::AuthError;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    dotenv().ok();
    let private_key = std::env::var("JWT_PRIVATE_KEY").expect("JWT PRIVATE KEY NOT SET");
    let public_key = std::env::var("JWT_PUBLIC_KEY").expect("JWT PUBLIC KEY NOT SET");
    Keys::new(private_key.as_bytes(), public_key.as_bytes())
});

pub static KEYS_REFRESH: LazyLock<Keys> = LazyLock::new(|| {
    dotenv().ok();
    let private_key =
        std::env::var("JWT_REFRESH_PRIVATE_KEY").expect("JWT REFRESH PRIVATE KEY NOT SET");
    let public_key =
        std::env::var("JWT_REFRESH_PUBLIC_KEY").expect("JWT REFRESH PUBLIC KEY NOT SET");
    Keys::new(private_key.as_bytes(), public_key.as_bytes())
});

impl<S> FromRequestParts<S> for RefreshClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<TypedHeader<Cookie>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        let refresh_token = cookies
            .get("refreshToken")
            .ok_or(AuthError::InvalidToken)?
            .to_string();

        let validation = Validation::new(Algorithm::RS256);
        let token_data =
            decode::<RefreshClaims>(&refresh_token, &KEYS_REFRESH.decoding, &validation).map_err(
                |e| {
                    eprintln!("Token extraction error: {:?}", e);
                    AuthError::InvalidToken
                },
            )?;

        Ok(token_data.claims)
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| {
                eprintln!("Token extraction error: {:?}", e);
                AuthError::InvalidToken
            })?;
        let validation = Validation::new(Algorithm::RS256);

        let token_data =
            decode::<Claims>(bearer.token(), &KEYS.decoding, &validation).map_err(|e| {
                eprintln!("Token extraction error: {:?}", e);
                AuthError::InvalidToken
            })?;
        Ok(token_data.claims)
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
    pub sub: String,
    pub name: String,
    pub username: String,
    pub role: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,      // Issued at timestamp
    pub nbf: Option<usize>, // Optional: Not valid before timestamp
    pub jti: Option<String>, // Optional: Unique token identifier
    // pub aud: String,     // Audience
    pub iss: String,     // Issuer
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
}
