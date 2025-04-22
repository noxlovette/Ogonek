use crate::auth::claims::{Claims, RefreshClaims, KEYS, KEYS_REFRESH};
use crate::auth::error::AuthError;
use crate::models::users::User;
use jsonwebtoken::{encode, Header};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn generate_token(user: &User) -> Result<String, AuthError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    // 15 minutes from now
    let exp = now + (60 * 15);

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
        // aud:"svelte:user:general".to_string(),
        iss: "auth:auth".to_string(),
    };

    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::RS256),
        &claims,
        &KEYS.encoding,
    )
    .map_err(|e| {
        eprintln!("Token creation error: {:?}", e);
        AuthError::TokenCreation
    })?;

    return Ok(token);
}

pub fn generate_refresh_token(user: &User) -> Result<String, AuthError> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + (60 * 60 * 24 * 30); // 30 days from now

    let claims = RefreshClaims {
        sub: user.id.clone().to_string(),
        exp,
    };

    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::RS256),
        &claims,
        &KEYS_REFRESH.encoding,
    )
    .map_err(|e| {
        eprintln!("Token creation error: {:?}", e);
        AuthError::TokenCreation
    })?;

    return Ok(token);
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
