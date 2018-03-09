#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate argparse;
extern crate env_logger;
extern crate iron;
extern crate mount;
extern crate postgres;
extern crate router;
extern crate serde_json;
extern crate staticfile;
extern crate time;

mod config;
mod handlers;
mod logger;
mod search;
mod state;
mod worker;

use config::Configuration;
use handlers::DatabasesHandler;
use handlers::EmptyHandler;
use handlers::StatusHandler;
use state::State;
use worker::DatabaseWorker;
use worker::MetadataWorker;

use iron::Iron;
use mount::Mount;
use router::Router;
use staticfile::Static;

fn start_database_worker(config: &Configuration, state: State) -> Option<DatabaseWorker> {
    info!("Starting database worker thread");

    let servers = config.servers().clone();
    let interval = config.interval();

    match DatabaseWorker::spawn(servers, interval, state) {
        Ok(worker) => Some(worker),
        Err(err) => {
            error!("Failed to spawn database worker thread: {}", err);

            None
        }
    }
}

fn start_metadata_worker(config: &Configuration, state: State) -> Option<MetadataWorker> {
    let interval = config.interval();
    let metadata_config = match config.metadata() {
        &Some(ref metadata_config) => metadata_config.clone(),
        &None => return None,
    };

    info!("Starting metadata worker thread");

    match MetadataWorker::spawn(metadata_config, interval, state) {
        Ok(worker) => Some(worker),
        Err(err) => {
            error!("Failed to spawn metadata worker thread: {}", err);

            None
        }
    }
}

fn initialize_server(state: State) -> Mount {
    let mut router = Router::new();
    let mut mount = Mount::new();

    router.post("/status", StatusHandler::new(state.clone()), "status");
    router.post(
        "/databases",
        DatabasesHandler::new(state.clone()),
        "databases",
    );
    router.post("/", EmptyHandler::new(), "empty");

    mount.mount("/public", Static::new("public/"));
    mount.mount("/api/v1", router);
    mount.mount("/", Static::new("template/index.html"));

    mount
}

fn main() {
    logger::init();

    info!("Reading configuration");

    let config = match Configuration::from_args() {
        Ok(config) => config,
        Err(err) => {
            error!("Failed to read configuration: {}", err);

            return;
        }
    };

    let state = State::default();
    let database_worker = start_database_worker(&config, state.clone());
    let metadata_worker = start_metadata_worker(&config, state.clone());
    let mount = initialize_server(state);

    info!("Binding to {}:{}", config.address(), config.port());

    if let Err(err) = Iron::new(mount).http((config.address().as_ref(), config.port())) {
        error!("Can not start server: {}", err);
    }

    if let Some(database_worker) = database_worker {
        database_worker.join();
    }

    if let Some(metadata_worker) = metadata_worker {
        metadata_worker.join();
    }
}
