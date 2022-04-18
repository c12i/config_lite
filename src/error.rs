use std::env::VarError;
use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("error parsing json file: {0}")]
    JsonParsingError(#[from] serde_json::Error),
    #[error("error parsing yaml file: {0}")]
    YamlParsingError(#[from] serde_yaml::Error),
    #[error("io error: {0}")]
    IoError(#[from] io::Error),
    #[error("Configuration file could be found")]
    FileNotFoundError,
    #[error("The file type `{0}` in the configurations directory is not supported")]
    UnsupportedFileTypeError(PathBuf),
    #[error("No value could be found for key `{0}`")]
    ValueError(String),
    #[error("Error reading configuration from env var: {0}")]
    EnvVarError(#[from] VarError),
    #[error("No matches invalid environmental variable interpolation: {0}")]
    RegexError(String),
    #[error("error: {0}")]
    Other(#[from] anyhow::Error),
}

impl From<PathBuf> for ConfigError {
    fn from(p: PathBuf) -> Self {
        ConfigError::UnsupportedFileTypeError(p)
    }
}