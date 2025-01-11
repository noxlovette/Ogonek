use crate::auth::error::{AuthError, PasswordHashError};
use crate::auth::jwt::{Claims, RefreshClaims};
use crate::auth::jwt::{KEYS, KEYS_REFRESH};
use crate::models::users::User;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, Header};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_token(user: &User) -> Result<String, AuthError> {
    // In your signup function:
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
        jti:Some(Uuid::new_v4().to_string()),
        // aud:"svelte:user:general".to_string(),
        iss:"auth:auth".to_string()
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

pub fn hash_password(pass: &str) -> Result<String, PasswordHashError> {
    let pass_bytes = pass.as_bytes();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2.hash_password(&pass_bytes, &salt)?.to_string();
    let parsed_hash = PasswordHash::new(&hash)?;

    argon2
        .verify_password(pass_bytes, &parsed_hash)
        .map_err(|_| PasswordHashError::VerificationError)?;

    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, PasswordHashError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
