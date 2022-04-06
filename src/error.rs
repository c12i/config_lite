use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("error parsing json file: {0}")]
    JsonParsingError(#[from] serde_json::Error),
    #[error("error parsing yaml file: {0}")]
    YamlParsingError(#[from] serde_yaml::Error),
    #[error("io error: {0}")]
    IoError(#[from] io::Error),
    #[error("no configuration file could be found")]
    FileNotFoundError,
    #[error("the file type in the configurations directory is not supported")]
    UnsupportedFileTypeError,
    #[error("no value could be found for the given key")]
    ValueError,
    #[error("error: {0}")]
    Other(#[from] anyhow::Error),
}
