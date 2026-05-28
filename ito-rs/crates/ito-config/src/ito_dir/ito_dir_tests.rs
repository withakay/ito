use super::*;

#[test]
fn get_ito_dir_name_defaults_to_dot_ito() {
    let td = tempfile::tempdir().unwrap();
    let ctx = ConfigContext::default();
    assert_eq!(get_ito_dir_name(td.path(), &ctx), ".ito");
}

#[test]
fn repo_config_overrides_global_config() {
    let td = tempfile::tempdir().unwrap();
    std::fs::write(
        td.path().join("ito.json"),
        "{\"projectPath\":\".repo-ito\"}",
    )
    .unwrap();

    let home = tempfile::tempdir().unwrap();
    let cfg_dir = home.path().join(".config/ito");
    std::fs::create_dir_all(&cfg_dir).unwrap();
    std::fs::write(
        cfg_dir.join("config.json"),
        "{\"projectPath\":\".global-ito\"}",
    )
    .unwrap();

    let ctx = ConfigContext {
        xdg_config_home: None,
        home_dir: Some(home.path().to_path_buf()),
        project_dir: None,
    };

    assert_eq!(get_ito_dir_name(td.path(), &ctx), ".repo-ito");
}

#[test]
fn dot_repo_config_overrides_repo_config() {
    let td = tempfile::tempdir().unwrap();
    std::fs::write(
        td.path().join("ito.json"),
        "{\"projectPath\":\".repo-ito\"}",
    )
    .unwrap();
    std::fs::write(
        td.path().join(".ito.json"),
        "{\"projectPath\":\".dot-ito\"}",
    )
    .unwrap();

    let ctx = ConfigContext::default();
    assert_eq!(get_ito_dir_name(td.path(), &ctx), ".dot-ito");
}

#[test]
fn get_ito_path_normalizes_dotdot_segments() {
    let td = tempfile::tempdir().unwrap();
    let repo = td.path();
    std::fs::create_dir_all(repo.join("a")).unwrap();
    std::fs::create_dir_all(repo.join("b")).unwrap();

    let ctx = ConfigContext::default();
    let p = repo.join("a/../b");

    let ito_path = get_ito_path(&p, &ctx);
    assert!(ito_path.ends_with("b/.ito"));
}

#[test]
fn invalid_repo_project_path_falls_back_to_default() {
    let td = tempfile::tempdir().unwrap();
    std::fs::write(
        td.path().join("ito.json"),
        "{\"projectPath\":\"../escape\"}",
    )
    .unwrap();

    let ctx = ConfigContext::default();
    assert_eq!(get_ito_dir_name(td.path(), &ctx), ".ito");
}

#[test]
fn sanitize_rejects_path_separators_and_overlong_values() {
    assert_eq!(sanitize_ito_dir_name(".ito"), Some(".ito".to_string()));
    assert_eq!(sanitize_ito_dir_name("../x"), None);
    assert_eq!(sanitize_ito_dir_name("a/b"), None);
    assert_eq!(sanitize_ito_dir_name("a\\b"), None);
    assert_eq!(sanitize_ito_dir_name(&"a".repeat(129)), None);
}
