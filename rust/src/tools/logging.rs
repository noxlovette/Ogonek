// initiate logging

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn init_logging() {
    use std::fs::OpenOptions;
    use tracing_subscriber::fmt::writer::MakeWriterExt;

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("logs/debug.log")
        .expect("Failed to open log file");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(file.with_max_level(tracing::Level::DEBUG)),
        )
        .init();
}
