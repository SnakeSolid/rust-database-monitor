use time;

use search::Document;

#[derive(Debug)]
pub struct DatabaseInfo {
    server_name: String,
    database_name: String,
    database_collate: String,
    database_owner: String,
    last_update: i64,
    commit: Option<usize>,
    project: Option<String>,
    branch: Option<String>,
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
            project: None,
            branch: None,
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

    pub fn commit(&self) -> Option<usize> {
        self.commit
    }

    pub fn project(&self) -> &Option<String> {
        &self.project
    }

    pub fn branch(&self) -> &Option<String> {
        &self.branch
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }
}
