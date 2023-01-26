use crate::error_def::BppConfigError;
use config::{Config as ConfigBase, Environment, File};
use error_stack::{report, Result};
use serde::Deserialize;
use std::env;

/// Enables building a Struct from config attributes
pub trait FromConfig: Sized {
    fn from_config(config: &Config) -> Result<Self, BppConfigError>;
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub service: ServiceConfig,
}

static DEFAULT_CONFIG_FILE: &str = "~/bpp_server.config";

impl Config {
    /// Load config options from a yaml file and overload
    ///  options with environment variable starting with `BPP_`
    ///  Example:
    ///     BPP_logging_level -> logging.level
    ///
    /// If BPP_SERVER_CONFIG is defined, it will be used
    /// instead of the default config path
    pub fn load() -> Result<Self, BppConfigError> {
        let config_file = env::var("BPP_SERVER_CONFIG").unwrap_or(DEFAULT_CONFIG_FILE.to_string());

        let config = ConfigBase::builder()
            .add_source(File::with_name(config_file.as_str()).required(true))
            .add_source(Environment::with_prefix("BPP").separator("_"))
            .build();

        config
            .map(|c| Config::deserialize(c).unwrap())
            .map_err(|err| {
                report!(err).change_context(BppConfigError::FailedToLoadConfigOptions(
                    "Config could not be loaded".to_string(),
                ))
            })
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DatabaseConfig {
    pub connection_str: String,
    pub password_str: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            connection_str: "user@url.ca:8080".to_string(),
            password_str: "123olivera4".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    pub level: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            level: "INFO".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ServiceConfig {
    pub name: String,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        ServiceConfig {
            name: "NoteService".to_string(),
        }
    }
}
