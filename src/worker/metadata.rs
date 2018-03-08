use std::collections::HashSet;
use std::io::Result as IoResult;
use std::thread::Builder;
use std::thread::JoinHandle;
use std::thread;
use std::time::Duration;

use postgres::Connection;
use postgres::TlsMode;

use config::MetadataConnInfo;
use state::State;

use super::WorkerResult;

pub struct MetadataWorker {
    join_handle: JoinHandle<()>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ServerDatabase {
    server_name: String,
    database_name: String,
}

impl ServerDatabase {
    fn new(server_name: &str, database_name: &str) -> ServerDatabase {
        ServerDatabase {
            server_name: server_name.into(),
            database_name: database_name.into(),
        }
    }

    fn server_name(&self) -> &String {
        &self.server_name
    }

    fn database_name(&self) -> &String {
        &self.database_name
    }
}

#[derive(Debug)]
struct DatabaseMatadata {
    server_name: String,
    database_name: String,
    commit: i64,
    branch_name: String,
    project_name: String,
}

impl DatabaseMatadata {
    fn new(
        server_name: &str,
        database_name: &str,
        commit: i64,
        branch_name: &str,
        project_name: &str,
    ) -> DatabaseMatadata {
        DatabaseMatadata {
            server_name: server_name.into(),
            database_name: database_name.into(),
            commit,
            branch_name: branch_name.into(),
            project_name: project_name.into(),
        }
    }

    fn server_name(&self) -> &String {
        &self.server_name
    }

    fn database_name(&self) -> &String {
        &self.database_name
    }

    fn commit(&self) -> i64 {
        self.commit
    }

    fn branch_name(&self) -> &String {
        &self.branch_name
    }

    fn project_name(&self) -> &String {
        &self.project_name
    }
}

fn query_database_metadata(
    connection_info: &MetadataConnInfo,
    databases: &Vec<ServerDatabase>,
) -> WorkerResult<Vec<DatabaseMatadata>> {
    let url = format!(
        "postgresql://{3}:{4}@{0}:{1}/{2}",
        connection_info.host(),
        connection_info.port(),
        connection_info.database(),
        connection_info.role(),
        connection_info.password()
    );
    let connection = Connection::connect(url, TlsMode::None)?;
    let statement = connection.prepare(connection_info.query())?;
    let mut result = Vec::default();

    for database in databases {
        let server_name = database.server_name();
        let database_name = database.database_name();

        for row in &statement.query(&[server_name, database_name])? {
            let commit: i64 = row.get(0);
            let branch_name: String = row.get(1);
            let project_name: String = row.get(2);

            result.push(DatabaseMatadata::new(
                server_name,
                database_name,
                commit,
                &branch_name,
                &project_name,
            ));
        }
    }

    Ok(result)
}

fn do_work(connection_info: &MetadataConnInfo, interval: Duration, state: State) {
    let mut ignored_databases: HashSet<ServerDatabase> = HashSet::default();

    loop {
        info!("Updating metadata started");

        let mut pending_databases = Vec::new();

        state.for_each_database(&mut |_, database| {
            if !database.commit().is_some() && !database.project_name().is_some()
                && !database.branch_name().is_some()
            {
                let server_database =
                    ServerDatabase::new(database.server_name(), database.database_name());

                if !ignored_databases.contains(&server_database) {
                    ignored_databases.insert(server_database.clone());
                    pending_databases.push(server_database);
                }
            }
        });

        match query_database_metadata(connection_info, &pending_databases) {
            Ok(databases) => for database in &databases {
                state.set_database_metadata(
                    database.server_name(),
                    database.database_name(),
                    database.commit(),
                    database.branch_name(),
                    database.project_name(),
                );
            },
            Err(err) => {
                warn!("Failed to update metadata: {}", err);
            }
        };

        info!("Updating metadata finished");

        thread::sleep(interval);
    }
}

impl MetadataWorker {
    pub fn spawn(
        connection_info: MetadataConnInfo,
        interval: u64,
        state: State,
    ) -> IoResult<MetadataWorker> {
        let interval = Duration::from_secs(interval);
        let join_handle = Builder::new()
            .name("Meta-data worker".into())
            .spawn(move || do_work(&connection_info, interval, state))?;

        Ok(MetadataWorker {
            join_handle: join_handle,
        })
    }

    pub fn join(self) {
        if let Err(_) = self.join_handle.join() {
            info!("Failed to join metadata worker thread");
        }
    }
}
