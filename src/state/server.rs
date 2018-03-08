#[derive(Debug)]
pub struct ServerInfo {
    name: String,
    description: Option<String>,
}

impl ServerInfo {
    pub fn new(name: &str, description: &Option<String>) -> ServerInfo {
        ServerInfo {
            name: name.into(),
            description: description.clone(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }
}
