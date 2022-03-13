use config_lite::Config;

#[test]
fn it_works() {
    let cfg = Config::new();
    let actual_file_content = std::fs::read_to_string("./config/default.json").unwrap();
		assert_eq!(cfg.file_content, actual_file_content);
}
