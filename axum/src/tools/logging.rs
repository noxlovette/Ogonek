use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn init_logging() -> anyhow::Result<()> {
    if tracing::dispatcher::has_been_set() {
        return Ok(());
    }

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level = if cfg!(debug_assertions) {
            "debug"
        } else {
            "info"
        };
        format!(
            "{}={},tower_http=debug,axum::rejection=trace,sqlx=info,hyper=info",
            env!("CARGO_CRATE_NAME"),
            level
        )
        .into()
    });

    let registry = tracing_subscriber::registry().with(env_filter);

    // Use JSON formatting in production, pretty formatting in development
    if std::env::var("LOG_FORMAT").as_deref() == Ok("json") {
        registry
            .with(fmt::layer().json().with_current_span(false))
            .with(sentry::integrations::tracing::layer())
            .try_init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize JSON logging: {}", e))?;
    } else {
        registry
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_ansi(atty::is(atty::Stream::Stdout))
                    .compact(),
            )
            .with(sentry::integrations::tracing::layer())
            .try_init()
            .map_err(|e| anyhow::anyhow!("Failed to initialize pretty logging: {}", e))?;
    }

    tracing::info!("Logging initialized successfully");
    Ok(())
}
