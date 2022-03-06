use serde::Deserialize;

#[derive(Debug)]
pub struct Config {
    // TODO: replace `String` with `&str`
    filename: String,
    config_directory: ConfigDirecrtory,
    file_type: FileType 
}

#[derive(Debug)]
pub struct ConfigDirecrtory {
    path: String,
    name: String
}

// TODO: Derive the `Default` trait for the `Config` and `ConfigDirectory` struct:
//       Default `Config`: filename = base/ default; config_path = process.cwd; config_dir = config

// TODO: Alternatively, allow the user to build the config struct via builder pattern
//       Provide a `ConfigBuilder` struct to facilitate this

#[derive(Debug)]
pub enum FileType {
    Json,
    Yaml
}

impl FileType {
    pub fn parse<'a, T: Deserialize<'a>>(s: &str) -> T {
        todo!()
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

    pub fn get<'a, T: Deserialize<'a>>(s: String) -> T {
        // it looks like parsin should be calling `self.file_type.parse(s)`

        // On parsing, check custom formatted string (structure TBD, but this is just an example)
        // matches `${{ENV_VAR_NAME}}`, and instead of passing this as the value, read from the
        // environmental variable name provided
        todo!()
    }

    fn read_from_env_var<'a, T: Deserialize<'a>>() -> T  {
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
