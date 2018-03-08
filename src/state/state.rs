use std::sync::Arc;
use std::sync::RwLock;

use search::Query;

use super::DatabaseInfo;
use super::DatabaseRow;
use super::InternalState;
use super::ServerInfo;

#[derive(Debug, Clone)]
pub struct State {
    inner: Arc<RwLock<InternalState>>,
}

impl State {
    pub fn update_server(
        &self,
        server_name: &str,
        server_description: &Option<String>,
        databases: Vec<DatabaseInfo>,
    ) {
        if let Ok(mut inner) = self.inner.write() {
            inner.update_server(server_name, server_description);
            inner.update_databases(server_name, databases);
        } else {
            warn!("Failed to lock state for write");
        }
    }

    pub fn query(&self, query: &Query) -> Vec<DatabaseRow> {
        let mut result = Vec::new();

        if let Ok(inner) = self.inner.read() {
            inner.query(query, &mut |server, database, weight| {
                let row = DatabaseRow::new(
                    server.name(),
                    server.description(),
                    database.database_name(),
                    database.database_collate(),
                    database.database_owner(),
                    database.last_update(),
                    weight,
                );

                result.push(row);
            });
        } else {
            warn!("Failed to lock state for read");
        }

        result
    }

    pub fn for_each_database(&self, callback: &mut FnMut(&ServerInfo, &DatabaseInfo)) {
        if let Ok(inner) = self.inner.read() {
            inner.for_each_database(callback);
        } else {
            warn!("Failed to lock state for read");
        }
    }

    pub fn set_database_metadata(
        &self,
        server_name: &str,
        database_name: &str,
        commit: i64,
        branch_name: &str,
        project_name: &str,
    ) {
        if let Ok(mut inner) = self.inner.write() {
            inner.set_database_metadata(
                server_name,
                database_name,
                commit,
                branch_name,
                project_name,
            );
        } else {
            warn!("Failed to lock state for write");
        }
    }

    pub fn last_update(&self) -> Option<i64> {
        if let Ok(inner) = self.inner.read() {
            Some(inner.last_update())
        } else {
            warn!("Failed to lock state for read");

            None
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            inner: Arc::new(RwLock::new(InternalState::default())),
        }
    }
}
