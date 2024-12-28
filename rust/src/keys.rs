use crate::schemas::auth::Keys;
use dotenvy::dotenv;
use std::sync::LazyLock;

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});
