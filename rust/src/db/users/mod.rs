// src/posts/mod.rs
use crate::models::{NewUser, User, UserUpdate};
use crate::schema::users;
use diesel::prelude::*;
use bcrypt::{hash, DEFAULT_COST};
use crate::db::postgres::establish_connection;
use diesel::result::Error as DieselError;

// CREATE

pub fn create_user(username: &str, password: &str, superuser: &bool) -> Result<User, DieselError> {
    let connection = &mut establish_connection();
    let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");
    let new_user = NewUser {
        username,
        password: &hashed_password,
        date_joined: chrono::Local::now().naive_local(),
        last_login: None,
        is_superuser: Some(*superuser),
        first_name: None,
        last_name: None,
        email: None,
        is_staff: None,
        is_active: None,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
}

// RETRIEVE

use bcrypt::verify;
use uuid::Uuid;

pub fn retrieve_user(usernamestring: &str, passwordstring: &str) -> Result<User, String> {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let user_result: User = users
        .filter(username.eq(usernamestring))
        .first(connection)
        .map_err(|e| format!("Failed to fetch user: {}", e))?;
    
    if verify(passwordstring, &user_result.password).map_err(|_| "Failed to verify password".to_string())? {
        println!("Password verification succeeded");
        Ok(user_result)
    } else {
        Err("Invalid password".to_string())
    }
}

// UPDATE

pub fn update_user(user_id: String, update_data: UserUpdate) -> Result<User, DieselError> {
    let connection = &mut establish_connection();
    
    // If you're updating the password, you should hash it before saving
    let update_data = if let Some(ref password) = update_data.password {
        let hashed_password = hash(password, DEFAULT_COST).expect("Failed to hash password");
        UserUpdate {
            password: Some(hashed_password),
            ..update_data
        }
    } else {
        update_data
    };

    // Perform the update operation
    let user_id = Uuid::parse_str(&user_id).expect("Invalid user ID");
    diesel::update(users::table.find(user_id))
        .set(&update_data)
        .get_result(connection)
}