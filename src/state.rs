use std::sync::Arc;
use std::collections::HashMap;
use std::sync::RwLock;

use time;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ServerState {
    name: String,
    last_update: i64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DatabaseState {
    pub name: String,
    pub collate: String,
    pub owner: String,
    pub last_update: i64,
}

#[derive(Debug)]
struct InternalState {
    servers: Vec<ServerState>,
    databases: HashMap<String, Vec<DatabaseState>>,
    last_update: i64,
}

#[derive(Debug, Clone)]
pub struct State {
    inner: Arc<RwLock<InternalState>>,
}


impl DatabaseState {
    pub fn new<S1, S2, S3>(name: S1, collate: S2, owner: S3) -> DatabaseState
        where S1: Into<String>,
              S2: Into<String>,
              S3: Into<String>
    {
        let now = time::get_time().sec;

        DatabaseState {
            name: name.into(),
            collate: collate.into(),
            owner: owner.into(),
            last_update: now,
        }
    }
}


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DatabaseRow {
    pub server_name: String,
    pub database_name: String,
    pub database_collate: String,
    pub database_owner: String,
    pub last_update: i64,
}


impl DatabaseRow {
    fn new<S1, S2, S3, S4>(server_name: S1,
                           database_name: S2,
                           database_collate: S3,
                           database_owner: S4,
                           last_update: i64)
                           -> Self
        where S1: Into<String>,
              S2: Into<String>,
              S3: Into<String>,
              S4: Into<String>
    {
        DatabaseRow {
            server_name: server_name.into(),
            database_name: database_name.into(),
            database_collate: database_collate.into(),
            database_owner: database_owner.into(),
            last_update: last_update,
        }
    }
}


impl State {
    pub fn update_server<S>(&self, name: S, databases: Vec<DatabaseState>)
        where S: Into<String>
    {
        if let Ok(mut inner) = self.inner.write() {
            let name = name.into();
            let now = time::get_time().sec;
            let updated;

            inner.databases.insert(name.clone(), databases);

            if let Some(server_state) =
                inner.servers
                    .iter_mut()
                    .filter(|server_state| server_state.name == name)
                    .next() {
                server_state.last_update = now;
                updated = true;
            } else {
                updated = false;
            }

            if !updated {
                inner.servers.push(ServerState {
                    name: name,
                    last_update: now,
                })
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
                for database in databases {
                    let row = DatabaseRow::new(server_name.clone(),
                                               database.name.clone(),
                                               database.collate.clone(),
                                               database.owner.clone(),
                                               database.last_update);

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
            servers: Vec::new(),
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
