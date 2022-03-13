use config_lite::Config;

#[test]
fn file_content_is_saved_into_config_struct() {
    let config = Config::new();
    let actual_file_content = std::fs::read_to_string("./config/default.json").unwrap();
    assert_eq!(config.file_content, actual_file_content);
}

#[test]
fn get_value_from_config_file() {
    let config = Config::new();
    let val = config.get::<String>("foo");
    assert_eq!(val, "bar".to_string());
}
