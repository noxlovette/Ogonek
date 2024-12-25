use self::models::User;
use rust::*;
use uuid::Uuid;
use diesel::prelude::*;
use std::env::args;

fn main() {
    use self::schema::users::dsl::users;

    let user_id = args()
        .nth(1)
        .expect("get_user requires a user id");

    // Parse the user_id string into a UUID
    let user_uuid = match Uuid::parse_str(&user_id) {
        Ok(uuid) => uuid,
        Err(e) => {
            eprintln!("Invalid UUID format: {}", e);
            return; // Exit the program if the UUID is invalid
        }
    };

    let connection = &mut establish_connection();

    let user = users
        .find(user_uuid)
        .select(User::as_select())
        .first(connection)
        .optional();

    match user {
        Ok(Some(user)) => println!("User with id: {} has a username: {}", user.id, user.username),
        Ok(None) => println!("Unable to find user {}", user_id),
        Err(e) => println!("An error occurred while fetching user {}: {}", user_id, e),
    }
}