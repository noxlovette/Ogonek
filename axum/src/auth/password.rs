use crate::auth::error::PasswordHashError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        // Arrange
        let password = "secure_password123";

        // Act
        let hashed = hash_password(password).expect("Failed to hash password");
        let verify_result = verify_password(&hashed, password).expect("Failed verify call");
        let verify_wrong = verify_password(&hashed, "wrong_password").expect("Failed verify call");

        // Assert
        assert!(
            verify_result,
            "Password verification should succeed with correct password"
        );
        assert!(
            !verify_wrong,
            "Password verification should fail with incorrect password"
        );
    }

    #[test]
    fn test_password_hash_different_each_time() {
        // This tests that we're using a salt properly
        let password = "secure_password123";

        let hash1 = hash_password(password).expect("Failed to hash password");
        let hash2 = hash_password(password).expect("Failed to hash password");

        assert_ne!(
            hash1, hash2,
            "Password hashes should differ due to different salts"
        );
    }

    #[test]
    fn test_password_error_cases() {
        // Testing with empty password
        let result = hash_password("");
        assert!(
            result.is_ok(),
            "Empty passwords should be handled (though they shouldn't be allowed at higher levels)"
        );

        // Very long password (testing for potential overflow issues)
        let long_password = "a".repeat(10000);
        let result = hash_password(&long_password);
        assert!(result.is_ok(), "Long passwords should be handled properly");
    }
}
