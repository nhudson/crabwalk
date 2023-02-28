use std::sync::Arc;
use tokio::signal;
use tracing::info;

pub async fn axum_shutdown() {
    let sigint = async {
        signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let term = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let term = std::future::pending::<()>();

    tokio::select! {
        _ = sigint => {
            info!("Received SIGINT, shutting down");
        }
        _ = term => {
            info!("Received SIGTERM, shutting down");
        }
    }
}
