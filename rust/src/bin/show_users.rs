use models::User;
use diesel::prelude::*;
use rust::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(is_superuser.eq(true))
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.username);
        println!("{}", user.email.unwrap_or("No email".to_string()));
    }
}