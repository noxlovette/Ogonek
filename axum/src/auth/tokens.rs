use crate::auth::claims::{Claims, KEYS};

use crate::auth::error::AuthError;
use crate::models::users::InviteToken;
use crate::models::TokenWithExpiry;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use jsonwebtoken::Algorithm;
use jsonwebtoken::{encode, Header};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn generate_token(user_id: &str, secs: u64) -> Result<TokenWithExpiry, AuthError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let exp = now + secs;

    let claims = Claims {
        sub: user_id.to_string(),
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
