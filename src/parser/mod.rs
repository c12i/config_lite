use regex::Regex;

use crate::{ConfigError, ConfigResult};

pub mod json;
pub mod yaml;

pub(crate) fn get_value_from_env_var<'a, T: for<'de> serde::Deserialize<'de>>(
    value: &str,
) -> ConfigResult<T> {
    // not expected to fail
    let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
    match re.captures(value) {
        Some(c) => {
            let env_var_name = c
                .get(1)
                .ok_or_else(|| ConfigError::RegexError(value.to_string()))?
                .as_str();
            // handle error getting env vars
            let value = std::env::var(env_var_name)?;
            let value = serde_json::Value::String(value);
            Ok(serde_json::from_value(value)?)
        }
        None => {
					let value = serde_json::Value::String(value.to_string());
					Ok(serde_json::from_value(value)?)
				},
    }
}
