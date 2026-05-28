use super::*;

use ito_common::fs::StdFs;

#[test]
fn resolve_with_ctx_sets_none_when_ito_dir_is_missing() {
    let project = tempfile::tempdir().expect("tempdir");
    let ctx = ConfigContext::default();

    let resolved = ItoContext::resolve_with_ctx(&StdFs, project.path(), ctx);

    assert_eq!(resolved.project_root, project.path());
    assert_eq!(resolved.ito_path, None);
    assert_eq!(resolved.config.loaded_from, Vec::<PathBuf>::new());
}

#[test]
fn resolve_with_ctx_sets_ito_path_when_directory_exists() {
    let project = tempfile::tempdir().expect("tempdir");
    let ito_dir = project.path().join(".ito");
    std::fs::create_dir_all(&ito_dir).expect("create .ito dir");

    let resolved = ItoContext::resolve_with_ctx(&StdFs, project.path(), ConfigContext::default());

    assert_eq!(resolved.ito_path, Some(ito_dir));
}

#[test]
fn resolve_with_ctx_uses_explicit_config_context_paths() {
    let project = tempfile::tempdir().expect("tempdir");
    let xdg_home = project.path().join("xdg");
    let ctx = ConfigContext {
        xdg_config_home: Some(xdg_home.clone()),
        home_dir: Some(project.path().join("home")),
        project_dir: None,
    };

    let resolved = ItoContext::resolve_with_ctx(&StdFs, project.path(), ctx);

    assert_eq!(resolved.config_dir, Some(xdg_home.join("ito")));
}
