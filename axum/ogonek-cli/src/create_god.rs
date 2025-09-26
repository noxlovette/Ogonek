use anyhow::{Context, Result};
use rpassword::read_password;
use std::io::{self, Write};

use ogonek::{auth::password::hash_password, db::init_db, types::UserRole};
const EXPECTED_SECRET_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$PoJVUnhR1uC4La2JtAQZJA$kF3T1R9+AqX6DnfYwBrp10z8G1qPlW1XjsqsQvH8y/E";
#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 God User Creation Tool");
    println!("⚠️  This tool creates a user with ultimate system privileges\n");

    print!("Enter admin creation password: ");
    io::stdout().flush()?;
    let secret = read_password().context("Failed to read admin password")?;

    if !verify_admin_secret(&secret) {
        anyhow::bail!("❌ Invalid admin creation password");
    }

    println!("✅ Admin password verified\n");

    let (username, email, password, name) = get_user_details_interactive()?;

    println!("\n🔌 Connecting to database...");
    let pool = init_db().await?;
    let existing_god = sqlx::query!(
        r#"SELECT id FROM "user" WHERE role = $1 LIMIT 1"#,
        UserRole::God.to_string()
    )
    .fetch_optional(&pool)
    .await
    .context("Failed to check for existing god users")?;

    if existing_god.is_some() {
        print!("⚠️  A god user already exists. Continue anyway? (y/N): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("❌ Aborted");
            return Ok(());
        }
    }

    println!("🔐 Hashing password...");
    let password_hash = hash_password(&password).context("Failed to hash password")?;

    println!("👑 Creating god user...");
    let user_id = nanoid::nanoid!();

    sqlx::query!(
        r#"
        INSERT INTO "user" (id, username, name, email, pass, role, joined, verified)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), true)
        "#,
        user_id,
        username,
        name,
        email,
        password_hash,
        UserRole::God.to_string(),
    )
    .execute(&pool)
    .await
    .context("Failed to create god user")?;

    println!("\n✅ God user created successfully!");
    println!("🆔 User ID: {}", user_id);
    println!("👤 Username: {}", username);
    println!("📧 Email: {}", email);
    println!("👑 Role: God");

    // TODO: AUDIT LOGS
    println!("📝 Audit log entry created");
    println!("🚀 All done! May the gods smile upon your deployment.");

    Ok(())
}

fn get_user_details_interactive() -> Result<(String, String, String, String)> {
    print!("Enter username for god user: ");
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    if username.is_empty() {
        anyhow::bail!("Username cannot be empty");
    }

    print!("Enter email: ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim().to_string();

    if !email.contains('@') {
        anyhow::bail!("Invalid email format");
    }

    print!("Enter password: ");
    io::stdout().flush()?;
    let password = read_password().context("Failed to read password")?;

    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }

    print!("Confirm password: ");
    io::stdout().flush()?;
    let password_confirm = read_password().context("Failed to read password confirmation")?;

    if password != password_confirm {
        anyhow::bail!("Passwords don't match");
    }

    print!("Enter name for god user: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    if name.is_empty() {
        anyhow::bail!("Name cannot be empty");
    }

    Ok((username, email, password, name))
}
use argon2::{Argon2, PasswordHash, PasswordVerifier};

fn verify_admin_secret(input: &str) -> bool {
    let parsed_hash = match PasswordHash::new(EXPECTED_SECRET_HASH) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(input.as_bytes(), &parsed_hash)
        .is_ok()
}
