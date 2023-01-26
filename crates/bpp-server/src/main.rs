extern crate core;

use crate::config::{Config, FromConfig};
use crate::error_def::BppConfigError;

mod config;
pub mod error_def;

struct NoteService {
    pub service_name: String,
    node: NoteDao,
}

impl FromConfig for NoteService {
    fn from_config(config: &Config) -> error_stack::Result<Self, BppConfigError> {
        Ok(NoteService {
            service_name: config.service.name.clone(),
            node: NoteDao::from_config(&config)?,
        })
    }
}

struct NoteDao {
    connection_str: String, // user@host.com:8080
    password: String,
}

impl FromConfig for NoteDao {
    fn from_config(config: &Config) -> error_stack::Result<Self, BppConfigError> {
        Ok(NoteDao {
            connection_str: config.database.connection_str.clone(),
            password: config.database.password_str.clone(),
        })
    }
}

fn main() {
    if let Ok(config) = Config::load() {
        // TODO: Remove unwrap and handle errors
        let service = NoteService::from_config(&config).unwrap();
        println!("This is the service: {:}", service.service_name)
    } else {
        panic!("Failed to start!")
    }
    // NoteService::build(&config);

    println!("Hello, world!");
}
