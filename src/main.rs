#[macro_use]
extern crate log;
extern crate argparse;
extern crate env_logger;
extern crate iron;
extern crate mount;
extern crate postgres;
extern crate router;
extern crate rustc_serialize;
extern crate staticfile;
extern crate time;

mod config;
mod handlers;
mod logger;
mod state;
mod worker;

use config::Configuration;
use handlers::DatabasesHandler;
use handlers::EmptyHandler;
use handlers::StatusHandler;
use state::State;
use worker::Worker;

use iron::Iron;
use mount::Mount;
use router::Router;
use staticfile::Static;


fn main() {
    if let Err(err) = logger::init() {
        panic!("Failed to initalize logger: {}", err);
    }

    info!("Reading configuration");

    let config = match Configuration::from_args() {
        Ok(config) => config,
        Err(err) => {
            error!("Failed to read configuration: {}", err);

            return;
        }
    };

    let state = State::default();

    info!("Starting worker thread");

    let worker = match Worker::spawn(config.clone(), state.clone()) {
        Ok(worker) => worker,
        Err(err) => {
            error!("Failed to spawn worker thread: {}", err);

            return;
        }
    };

    let mut router = Router::new();

    router.post("/status", StatusHandler::new(state.clone()), "status");
    router.post(
        "/databases",
        DatabasesHandler::new(state.clone()),
        "databases",
    );
    router.post("/", EmptyHandler::new(), "empty");

    let mut mount = Mount::new();
    mount.mount("/public", Static::new("public/"));
    mount.mount("/api/v1", router);
    mount.mount("/", Static::new("template/index.html"));

    info!("Binding to {}:{}", config.address(), config.port());

    if let Err(err) = Iron::new(mount).http((config.address().as_ref(), config.port())) {
        error!("Can not start server: {}", err);
    }

    worker.join();
}
