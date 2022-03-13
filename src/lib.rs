use std::convert::TryFrom;
use std::path::PathBuf;

use serde::Deserialize;

mod parser;

pub use parser::json::parse_json;
pub use parser::yaml::parse_yaml;

#[derive(Debug)]
pub struct Config {
    pub config_file_path: PathBuf,
    pub(crate) filetype: FileType,
}

#[derive(Debug)]
pub enum FileType {
    Json,
    Yaml,
}

impl FileType {
    pub fn parse<'a, T: Deserialize<'a>>(self, s: &str, file_path: &PathBuf) -> T {
        // TODO: Add regex to validate string path `s`
        let file_content = std::fs::read_to_string(file_path).unwrap();
        match self {
            FileType::Json => parse_json(file_content, s),
            FileType::Yaml => parse_yaml(file_content, s)
        }
    }
}

impl TryFrom<PathBuf> for FileType {
    // TODO: Replace with custom error type
    type Error = String;

    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        if let Some(s) = p.extension() {
            // TODO: Handle error
            let s = s.to_str().unwrap();
            match s {
                "json" => Ok(FileType::Json),
                "yaml" | "yml" => Ok(FileType::Yaml),
                _ => Err("Unsupported FileType".to_string()),
            }
        } else {
            Err("Not file detected".to_string())
        }
    }
}

impl Config {
    pub fn build() -> Self {
        let filename = Config::get_current_configuration_environment();
        let config_path = Config::get_config_path();
        let res = std::fs::read_dir(config_path)
            .unwrap()
            .filter(|d| {
                d.as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap()
                    .split(".")
                    .next()
                    .unwrap()
                    == filename
            })
            .map(|v| v.unwrap().path())
            .collect::<Vec<PathBuf>>();
        println!("{:#?}", res);
        let config_file_path = res.iter().next().unwrap();
        Config {
            filetype: FileType::try_from(config_file_path.to_owned()).unwrap(),
            config_file_path: config_file_path.to_owned(),
        }
    }

    fn get_config_path() -> PathBuf {
        let path = {
            if let Ok(value) = std::env::var("CONFIG_LITE_DIR_PATH") {
                let mut path_buf = PathBuf::new();
                path_buf.push(value);
                path_buf.push(Config::get_config_directory_name());
                return path_buf;
            }
            let mut config_path = std::env::current_dir().unwrap();
            config_path.push(Config::get_config_directory_name());
            config_path
        };
        path
    }

    fn get_current_configuration_environment() -> String {
        match std::env::var("CONFIG_LITE_ENV") {
            Ok(var) => var,
            Err(_) => "default".to_string(),
        }
    }

    fn get_config_directory_name() -> String {
        match std::env::var("CONFIG_LITE_DIRECTORY_NAME") {
            Ok(var) => var,
            Err(_) => "config".to_string(),
        }
    }

    pub fn get<'a, T: Deserialize<'a>>(s: String) -> T {
        let config_instance = Config::build();
        config_instance
            .filetype
            .parse(&s, &config_instance.config_file_path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
