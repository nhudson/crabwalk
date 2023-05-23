use actix_web::{dev::ServerHandle, middleware, web, App, HttpServer};
use log::{debug, info};
use parking_lot::Mutex;

mod config;
mod github;
mod health;
use crate::{config::Config, github::github as gh, health::liveness, health::readiness};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::default();
    debug!("Config: {:?}", config);
    info!(
        "Starting HTTP server at http://{}:{}/",
        config.server_host, config.server_port
    );

    let stop_handle = web::Data::new(StopHandle::default());

    let server = HttpServer::new({
        let stop_handle = stop_handle.clone();
        move || {
            {
                App::new()
                    .app_data(stop_handle.clone())
                    .wrap(middleware::Logger::default().exclude("/health"))
            }
            .service(liveness)
            .service(readiness)
            .service(gh)
        }
    })
    .bind((config.server_host.clone(), config.server_port))?
    .shutdown_timeout(5)
    .run();

    stop_handle.register(server.handle());

    server.await
}

#[derive(Default)]
struct StopHandle {
    inner: Mutex<Option<ServerHandle>>,
}

impl StopHandle {
    // Set the ServerHandle to stop
    pub(crate) fn register(&self, handle: ServerHandle) {
        *self.inner.lock() = Some(handle);
    }

    //pub(crate) fn stop(&self, graceful: bool) {
    //    #[allow(clippy::let_underscore_future)]
    //    let _ = self.inner.lock().as_ref().unwrap().stop(graceful);
    //}
}
