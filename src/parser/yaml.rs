pub fn parse_yaml<'a, T: for<'de> serde::Deserialize<'de>>(
    file_content: &str,
    string_path: &str,
) -> T {
    let value: serde_yaml::Value = serde_yaml::from_str(&file_content).unwrap();
    let _string_path_vec = string_path.split(".").collect::<Vec<&str>>();
    serde_yaml::from_value(value).unwrap()
}
