use std::io::stdin;
use rust::db::postgres::establish_connection;
use rust::db::users::create_user;


fn main() {
    let connection = &mut establish_connection();

    let mut username = String::new();
    let mut password = String::new();
    let mut superuser = String::new();
    let mut superuser_bool = false;
    let mut second_password = String::new();

    println!("Username?");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end(); // Remove the trailing newline

    println!("\nYour password for {}\n", username);
    stdin().read_line(&mut password).unwrap();

    println!("\nPlease confirm your password for {}\n", username);
    stdin().read_line(&mut second_password).unwrap();

    if password.trim_end() != second_password.trim_end() {
        println!("\nPasswords do not match. Please try again.");
        return;
    }

    println!("\nAre you a superuser? (y/n)");
    stdin().read_line(&mut superuser).unwrap();
    if superuser.trim_end() == "y" {
        superuser_bool = true;
    }

    let user = create_user(connection, &username, &password, &superuser_bool);
    println!("\nUser Created {username} with id {}", user.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
