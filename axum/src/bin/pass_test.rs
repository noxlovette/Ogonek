use rust::auth::error::PasswordHashError;
use rust::auth::helpers::{hash_password, verify_password};

fn main() -> Result<(), PasswordHashError> {
    let password = "mypassword123";
    let hash = hash_password(password)?;

    // Should print true
    println!("Valid password: {}", verify_password(&hash, password)?);

    // Should print false
    println!(
        "Invalid password: {}",
        verify_password(&hash, "wrongpassword")?
    );

    Ok(())
}
