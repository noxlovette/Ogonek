// src/bin/generate_admin_hash.rs
use anyhow::{Context, Result};
use rpassword::read_password;
use std::io::{self, Write};

use ogonek::auth::password::hash_password;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔐 Admin Password Hash Generator");
    println!("This generates the hash for your admin creation password\n");

    print!("Enter your admin creation password: ");
    io::stdout().flush()?;
    let password = read_password().context("Failed to read password")?;

    if password.len() < 12 {
        eprintln!("⚠️  Warning: Consider using a longer password (12+ chars)");
    }

    print!("Confirm password: ");
    io::stdout().flush()?;
    let password_confirm = read_password().context("Failed to read password confirmation")?;

    if password != password_confirm {
        anyhow::bail!("❌ Passwords don't match");
    }

    println!("\n🔄 Generating hash...");
    let hash = hash_password(&password).context("Failed to hash password")?;

    println!("\n✅ Hash generated successfully!");
    println!("📋 Copy this hash to your create_god_user.rs file:");
    println!("const EXPECTED_SECRET_HASH: &str = \"{}\";", hash);

    println!("\n🔧 Also update the verify_admin_secret function to use proper Argon2 verification");

    Ok(())
}
