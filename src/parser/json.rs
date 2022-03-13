pub fn parse_json<'a, T: for<'de> serde::Deserialize<'de>>(
    file_content: &str,
    string_path: &str,
) -> T {
    let value: serde_json::Value = serde_json::from_str(&file_content).unwrap();
    let mut string_path_vec = string_path.split(".").collect::<Vec<&str>>();
    serde_json::from_value(value).unwrap()
}
