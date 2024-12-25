// src/api/mod.rs
use bcrypt::verify;
use crate::models::users::User;
use diesel::prelude::*;
use crate::db::postgres::*;

pub fn fetch_user(usernamestring: &str, passwordstring: &str) -> Result<User, String> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    // Query the database for the user by username
    let user_result: User = users
        .filter(username.eq(usernamestring))
        .first(connection)
        .map_err(|e| format!("Failed to fetch user: {}", e))?;
    
    // Verify the password
    // Here, 'password' is the plain text password, and user_result.password is the bcrypt hash stored in the database
    if verify(passwordstring, &user_result.password).map_err(|_| "Failed to verify password".to_string())? {
        println!("Password verification succeeded");
        Ok(user_result)
    } else {
        Err("Invalid password".to_string())
    }
}