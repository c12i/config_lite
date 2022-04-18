use std::env;

use crate::{ConfigError, ConfigResult};
use regex::Regex;

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
            .ok_or_else(|| ConfigError::ValueError(current_value.to_string()))?;
    }
    // check if current value is a string and if the string matches a {{}} regex pattern
    if current_value.is_string() {
        if let Some(value_string) = current_value.as_str() {
            // not expected to fail
            let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
            if let Some(c) = re.captures(value_string) {
                // return Err if therer are no matches
                let env_var_name = c
                    .get(1)
                    .ok_or_else(|| ConfigError::RegexError(value_string.to_string()))?
                    .as_str();
                // handle error getting env vars
                let value = env::var(env_var_name)?;
                let value = serde_json::Value::String(value);
                return Ok(serde_json::from_value(value)?);
            }
        }
    }
    Ok(serde_json::from_value(current_value.to_owned())?)
}
