use std::sync::Arc;
use std::collections::HashMap;
use std::sync::RwLock;

use time;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ServerState {
    name: String,
    description: Option<String>,
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

impl ServerState {
    fn new(name: String, description: Option<String>, last_update: i64) -> ServerState {
        ServerState {
            name,
            description,
            last_update,
        }
    }
}

impl DatabaseState {
    pub fn new(name: String, collate: String, owner: String) -> DatabaseState {
        let now = time::get_time().sec;
        let lower_name = name.to_lowercase();

        DatabaseState {
            name: name,
            collate: collate,
            owner: owner,
            last_update: now,
            lower_name: lower_name,
        }
    }
}


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DatabaseRow {
    pub server_name: String,
    pub server_description: Option<String>,
    pub database_name: String,
    pub database_collate: String,
    pub database_owner: String,
    pub lower_name: String,
    pub last_update: i64,
}


impl DatabaseRow {
    fn new(
        server_name: String,
        server_description: Option<String>,
        database_name: String,
        database_collate: String,
        database_owner: String,
        lower_name: String,
        last_update: i64,
    ) -> Self {
        DatabaseRow {
            server_name,
            server_description,
            database_name,
            database_collate,
            database_owner,
            lower_name,
            last_update,
        }
    }
}


impl State {
    pub fn update_server(
        &self,
        name: &String,
        description: &Option<String>,
        databases: Vec<DatabaseState>,
    ) {
        if let Ok(mut inner) = self.inner.write() {
            let now = time::get_time().sec;
            let updated;

            inner.databases.insert(name.clone(), databases);

            if let Some(server_state) = inner.servers.get_mut(name) {
                server_state.last_update = now;
                updated = true;
            } else {
                updated = false;
            }

            if !updated {
                inner.servers.insert(
                    name.clone(),
                    ServerState::new(name.clone(), description.clone(), now),
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
