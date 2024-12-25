// src/posts/mod.rs
use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use bcrypt::{hash, DEFAULT_COST};
use crate::db::postgres::establish_connection;
use diesel::result::Error as DieselError;

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
