use crate::auth::claims::{Claims, RefreshClaims, KEYS, KEYS_REFRESH};

use crate::auth::error::AuthError;
use crate::models::users::{InviteToken, User};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use jsonwebtoken::{encode, Header};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
pub fn generate_token_with_duration(
    user: &User,
    duration_secs: usize,
    is_refresh: bool,
) -> Result<String, AuthError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let exp = now + duration_secs;

    if is_refresh {
        let claims = RefreshClaims {
            sub: user.id.clone().to_string(),
            exp,
        };

        encode(
            &Header::new(jsonwebtoken::Algorithm::RS256),
            &claims,
            &KEYS_REFRESH.encoding,
        )
        .map_err(|e| {
            eprintln!("Refresh token creation error: {:?}", e);
            AuthError::TokenCreation
        })
    } else {
        let claims = Claims {
            name: user.name.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            sub: user.id.clone().to_string(),
            exp,
            iat: now,
            nbf: Some(now),
            jti: Some(Uuid::new_v4().to_string()),
            iss: "auth:auth".to_string(),
        };

        encode(
            &Header::new(jsonwebtoken::Algorithm::RS256),
            &claims,
            &KEYS.encoding,
        )
        .map_err(|e| {
            eprintln!("Access token creation error: {:?}", e);
            AuthError::TokenCreation
        })
    }
}

pub async fn decode_invite_token(token: String) -> Result<String, AuthError> {
    let decoded = URL_SAFE
        .decode(token)
        .map_err(|_| AuthError::InvalidToken)?;

    let decoded_str = String::from_utf8(decoded).map_err(|_| AuthError::InvalidToken)?;

    let token: InviteToken =
        serde_json::from_str(&decoded_str).map_err(|_| AuthError::InvalidToken)?;

    Ok(token.teacher_id)
}

pub async fn encode_invite_token(id: String) -> Result<String, AuthError> {
    let token = InviteToken::new(id);
    let json = serde_json::to_string(&token).map_err(|_| AuthError::TokenCreation)?;

    let encoded = URL_SAFE.encode(json.as_bytes());
    Ok(encoded)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::users::User;
    use jsonwebtoken::{decode, Validation};

    fn create_test_user() -> User {
        User {
            id: "test-user-id".to_string(),
            name: "Test User".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            pass: "hashed_password".to_string(),
            role: "user".to_string(),
            verified: true,
        }
    }

    #[test]
    fn test_generate_token() {
        // Arrange
        let user = create_test_user();

        // Act
        let token = generate_token(&user).expect("Failed to generate token");

        // Assert
        assert!(!token.is_empty(), "Token should not be empty");

        // Verify token can be decoded correctly
        let validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        let token_data =
            decode::<Claims>(&token, &KEYS.decoding, &validation).expect("Failed to decode token");

        assert_eq!(token_data.claims.sub, user.id);
        assert_eq!(token_data.claims.name, user.name);
        assert_eq!(token_data.claims.username, user.username);
        assert_eq!(token_data.claims.role, user.role);
        assert_eq!(token_data.claims.email, user.email);
        assert_eq!(token_data.claims.iss, "auth:auth");
    }

    #[test]
    fn test_generate_refresh_token() {
        // Arrange
        let user = create_test_user();

        // Act
        let token = generate_refresh_token(&user).expect("Failed to generate refresh token");

        // Assert
        assert!(!token.is_empty(), "Refresh token should not be empty");

        // Verify token can be decoded correctly
        let validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        let token_data = decode::<RefreshClaims>(&token, &KEYS_REFRESH.decoding, &validation)
            .expect("Failed to decode refresh token");

        assert_eq!(token_data.claims.sub, user.id);
        assert!(token_data.claims.exp > 0, "Expiration time should be set");
    }

    #[test]
    fn test_token_expiration() {
        // This test requires mocking time, which is complex
        // For a simplified test, we can check that the token expiration is set in the future
        let user = create_test_user();
        let token = generate_token(&user).expect("Failed to generate token");

        let validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        let token_data =
            decode::<Claims>(&token, &KEYS.decoding, &validation).expect("Failed to decode token");

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        assert!(
            token_data.claims.exp > now,
            "Token should expire in the future"
        );
        assert!(
            token_data.claims.exp <= now + (60 * 15),
            "Token should expire within 15 minutes"
        );
    }
}
