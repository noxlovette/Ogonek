use sentry::ClientInitGuard;

pub fn sentry() -> ClientInitGuard {
    sentry::init((
        std::env::var("SENTRY_DSN").expect("SENTRY_DSN must be set"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: std::env::var("SENTRY_TRACE_RATE")
                .unwrap_or_else(|_| "1.0".to_string())
                .parse::<f32>()
                .unwrap_or(1.0),
            environment: Some(
                std::env::var("APP_ENV")
                    .expect("APP_ENV must be set")
                    .into(),
            ),
            send_default_pii: true,
            ..Default::default()
        },
    ))
}
