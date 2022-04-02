use config_lite::Config;
use serde::Deserialize;

#[test]
fn yaml_file_content_is_saved_into_config_struct() {
    let config = Config::new();
    let actual_file_content = std::fs::read_to_string("./config/default.yaml").unwrap();
    assert_eq!(config.file_content, actual_file_content);
}

#[derive(Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    screen_name: String,
    #[serde(rename(deserialize = "isActive"))]
    is_active: bool,
}

#[test]
fn get_value_from_yaml_config_file() {
    let config = Config::new();
    let val = config.get::<String>("foo");
    assert_eq!(val, "bar".to_string());

    let config_user = config.get::<User>("user");
    let actual_user = User {
        id: 1,
        name: "Foo Baz".to_string(),
        screen_name: "foo_baz".to_string(),
        is_active: true,
    };
    println!("{:?}", config_user);
    assert_eq!(1,1);
    assert_eq!(config_user.id, actual_user.id);
    assert_eq!(config_user.name, actual_user.name);
    assert_eq!(config_user.screen_name, actual_user.screen_name);
    assert_eq!(config_user.is_active, actual_user.is_active);
}
