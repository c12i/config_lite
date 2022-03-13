use config_lite::Config;

#[test]
fn it_works() {
    let cfg = Config::build();
    assert!(cfg.config_path.exists())
}
