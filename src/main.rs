#![allow(unused_imports, unused_variables)]
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
use std::{
    future::ready,
    net::SocketAddr,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
extern crate env_logger;
#[macro_use] extern crate log;

mod metrics;

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
    Router::new()
        .route("/github", get(github_handler))
        .route("/health", get(health_handler))
        .route("/metrics", get(move || ready(recorder_handle.render())))
        .route_layer(middleware::from_fn(metrics::track_metrics))
}

async fn start_server() {
    let app = run_app();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    env_logger::init();
    setup_logging();
    let _app = tokio::select! {
        _ = start_server() => info!("Starting crabwalk...")
    };
}

fn setup_logging() {
    let builder = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env());
    let builder = builder
        .with_line_number(true)
        .with_target(false)
        .with_file(true)
        .init();
}
