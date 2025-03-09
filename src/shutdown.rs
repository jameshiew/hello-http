#[cfg(unix)]
pub async fn shutdown_handler() {
    let mut interrupts = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
        .expect("failed to register SIGINT handler");
    let mut terminates = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to register SIGTERM handler");
    let mut quits = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::quit())
        .expect("failed to register SIGQUIT handler");
    tokio::select! {
        _ = interrupts.recv() => {
            tracing::info!("received SIGINT, shutting down");
        }
        _ = terminates.recv() => {
            tracing::info!("received SIGTERM, shutting down");
        }
        _ = quits.recv() => {
            tracing::info!("received SIGQUIT, shutting down");
        }
    };
}

#[cfg(windows)]
pub async fn shutdown_handler() {
    if tokio::signal::ctrl_c().await.is_ok() {
        tracing::info!("received Ctrl+C, shutting down");
    } else {
        tracing::error!("failed to listen for Ctrl+C");
    }
}