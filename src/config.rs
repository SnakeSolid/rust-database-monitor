use std::fs::File;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result as IoResult;
use std::path::Path;

use serde_json;

use argparse::ArgumentParser;
use argparse::StoreOption;

#[derive(Deserialize, Debug, Clone)]
pub struct ServerConnInfo {
    host: String,
    port: Option<u16>,
    description: Option<String>,
    role: String,
    password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MetadataConnInfo {
    host: String,
    port: Option<u16>,
    database: String,
    role: String,
    password: String,
    query: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    address: String,
    port: u16,
    interval: u64,
    metadata: Option<MetadataConnInfo>,
    servers: Vec<ServerConnInfo>,
}

const DEFAULT_PORT: u16 = 5432;

impl ServerConnInfo {
    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_PORT)
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn role(&self) -> &String {
        &self.role
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

impl Configuration {
    pub fn from_args() -> IoResult<Configuration> {
        let mut address: Option<String> = None;
        let mut port: Option<u16> = None;
        let mut interval: Option<u64> = None;
        let mut config_file: Option<String> = None;

        {
            let mut ap = ArgumentParser::new();

            ap.set_description("PostgreSQL database monitor.");
            ap.refer(&mut address).add_option(
                &["-b", "--bind"],
                StoreOption,
                "Address to bind on (default: localhost)",
            );
            ap.refer(&mut port).add_option(
                &["-p", "--port"],
                StoreOption,
                "Port to listen (default: 8080)",
            );
            ap.refer(&mut interval).add_option(
                &["-i", "--interval"],
                StoreOption,
                "Probe interval in seconds (default: 600)",
            );
            ap.refer(&mut config_file).add_option(
                &["-c", "--config"],
                StoreOption,
                "Path to configuration file",
            );
            ap.parse_args_or_exit();
        }

        let mut config = match config_file {
            Some(config_file) => Self::read_from_file(config_file)?,
            None => {
                error!("Configuration file path is required");

                return Err(IoError::new(
                    ErrorKind::NotFound,
                    "Configuration file not found",
                ));
            }
        };

        if let Some(address) = address {
            config.address = address;
        }

        if let Some(port) = port {
            config.port = port;
        }

        if let Some(interval) = interval {
            config.interval = interval;
        }

        Ok(config)
    }

    fn read_from_file<P>(path: P) -> IoResult<Configuration>
    where
        P: AsRef<Path>,
    {
        let mut file = File::open(path)?;
        let mut raw = String::new();

        file.read_to_string(&mut raw)?;

        match serde_json::from_str(&raw) {
            Ok(config) => Ok(config),
            Err(err) => {
                error!("Failed to parse configuration file: {}", err);

                Err(IoError::new(ErrorKind::Other, err))
            }
        }
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn metadata(&self) -> &Option<MetadataConnInfo> {
        &self.metadata
    }

    pub fn servers(&self) -> &Vec<ServerConnInfo> {
        &self.servers
    }
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            address: "localhost".into(),
            port: 8080,
            interval: 600,
            metadata: None,
            servers: Vec::new(),
        }
    }
}
