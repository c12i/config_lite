use crate::{ConfigError, ConfigResult};

use super::get_value_from_env_var;

pub(crate) fn parse_yaml<'a, T: for<'de> serde::Deserialize<'de>>(
    file_content: &str,
    string_path: &str,
) -> ConfigResult<T> {
    let value: serde_yaml::Value = serde_yaml::from_str(&file_content)?;
    let string_path_vec = string_path.split(".").collect::<Vec<&str>>();
    let mut current_value = &value;
    for s in string_path_vec {
        current_value = current_value.get(s).ok_or_else(|| {
            ConfigError::ValueError(current_value.as_str().unwrap_or("").to_string())
        })?;
    }
    if current_value.is_string() {
        if let Some(value_string) = current_value.as_str() {
            return get_value_from_env_var(value_string);
        }
    }
    Ok(serde_yaml::from_value(current_value.to_owned())?)
}
