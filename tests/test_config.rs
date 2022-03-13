use config_lite::Config;

#[test]
fn file_content_is_saved_into_config_struct() {
    let config = Config::new();
    let actual_file_content = std::fs::read_to_string("./config/default.json").unwrap();
		assert_eq!(config.file_content, actual_file_content);
}
