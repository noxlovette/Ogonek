use ogonek::app::{sentry, server};

fn main() {
    dotenvy::dotenv().ok();

    let _guard = sentry();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime")
        .block_on(async {
            if let Err(e) = server().await {
                tracing::error!("Server error: {}", e);
                std::process::exit(1);
            }
        });
}
