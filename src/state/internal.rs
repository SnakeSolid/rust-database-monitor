use std::collections::HashMap;
use std::collections::HashSet;

use time;

use search::Query;

use super::DatabaseInfo;
use super::ServerInfo;

#[derive(Debug)]
pub struct InternalState {
    servers: HashMap<String, ServerInfo>,
    databases: HashMap<String, HashMap<String, DatabaseInfo>>,
    last_update: i64,
}

impl InternalState {
    pub fn update_server(&mut self, server_name: &str, server_description: &Option<String>) {
        let now = time::get_time().sec;
        let name = server_name.into();

        self.servers
            .entry(name)
            .or_insert_with(|| ServerInfo::new(server_name, server_description));

        self.last_update = now;
    }

    pub fn update_databases(&mut self, server_name: &str, databases: Vec<DatabaseInfo>) {
        let now = time::get_time().sec;
        let name = server_name.into();
        let entry = self.databases
            .entry(name)
            .or_insert_with(|| HashMap::default());
        let mut keys: HashSet<_> = entry.keys().cloned().collect();

        for database in databases {
            let database_name = database.database_name().clone();

            keys.remove(&database_name);
            entry
                .entry(database_name)
                .or_insert(database)
                .set_last_update(now);
        }

        for key in keys {
            entry.remove(&key);
        }

        self.last_update = now;
    }

    pub fn query(&self, query: &Query, callback: &mut FnMut(&ServerInfo, &DatabaseInfo, usize)) {
        for (server_name, databases) in &self.databases {
            let server_info = match self.servers.get(server_name) {
                Some(server_info) => server_info,
                None => {
                    warn!("Failed to lookup server with name {}", server_name);

                    continue;
                }
            };

            for database_info in databases.values() {
                let document = database_info.document();

                if let Some(weight) = document.weight_for(query) {
                    callback(server_info, database_info, weight);
                }
            }
        }
    }

    pub fn for_each_database(&self, callback: &mut FnMut(&ServerInfo, &DatabaseInfo)) {
        for (server_name, databases) in &self.databases {
            let server_info = match self.servers.get(server_name) {
                Some(server_info) => server_info,
                None => {
                    warn!("Failed to lookup server with name {}", server_name);

                    continue;
                }
            };

            for database_info in databases.values() {
                callback(server_info, database_info);
            }
        }
    }

    pub fn set_database_metadata(
        &mut self,
        server_name: &str,
        database_name: &str,
        commit: i64,
        branch_name: &str,
        project_name: &str,
    ) {
        if let Some(databases) = self.databases.get_mut(server_name) {
            for database in databases.values_mut() {
                if database.database_name() == database_name {
                    database.set_commit(commit);
                    database.set_branch_name(branch_name);
                    database.set_project_name(project_name);

                    let commit = format!("{}", commit);

                    database
                        .document_mut()
                        .extend(&[&commit, branch_name, project_name]);

                    break;
                }
            }
        }
    }

    pub fn last_update(&self) -> i64 {
        self.last_update
    }
}

impl Default for InternalState {
    fn default() -> InternalState {
        InternalState {
            servers: HashMap::default(),
            databases: HashMap::default(),
            last_update: 0,
        }
    }
}
