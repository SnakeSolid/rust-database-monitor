#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DatabaseRow {
    server_name: String,
    server_description: Option<String>,
    database_name: String,
    database_collate: String,
    database_owner: String,
    commit: Option<i64>,
    branch_name: Option<String>,
    project_name: Option<String>,
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
        commit: &Option<i64>,
        branch_name: &Option<String>,
        project_name: &Option<String>,
        last_update: i64,
        weight: usize,
    ) -> Self {
        DatabaseRow {
            server_name: server_name.into(),
            server_description: server_description.clone(),
            database_name: database_name.into(),
            database_collate: database_collate.into(),
            database_owner: database_owner.into(),
            commit: commit.clone(),
            branch_name: branch_name.clone(),
            project_name: project_name.clone(),
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

    pub fn commit(&self) -> &Option<i64> {
        &self.commit
    }

    pub fn branch_name(&self) -> &Option<String> {
        &self.branch_name
    }

    pub fn project_name(&self) -> &Option<String> {
        &self.project_name
    }

    pub fn last_update(&self) -> i64 {
        self.last_update
    }

    pub fn weight(&self) -> usize {
        self.weight
    }
}
