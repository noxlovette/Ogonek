use crate::auth::error::AuthError;
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use dotenvy::dotenv;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    dotenv().ok();
    let private_key = std::env::var("JWT_PRIVATE_KEY").expect("JWT PRIVATE KEY NOT SET");
    let public_key = std::env::var("JWT_PUBLIC_KEY").expect("JWT PUBLIC KEY NOT SET");
    Keys::new(private_key.as_bytes(), public_key.as_bytes())
});

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
                eprintln!("Token extraction error: {e:?}");
                AuthError::InvalidToken
            })?;
        let validation = Validation::new(Algorithm::RS256);

        let token_data =
            decode::<Claims>(bearer.token(), &KEYS.decoding, &validation).map_err(|e| {
                eprintln!("Token extraction error: {e:?}");
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
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}
