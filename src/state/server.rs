#[derive(Debug)]
pub struct ServerInfo {
    name: String,
    description: Option<String>,
    last_update: i64,
}

impl ServerInfo {
    pub fn new(name: &str, description: &Option<String>, last_update: i64) -> ServerInfo {
        ServerInfo {
            name: name.into(),
            description: description.clone(),
            last_update,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn last_update(&self) -> i64 {
        self.last_update
    }

    pub fn set_last_update(&mut self, last_update: i64) {
        self.last_update = last_update;
    }
}
