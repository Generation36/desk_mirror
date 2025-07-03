use crate::location::Location;
use serde::Deserialize;
use std::{collections::HashMap, fs, io, path::Path};
use thiserror::Error;
use toml::Value;

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
    pub forecast_days: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file")]
    ReadError(#[from] io::Error),

    #[error("Failed to parse TOML: {0}")]
    ParseError(#[from] toml::de::Error),

    #[error("Missing values found ")]
    MissingValueError,
}

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<HashMap<String, Value>, ConfigError> {
    let content = fs::read_to_string(path)?;
    let value: Value = toml::from_str(&content)?;

    let table = match value {
        Value::Table(table) => table,
        _ => return Err(ConfigError::MissingValueError),
    };

    Ok(table.into_iter().collect())
}
