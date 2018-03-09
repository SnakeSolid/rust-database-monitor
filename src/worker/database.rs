use std::io::Result as IoResult;
use std::thread::Builder;
use std::thread::JoinHandle;
use std::thread;
use std::time::Duration;

use postgres::Connection;
use postgres::TlsMode;

use config::ServerConnInfo;
use state::DatabaseInfo;
use state::State;

use super::WorkerResult;

pub struct DatabaseWorker {
    join_handle: JoinHandle<()>,
}

fn server_database_infos(connection_info: &ServerConnInfo) -> WorkerResult<Vec<DatabaseInfo>> {
    let url = format!(
        "postgresql://{2}:{3}@{0}:{1}/postgres",
        connection_info.host(),
        connection_info.port(),
        connection_info.role(),
        connection_info.password()
    );
    let conn = Connection::connect(url, TlsMode::None)?;
    let rows = conn.query(include_str!("query-databases.sql"), &[])?;

    let result = rows.into_iter()
        .map(|row| {
            let database_name: String = row.get(0);
            let collation_name: String = row.get(1);
            let owner: String = row.get(2);

            DatabaseInfo::new(
                connection_info.host(),
                &database_name,
                &collation_name,
                &owner,
            )
        })
        .collect();

    Ok(result)
}

fn do_work(servers: Vec<ServerConnInfo>, interval: Duration, state: State) {
    loop {
        info!("Updating servers started");

        for connection_info in &servers {
            debug!("Updating server {}", connection_info.host());

            match server_database_infos(connection_info) {
                Ok(dbs) => state.update_server(
                    &connection_info.host(),
                    &connection_info.description(),
                    dbs,
                ),
                Err(err) => warn!(
                    "Failed to update server {}: {}",
                    connection_info.host(),
                    err
                ),
            }
        }

        info!("Updating servers finished");

        thread::sleep(interval);
    }
}

impl DatabaseWorker {
    pub fn spawn(
        servers: Vec<ServerConnInfo>,
        interval: u64,
        state: State,
    ) -> IoResult<DatabaseWorker> {
        let interval = Duration::from_secs(interval);
        let join_handle = Builder::new()
            .name("Database worker".into())
            .spawn(move || do_work(servers, interval, state))?;

        Ok(DatabaseWorker {
            join_handle: join_handle,
        })
    }

    pub fn join(self) {
        if let Err(_) = self.join_handle.join() {
            info!("Failed to join database worker thread");
        }
    }
}
