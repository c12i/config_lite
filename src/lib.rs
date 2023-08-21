#![doc = include_str!("../README.md")]

use anyhow::anyhow;
use fancy_regex::Regex;
use std::convert::TryFrom;
use std::path::PathBuf;

mod error;
mod parser;
mod utils;

pub use error::ConfigError;
use parser::json::parse_json;
use parser::yaml::parse_yaml;
use utils::{get_config_path, get_current_configuration_environment};

type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) filetype: FileType,
    pub file_content: String,
}

#[derive(Debug, Clone)]
pub enum FileType {
    Json,
    Yaml,
}

impl TryFrom<PathBuf> for FileType {
    type Error = ConfigError;

    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        if let Some(s) = p.extension() {
            let s = s
                .to_str()
                .ok_or_else(|| ConfigError::Other(anyhow!("error converting OsStr to &str")))?;
            match s {
                "json" => Ok(FileType::Json),
                "yaml" | "yml" => Ok(FileType::Yaml),
                _ => Err(ConfigError::UnsupportedFileTypeError(p)),
            }
        } else {
            Err(ConfigError::FileNotFoundError)
        }
    }
}

impl Config {
    pub fn init() -> ConfigResult<Self> {
        let filename = get_current_configuration_environment();
        let config_path = get_config_path();
        let matching_entry = std::fs::read_dir(config_path)?
            .find(|entry| {
                let value = entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap_or("".into())
                    .split('.')
                    .next()
                    .unwrap_or("")
                    .to_owned();
                value == filename
            })
            .ok_or(ConfigError::FileNotFoundError)??;
        let config_file_path = matching_entry.path();
        let file_content = std::fs::read_to_string(&config_file_path)?;
        Ok(Config {
            filetype: FileType::try_from(config_file_path.to_owned())?,
            file_content,
        })
    }

    pub fn get<'a, T: for<'de> serde::Deserialize<'de>>(&self, s: &'a str) -> ConfigResult<T> {
        let re = Regex::new(r"^(?!.*\.{2,})(?!\S*\.$)\S+(\.\S+)*$").unwrap();
        if re.is_match(s).is_err() {
            return Err(ConfigError::InvalidStringPathError(s.to_string()));
        }
        match self.filetype {
            FileType::Json => parse_json(&self.file_content, s),
            FileType::Yaml => parse_yaml(&self.file_content, s),
        }
    }
}
