use std::env;

use tracing::metadata::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logger() {
    let use_ansi = env::var("HTTP_LOG_ANSI").unwrap_or("1".to_owned());
    let use_ansi = !(use_ansi.is_empty() || use_ansi == "0"); // i.e. HTTP_LOG_ANSI=0 turns it off

    let format = fmt::format()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(use_ansi)
        .compact();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .event_format(format)
        .init();
    tracing::debug!(%use_ansi, "Logger initialized");
}
