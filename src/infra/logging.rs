use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(pretty_dev: bool) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info,sqlx::query=warn"));

    let registry = tracing_subscriber::registry().with(env_filter);

    if pretty_dev {
        // Dev: pretty
        registry
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_level(true)
                    .with_ansi(true)
                    .compact(),
            )
            .init();
    } else {
        // Prod: JSON
        registry
            .with(fmt::layer().with_target(true).with_level(true).json())
            .init();
    }
}
