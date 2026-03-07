use ito_config::ConfigContext;
use ito_core::backend_auth::init_backend_auth;

#[test]
fn init_rejects_non_object_backend_server() {
    let home = tempfile::tempdir().unwrap();
    let config_dir = home.path().join(".config/ito");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("config.json"),
        r#"{"backendServer": "not-an-object"}"#,
    )
    .unwrap();

    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    let err = init_backend_auth(&ctx).unwrap_err();
    assert!(
        err.to_string()
            .contains("'backendServer' must be a JSON object"),
        "unexpected error: {err}"
    );
}
