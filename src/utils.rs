use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    let config_directory_name = get_config_directory_name();
	// TODO: Find out why setting this env var doens't work
	if let Ok(value) = std::env::var("CONFIG_LITE_DIR_PATH") {
		let mut path_buf = PathBuf::new();
		path_buf.push(value);
		path_buf.push(config_directory_name);
		return path_buf;
	}
	let mut config_path = std::env::current_dir().unwrap();
	config_path.push(config_directory_name);
	config_path
}

pub fn get_current_configuration_environment() -> String {
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
