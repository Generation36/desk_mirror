use serde::Deserialize;
use std::{fs, path::Path, fmt};
use crate::location::Location;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub location: Location,
    pub weather_api: ApiConfig,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub url: String,
    pub current_values: String,
    pub daily_values: String,
    pub units: String,
    pub forecast_days: u8,
}

#[derive(Debug)]
pub enum ConfigError {
    ReadError {
        path: String,
        source: std::io::Error,
    },
    ParseError(toml::de::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::ReadError { path, source } => {
                write!(f, "Failed to read config file at '{}': {}", path, source)
            }
            ConfigError::ParseError(e) => write!(f, "Failed to parse TOML: {}", e),
        }
    }
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let file_content = fs::read_to_string(&path)
                                    .map_err(|e| ConfigError::ReadError { 
                                        path: path.as_ref().display().to_string(), 
                                        source: e,
                                    })?;

    let config = toml::from_str(&file_content).map_err(ConfigError::ParseError)?;

    Ok(config)
}