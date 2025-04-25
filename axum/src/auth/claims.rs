use crate::auth::error::AuthError;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization, Cookie},
    TypedHeader,
};
use dotenvy::dotenv;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Validation};
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

// the main implementation that actually validates the JWT-refresh
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

// same thing but for the access token
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
    pub iat: usize,          // Issued at timestamp
    pub nbf: Option<usize>,  // Optional: Not valid before timestamp
    pub jti: Option<String>, // Optional: Unique token identifier
    // pub aud: String,     // Audience
    pub iss: String, // Issuer
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::tokens::generate_token;
    use axum::extract::FromRequestParts;
    use axum::http::{header, HeaderMap, HeaderValue, Request};

    // Helper to create test auth headers
    fn create_auth_header(token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", token);
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&auth_value).unwrap(),
        );
        headers
    }

    #[tokio::test]
    async fn test_claims_extraction() {
        // Arrange
        let user = crate::models::users::User {
            id: "test-user-id".to_string(),
            name: "Test User".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            pass: "hashed_password".to_string(),
            role: "user".to_string(),
            verified: true,
        };

        let token = generate_token(&user).expect("Failed to generate token");
        let _headers = create_auth_header(&token);

        let req = Request::builder()
            .uri("https://example.com/test")
            .header(header::AUTHORIZATION, format!("Bearer {}", token))
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();

        // Act
        let claims = Claims::from_request_parts(&mut parts, &())
            .await
            .expect("Failed to extract claims");

        // Assert
        assert_eq!(claims.sub, user.id);
        assert_eq!(claims.name, user.name);
        assert_eq!(claims.username, user.username);
        assert_eq!(claims.role, user.role);
        assert_eq!(claims.email, user.email);
    }

    #[tokio::test]
    async fn test_invalid_token_extraction() {
        // Arrange
        let invalid_token = "invalid.jwt.token";
        let _headers = create_auth_header(invalid_token);

        let req = Request::builder()
            .uri("https://example.com/test")
            .header(header::AUTHORIZATION, format!("Bearer {}", invalid_token))
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();

        // Act
        let result = Claims::from_request_parts(&mut parts, &()).await;

        // Assert
        assert!(result.is_err(), "Should fail with invalid token");

        match result {
            Err(err) => {
                // Check that it's the right kind of error
                assert!(
                    matches!(err, AuthError::InvalidToken),
                    "Should be InvalidToken error"
                );
            }
            _ => panic!("Expected an error but got success"),
        }
    }

    #[tokio::test]
    async fn test_missing_auth_header() {
        // Arrange
        let req = Request::builder()
            .uri("https://example.com/test")
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();

        // Act
        let result = Claims::from_request_parts(&mut parts, &()).await;

        // Assert
        assert!(result.is_err(), "Should fail with missing auth header");
    }
}
