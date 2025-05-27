use crate::auth::claims::{Claims, KEYS};

use crate::auth::error::AuthError;
use crate::models::users::InviteToken;
use crate::models::TokenWithExpiry;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use jsonwebtoken::Algorithm;
use jsonwebtoken::{encode, Header};
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
        eprintln!("Token generation failed: {:?}", e);
        AuthError::TokenCreation
    })?;

    Ok(TokenWithExpiry {
        token,
        expires_at: exp,
    })
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
    // requires GitHub Actions to have jwt keys

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
    }

    #[tokio::test]
    async fn test_encode_decode_invite_token() {
        let teacher_id = "teacher_123".to_string();

        let encoded = encode_invite_token(teacher_id.clone())
            .await
            .expect("Encoding failed");

        let decoded = decode_invite_token(encoded).await.expect("Decoding failed");

        assert_eq!(decoded, teacher_id);
    }

    #[tokio::test]
    async fn test_decode_invite_token_invalid_format() {
        let result = decode_invite_token("invalid_base64".to_string()).await;
        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[tokio::test]
    async fn test_encode_invite_token_failure() {
        // Not much can go wrong in the current implementation unless `serde_json::to_string` fails,
        // which is unlikely with valid data. You could artificially force an error by mocking if needed.
        let teacher_id = "teacher_123".to_string();
        let result = encode_invite_token(teacher_id).await;

        assert!(result.is_ok());
    }
}
