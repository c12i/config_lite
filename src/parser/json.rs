use crate::{ConfigError, ConfigResult};

pub fn parse_json<'a, T: for<'de> serde::Deserialize<'de>>(
    file_content: &str,
    string_path: &str,
) -> ConfigResult<T> {
    let value: serde_json::Value = serde_json::from_str(&file_content)?;
    let string_path_vec = string_path.split(".").collect::<Vec<&str>>();
    let mut current_value = &value;
    for s in string_path_vec {
        current_value = current_value
            .get(s)
            .ok_or_else(|| ConfigError::ValueError)?;
    }
    Ok(serde_json::from_value(current_value.to_owned())?)
}
