use self::models::User;
use diesel::prelude::*;
use rust::*;
use std::env::args;
use uuid::Uuid;

fn main() {
    use self::schema::users::dsl::{users, email};

    let user_id = args()
        .nth(1)
        .expect("update_user_email requires a user id");
    
    let new_email = args()
        .nth(2)
        .expect("update_user_email requires an email address");

    let user_uuid = match Uuid::parse_str(&user_id) {
        Ok(uuid) => uuid,
        Err(e) => {
            eprintln!("Invalid UUID format: {}", e);
            return; // Exit the program if the UUID is invalid
        }
    };

    let connection = &mut establish_connection();

    // This update will set the email if it's null or update it if it exists
    let updated_user = diesel::update(users.find(user_uuid))
        .set(email.eq(new_email))
        .get_result::<User>(connection)
        .optional()
        .unwrap_or_else(|e| {
            eprintln!("Failed to update user email: {}", e);
            std::process::exit(1);
        });

    match updated_user {
        Some(user) => println!("User email updated to: {:?}", user.email),
        None => println!("User not found for id: {}", user_id),
    }
}