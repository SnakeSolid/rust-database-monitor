use std::collections::HashMap;

use time;

use search::Query;

use super::DatabaseInfo;
use super::ServerInfo;

#[derive(Debug)]
pub struct InternalState {
    servers: HashMap<String, ServerInfo>,
    databases: HashMap<String, Vec<DatabaseInfo>>,
    last_update: i64,
}

impl InternalState {
    pub fn update_server(&mut self, server_name: &str, server_description: &Option<String>) {
        let now = time::get_time().sec;
        let name = server_name.into();
        let server = self.servers
            .entry(name)
            .or_insert_with(|| ServerInfo::new(server_name, server_description, 0));

        server.set_last_update(now);

        self.last_update = now;
    }

    pub fn update_databases(&mut self, server_name: &str, mut databases: Vec<DatabaseInfo>) {
        let now = time::get_time().sec;
        let name = server_name.into();
        let entry = self.databases.entry(name).or_insert_with(|| Vec::default());

        entry.clear();
        entry.extend(databases.drain(..));

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

            for database_info in databases {
                let document = database_info.document();

                if let Some(weight) = document.weight_for(query) {
                    callback(server_info, database_info, weight);
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
