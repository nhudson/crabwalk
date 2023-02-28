#![allow(unused_imports, unused_variables)]
use std::{
    future::ready,
    net::SocketAddr,
};
use axum::{
    body::Body,
    http::Response,
    middleware::{
        self, 
        Next,
    },
    routing::get,
    Router
};
use tokio::fs::metadata;
use tower_http::trace::{
        TraceLayer, 
        DefaultOnResponse,
        DefaultMakeSpan,
    };
use tracing::{
    error,
    info,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
    filter,
};

mod metrics;
mod utils;

async fn github_handler() -> Response<Body> {
    Response::builder()
        .status(204)
        .body(Body::empty())
        .unwrap()
}

async fn health_handler() -> Response<Body> {
    Response::builder()
        .status(204)
        .body(Body::empty())
        .unwrap()
}

fn run_app() -> Router {
    let recorder_handle = metrics::setup_metrics_recorder();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));
    Router::new()
        .route("/github_webhook", get(github_handler))
        .route("/healthz", get(health_handler))
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .route_layer(middleware::from_fn(metrics::track_metrics))
        .layer(trace_layer)
}

async fn start_server() {
    let app = run_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(utils::axum_shutdown())
        .await
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());
    tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().pretty()
                .with_filter(env_filter)
        )
        .init();
    tokio::select! {
        _ = start_server() => info!("Starting crabwalk...")
    };

    Ok(())
}
