// src/bin/login.rs
use rust::api::fetch_user;
use rust::auth::generate_jwt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "noxlovette";
    let password = "N15071997"; // This should come from user input

    match fetch_user(username, password) {
        Ok(user) => {
            let token = generate_jwt(user.id, user.is_superuser, &user.username)?;
            println!("Login successful. JWT Token: {}", token);
        },
        Err(e) => {
            eprintln!("Login failed: {}", e);
        }
    }

    Ok(())
}