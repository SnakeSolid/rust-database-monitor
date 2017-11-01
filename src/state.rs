use std::sync::Arc;
use std::collections::HashMap;
use std::sync::RwLock;

use time;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ServerState {
    name: String,
    description: String,
    last_update: i64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DatabaseState {
    pub name: String,
    pub collate: String,
    pub owner: String,
    pub last_update: i64,
    pub lower_name: String,
}

#[derive(Debug)]
struct InternalState {
    servers: HashMap<String, ServerState>,
    databases: HashMap<String, Vec<DatabaseState>>,
    last_update: i64,
}

#[derive(Debug, Clone)]
pub struct State {
    inner: Arc<RwLock<InternalState>>,
}


impl DatabaseState {
    pub fn new<S1, S2, S3>(name: S1, collate: S2, owner: S3) -> DatabaseState
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        let now = time::get_time().sec;
        let name = name.into();
        let lower_name = name.to_lowercase();

        DatabaseState {
            name: name,
            collate: collate.into(),
            owner: owner.into(),
            last_update: now,
            lower_name: lower_name,
        }
    }
}


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DatabaseRow {
    pub server_name: String,
    pub server_description: String,
    pub database_name: String,
    pub database_collate: String,
    pub database_owner: String,
    pub lower_name: String,
    pub last_update: i64,
}


impl DatabaseRow {
    fn new<S1, S2, S3, S4, S5, S6>(
        server_name: S1,
        server_description: S2,
        database_name: S3,
        database_collate: S4,
        database_owner: S5,
        lower_name: S6,
        last_update: i64,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
        S5: Into<String>,
        S6: Into<String>,
    {
        DatabaseRow {
            server_name: server_name.into(),
            server_description: server_description.into(),
            database_name: database_name.into(),
            database_collate: database_collate.into(),
            database_owner: database_owner.into(),
            lower_name: lower_name.into(),
            last_update: last_update,
        }
    }
}


impl State {
    pub fn update_server<S1, S2>(&self, name: S1, description: S2, databases: Vec<DatabaseState>)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        if let Ok(mut inner) = self.inner.write() {
            let name = name.into();
            let now = time::get_time().sec;
            let updated;

            inner.databases.insert(name.clone(), databases);

            if let Some(server_state) = inner.servers.get_mut(&name) {
                server_state.last_update = now;
                updated = true;
            } else {
                updated = false;
            }

            if !updated {
                inner.servers.insert(
                    name.clone(),
                    ServerState {
                        name: name,
                        description: description.into(),
                        last_update: now,
                    },
                );
            }

            inner.last_update = now;
        } else {
            warn!("Failed to lock state for write");
        }
    }

    pub fn databases(&self) -> Vec<DatabaseRow> {
        let mut result = Vec::new();

        if let Ok(inner) = self.inner.read() {
            for (server_name, databases) in &inner.databases {
                let server = match inner.servers.get(server_name) {
                    Some(server) => server,
                    None => {
                        warn!("Failed to lookup server with name {}", server_name);

                        continue;
                    }
                };

                for database in databases {
                    let row = DatabaseRow::new(
                        server.name.clone(),
                        server.description.clone(),
                        database.name.clone(),
                        database.collate.clone(),
                        database.owner.clone(),
                        database.lower_name.clone(),
                        database.last_update,
                    );

                    result.push(row);
                }
            }
        } else {
            warn!("Failed to lock state for read");
        }

        result
    }

    pub fn last_update(&self) -> Option<i64> {
        if let Ok(inner) = self.inner.read() {
            Some(inner.last_update)
        } else {
            warn!("Failed to lock state for read");

            None
        }
    }
}


impl Default for InternalState {
    fn default() -> Self {
        InternalState {
            servers: HashMap::new(),
            databases: HashMap::new(),
            last_update: 0,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State { inner: Arc::new(RwLock::new(InternalState::default())) }
    }
}
