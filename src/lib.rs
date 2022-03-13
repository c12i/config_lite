use std::convert::TryFrom;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug)]
pub struct Config {
    pub config_path: PathBuf,
    filetype: FileType,
}

// TODO: Derive the `Default` trait for the `Config` struct:
//       Default `Config`: filename = base/ default; config_path = process.cwd; config_dir = config

// TODO: Alternatively, allow the user to build the config struct via builder pattern
//       Provide a `ConfigBuilder` struct to facilitate this

#[derive(Debug)]
pub enum FileType {
    Json,
    Yaml,
}

impl FileType {
    #[allow(unused)]
    pub fn parse<'a, T: Deserialize<'a>>(s: &str) -> T {
        todo!()
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
    pub fn new() -> Config {
        /*
            Initialize `Config` by the value on the `CONFIG_LITE_ENV` variable
            This variable should store the name of the configurations file
            If the variable name is not set, default to something like `base` or `default`

            The supported file formats for the first iteration will be json and yaml.

            The configuration file extension would be programatically determined by looking
            for a file in the `CONFIG_LITE_DIR` that matches the `CONFIG_LITE_ENV` name.

            The type of file will determine which parsing logic to use.

            Edge case: in the case where there are multiple files of the same `CONFIG_LITE_ENV` name,
            there should be a criteria that determines which file to prioritize reading from.

            By default, the configurations directory is in the current dir the process is running
            in. There should be an option to however change this behaviour if needed, i.e
            - setting the path as an arg while building `Config`
            - reading from an environmental vaiable `CONFIG_LITE_DIR`
        */
        todo!()
    }

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
        let file = res.iter().next().unwrap();
        Config {
            filetype: FileType::try_from(file.to_owned()).unwrap(),
            config_path: file.to_owned(),
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

    #[allow(unused)]
    pub fn get<'a, T: Deserialize<'a>>(s: String) -> T {
        // it looks like parsin should be calling `self.file_type.parse(s)`

        // On parsing, check custom formatted string (structure TBD, but this is just an example)
        // matches `${{ENV_VAR_NAME}}`, and instead of passing this as the value, read from the
        // environmental variable name provided
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
