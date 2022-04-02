pub fn parse_yaml<'a, T: for<'de> serde::Deserialize<'de>>(
    file_content: &str,
    string_path: &str,
) -> T {
    let value: serde_yaml::Value = serde_yaml::from_str(&file_content).unwrap();
    let string_path_vec = string_path.split(".").collect::<Vec<&str>>();
    let mut current_value = &value;
    string_path_vec.into_iter().for_each(|s| {
        current_value = current_value.get(s).unwrap();
    });
    serde_yaml::from_value(current_value.to_owned()).unwrap()
}
