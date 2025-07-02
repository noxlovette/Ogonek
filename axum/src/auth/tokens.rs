use crate::auth::claims::{Claims, KEYS};

use crate::auth::error::AuthError;
use crate::models::TokenWithExpiry;
use crate::models::users::InviteToken;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};

use jsonwebtoken::{Algorithm, Header, Validation, decode, encode};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_token(
    user_id: &str,
    user_role: &str,
    secs: u64,
) -> Result<TokenWithExpiry, AuthError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let exp = now + secs;

    let claims = Claims {
        sub: user_id.to_string(),
        role: user_role.to_string(),
        exp: exp as usize,
        iat: now as usize,
    };

    let token = encode(&Header::new(Algorithm::RS256), &claims, &KEYS.encoding).map_err(|e| {
        eprintln!("Token generation failed: {e:?}");
        AuthError::TokenCreation
    })?;

    Ok(TokenWithExpiry {
        token,
        expires_at: exp,
    })
}

pub fn decode_token(token: &str) -> Result<Claims, AuthError> {
    let validation = Validation::new(Algorithm::RS256);

    let token_data = decode::<Claims>(token, &KEYS.decoding, &validation).map_err(|e| {
        eprintln!("Token extraction error: {e:?}");
        AuthError::InvalidToken
    })?;

    Ok(token_data.claims)
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
    #[test]
    fn test_generate_token_success() {
        let user_id = "test_user";
        let user_role = "admin";
        let duration = 3600;

        let result = generate_token(user_id, user_role, duration);

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(token.token.len() > 0);
        assert!(token.expires_at > 0);

        // Verify the token expires in the future
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert!(token.expires_at > now);
        assert!(token.expires_at <= now + duration);
    }

    #[test]
    fn test_generate_token_different_roles() {
        let test_cases = vec![
            ("user123", "admin", 3600),
            ("user456", "teacher", 1800),
            ("user789", "student", 900),
        ];

        for (user_id, role, duration) in test_cases {
            let result = generate_token(user_id, role, duration);
            assert!(result.is_ok(), "Failed for role: {}", role);

            let token = result.unwrap();
            assert!(token.token.len() > 0);
            assert!(token.expires_at > 0);
        }
    }

    #[test]
    fn test_decode_token_success() {
        let user_id = "test_user";
        let user_role = "admin";
        let duration = 3600;

        // Generate a token first
        let token_result = generate_token(user_id, user_role, duration);
        assert!(token_result.is_ok());
        let token = token_result.unwrap();

        // Now decode it
        let decode_result = decode_token(&token.token);
        assert!(decode_result.is_ok());

        let claims = decode_result.unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.role, user_role);
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_decode_token_invalid_format() {
        let invalid_tokens = vec![
            "invalid.token.format",
            "not_a_jwt_at_all",
            "",
            "header.payload", // Missing signature
            "too.many.parts.here.extra",
        ];

        for invalid_token in invalid_tokens {
            let result = decode_token(invalid_token);
            assert!(result.is_err(), "Should fail for token: {}", invalid_token);
            assert!(matches!(result.unwrap_err(), AuthError::InvalidToken));
        }
    }

    #[test]
    fn test_decode_token_malformed_jwt() {
        // Create a JWT-like string but with invalid base64
        let malformed_jwt =
            "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.invalid_payload.invalid_signature";

        let result = decode_token(malformed_jwt);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AuthError::InvalidToken));
    }

    #[test]
    fn test_generate_decode_roundtrip() {
        let test_cases = vec![
            ("user1", "admin", 3600),
            ("user_with_underscores", "teacher", 1800),
            ("123456", "student", 900),
            ("special@user.com", "moderator", 7200),
        ];

        for (user_id, role, duration) in test_cases {
            // Generate token
            let token_result = generate_token(user_id, role, duration);
            assert!(
                token_result.is_ok(),
                "Generation failed for user: {}",
                user_id
            );
            let token = token_result.unwrap();

            // Decode token
            let decode_result = decode_token(&token.token);
            assert!(
                decode_result.is_ok(),
                "Decoding failed for user: {}",
                user_id
            );
            let claims = decode_result.unwrap();

            // Verify claims
            assert_eq!(claims.sub, user_id);
            assert_eq!(claims.role, role);
            assert!(claims.exp > claims.iat);

            // Verify timing
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize;
            assert!(claims.iat <= now + 5); // Allow 5 second buffer for test execution
            assert!(claims.exp <= now + duration as usize + 5);
        }
    }

    #[test]
    fn test_token_expiry_calculation() {
        let user_id = "test_user";
        let user_role = "admin";
        let duration = 1800; // 30 minutes

        let before_generation = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let result = generate_token(user_id, user_role, duration);
        assert!(result.is_ok());

        let after_generation = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let token = result.unwrap();

        // Verify expiry is within expected range
        assert!(token.expires_at >= before_generation + duration);
        assert!(token.expires_at <= after_generation + duration);
    }

    #[tokio::test]
    async fn test_encode_decode_invite_token() {
        let teacher_id = "teacher_123".to_string();

        let encoded = encode_invite_token(teacher_id.clone())
            .await
            .expect("Encoding failed");

        assert!(!encoded.is_empty());

        let decoded = decode_invite_token(encoded).await.expect("Decoding failed");

        assert_eq!(decoded, teacher_id);
    }

    #[tokio::test]
    async fn test_encode_decode_invite_token_edge_cases() {
        let test_cases = vec![
            "".to_string(),
            "a".to_string(),
            "very_long_teacher_id_with_special_characters_123456789".to_string(),
            "teacher@example.com".to_string(),
            "teacher-with-dashes".to_string(),
            "teacher_with_underscores".to_string(),
            "123456789".to_string(),
        ];

        for teacher_id in test_cases {
            let encoded = encode_invite_token(teacher_id.clone())
                .await
                .expect(&format!("Encoding failed for: {teacher_id}"));

            let decoded = decode_invite_token(encoded)
                .await
                .expect(&format!("Decoding failed for: {teacher_id}"));

            assert_eq!(decoded, teacher_id);
        }
    }

    #[tokio::test]
    async fn test_decode_invite_token_invalid_cases() {
        let invalid_tokens = vec![
            "invalid_base64!@#".to_string(),
            "not_json_at_all".to_string(),
            "".to_string(),
            "🚀🎉".to_string(), // Unicode characters
        ];

        for invalid_token in invalid_tokens {
            let result = decode_invite_token(invalid_token.clone()).await;
            assert!(result.is_err(), "Should fail for token: {invalid_token}");
            assert!(matches!(result.unwrap_err(), AuthError::InvalidToken));
        }
    }

    #[tokio::test]
    async fn test_invite_token_contains_valid_json() {
        let teacher_id = "teacher_456".to_string();

        let encoded = encode_invite_token(teacher_id.clone())
            .await
            .expect("Encoding failed");

        // Manually decode to verify JSON structure
        let decoded_bytes = base64::engine::general_purpose::URL_SAFE
            .decode(&encoded)
            .expect("Base64 decode failed");

        let decoded_str = String::from_utf8(decoded_bytes).expect("UTF-8 conversion failed");

        // Should be valid JSON
        let json_value: serde_json::Value =
            serde_json::from_str(&decoded_str).expect("Invalid JSON");

        // Should contain teacher_id field
        assert!(json_value.get("teacher_id").is_some());
    }

    #[test]
    fn test_token_uniqueness() {
        let user_id = "test_user";
        let user_role = "admin";
        let duration = 3600;

        // Generate multiple tokens and verify they're unique
        let mut tokens = std::collections::HashSet::new();

        for _ in 0..5 {
            let result = generate_token(user_id, user_role, duration);
            assert!(result.is_ok());
            let token = result.unwrap();

            // Each token should be unique (due to different iat)
            assert!(tokens.insert(token.token), "Token should be unique");

            // Small delay to ensure different iat
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
