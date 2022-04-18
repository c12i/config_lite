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
        let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        // handle Value to &str conversion error
        let value_string = current_value.as_str().unwrap();
        match re.captures(value_string) {
            Some(c) => {
                // return Err if therer are no matches
                let env_var_name = c.get(1).unwrap().as_str();
                // handle error getting env vars
                let value = env::var(env_var_name).unwrap();
                let value = serde_json::Value::String(value);
                return Ok(serde_json::from_value(value)?);
            }
            None => {
                /*TODO: return Err */
                unimplemented!()
            }
        }
    }
    Ok(serde_json::from_value(current_value.to_owned())?)
}
