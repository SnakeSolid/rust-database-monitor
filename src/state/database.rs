use time;

use search::Document;

#[derive(Debug)]
pub struct DatabaseInfo {
    server_name: String,
    database_name: String,
    database_collate: String,
    database_owner: String,
    last_update: i64,
    commit: Option<i64>,
    project_name: Option<String>,
    branch_name: Option<String>,
    document: Document,
}

impl DatabaseInfo {
    pub fn new(
        server_name: &str,
        database_name: &str,
        database_collate: &str,
        database_owner: &str,
    ) -> DatabaseInfo {
        let now = time::get_time().sec;
        let document = Document::new(&[server_name, database_name]);

        DatabaseInfo {
            server_name: server_name.into(),
            database_name: database_name.into(),
            database_collate: database_collate.into(),
            database_owner: database_owner.into(),
            last_update: now,
            commit: None,
            project_name: None,
            branch_name: None,
            document: document,
        }
    }

    pub fn server_name(&self) -> &String {
        &self.server_name
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

    pub fn commit(&self) -> Option<i64> {
        self.commit
    }

    pub fn project_name(&self) -> &Option<String> {
        &self.project_name
    }

    pub fn branch_name(&self) -> &Option<String> {
        &self.branch_name
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn set_commit(&mut self, commit: i64) {
        self.commit = Some(commit);
    }

    pub fn set_project_name(&mut self, project_name: &str) {
        self.project_name = Some(project_name.into());
    }

    pub fn set_branch_name(&mut self, branch_name: &str) {
        self.branch_name = Some(branch_name.into());
    }

    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }
}
