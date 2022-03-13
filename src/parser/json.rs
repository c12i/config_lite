use std::path::PathBuf;

pub fn parse_json<'a, T: for<'de> serde::Deserialize<'de>>(
    file_path: &PathBuf,
    string_path: &str,
) -> T {
    let file_content = std::fs::read_to_string(file_path).unwrap();
    let value: serde_json::Value = serde_json::from_str(&file_content).unwrap();
    let mut string_path_vec = string_path.split(".").collect::<Vec<&str>>();
    serde_json::from_value(value).unwrap()
}
