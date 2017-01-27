use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::io::Result as IoResult;

use rustc_serialize::json;

use argparse::ArgumentParser;
use argparse::StoreOption;


#[derive(RustcDecodable, Debug, Clone)]
pub struct ServerConnInfo {
    pub name: String,
    pub role: String,
    pub password: String,
}

#[derive(RustcDecodable, Debug, Clone)]
pub struct Configuration {
    pub address: String,
    pub port: u16,
    pub interval: u64,
    pub servers: Vec<ServerConnInfo>,
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
            ap.refer(&mut address).add_option(&["-b", "--bind"],
                                              StoreOption,
                                              "Address to bind on (default: localhost)");
            ap.refer(&mut port).add_option(&["-p", "--port"],
                                           StoreOption,
                                           "Port to listen (default: 8080)");
            ap.refer(&mut interval).add_option(&["-i", "--interval"],
                                               StoreOption,
                                               "Probe interval in seconds (default: 600)");
            ap.refer(&mut config_file).add_option(&["-c", "--config"],
                                                  StoreOption,
                                                  "Path to configuration file");
            ap.parse_args_or_exit();
        }

        let mut config = match config_file {
            Some(config_file) => Self::read_from_file(config_file)?,
            None => Configuration::default(),
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
        where P: AsRef<Path>
    {
        let mut file = File::open(path)?;
        let mut raw = String::new();

        file.read_to_string(&mut raw)?;

        match json::decode(&raw) {
            Ok(config) => Ok(config),
            Err(err) => {
                error!("Failed to parse configuration file: {}", err);

                Err(IoError::new(ErrorKind::Other, err))
            }
        }
    }
}


impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            address: "localhost".into(),
            port: 8080,
            interval: 600,
            servers: Vec::new(),
        }
    }
}
