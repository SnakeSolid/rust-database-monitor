use std::io::Result as IoResult;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::time::Duration;
use std::thread;
use std::thread::Builder;
use std::thread::JoinHandle;

use postgres::Connection;
use postgres::TlsMode;

use config::Configuration;
use config::ServerConnInfo;
use state::State;
use state::DatabaseState;

pub struct Worker {
    join_handle: JoinHandle<()>,
}

fn server_database_infos(conn_info: &ServerConnInfo) -> IoResult<Vec<DatabaseState>> {
    let url = format!(
        "postgresql://{2}:{3}@{0}:{1}/postgres",
        conn_info.host(),
        conn_info.port(),
        conn_info.role(),
        conn_info.password()
    );
    let conn = match Connection::connect(url, TlsMode::None) {
        Ok(conn) => conn,
        Err(err) => {
            warn!("Failed to connect to server {}: {}", conn_info.host(), err);

            return Err(IoError::new(ErrorKind::Other, err));
        }
    };

    let rows = conn.query(
        r#"
    SELECT
        d.datname,
        d.datcollate,
        r.rolname
    FROM pg_database AS d
        INNER JOIN pg_roles AS r ON ( r.oid = d.datdba )
    WHERE
        rolcreaterole = FALSE AND
        rolcanlogin = TRUE"#,
        &[],
    )?;

    let result = rows.into_iter()
        .map(|row| {
            let database_name: String = row.get(0);
            let collation_name: String = row.get(1);
            let owner: String = row.get(2);

            DatabaseState::new(database_name, collation_name, owner)
        })
        .collect();

    Ok(result)
}


fn do_work(config: Configuration, state: State) {
    loop {
        info!("Updating servers started");

        for conn_info in config.servers() {
            debug!("Updating server {}", conn_info.host());

            match server_database_infos(conn_info) {
                Ok(dbs) => state.update_server(&conn_info.host(), &conn_info.description(), dbs),
                Err(err) => warn!("Failed to update server {}: {}", conn_info.host(), err),
            }
        }

        info!("Updating servers finished");

        thread::sleep(Duration::from_secs(config.interval()));
    }
}


impl Worker {
    pub fn spawn(config: Configuration, state: State) -> IoResult<Worker> {
        let join_handle = Builder::new()
            .name("Worker (update db info)".into())
            .spawn(move || do_work(config, state))?;

        Ok(Worker { join_handle: join_handle })
    }

    pub fn join(self) {
        if let Err(_) = self.join_handle.join() {
            info!("Failed to join worker thread");
        }
    }
}
