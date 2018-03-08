#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DatabaseRow {
    server_name: String,
    server_description: Option<String>,
    database_name: String,
    database_collate: String,
    database_owner: String,
    last_update: i64,
    weight: usize,
}

impl DatabaseRow {
    pub fn new(
        server_name: &str,
        server_description: &Option<String>,
        database_name: &str,
        database_collate: &str,
        database_owner: &str,
        last_update: i64,
        weight: usize,
    ) -> Self {
        DatabaseRow {
            server_name: server_name.into(),
            server_description: server_description.clone(),
            database_name: database_name.into(),
            database_collate: database_collate.into(),
            database_owner: database_owner.into(),
            last_update,
            weight,
        }
    }

    pub fn server_name(&self) -> &String {
        &self.server_name
    }

    pub fn server_description(&self) -> &Option<String> {
        &self.server_description
    }

    pub fn database_name(&self) -> &String {
        &self.database_name
    }

    pub fn database_collate(&self) -> &String {
        &self.database_collate
    }

    pub fn database_owner(&self) -> &String {
        &self.database_owner
    }

    pub fn last_update(&self) -> i64 {
        self.last_update
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}
