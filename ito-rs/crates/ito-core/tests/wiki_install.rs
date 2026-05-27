use std::collections::BTreeSet;

use ito_config::ConfigContext;
use ito_core::installers::{InitOptions, InstallMode, install_default_templates};

fn install(project: &std::path::Path, mode: InstallMode, opts: InitOptions) {
    let ctx = ConfigContext {
        project_dir: Some(project.to_path_buf()),
        ..Default::default()
    };
    install_default_templates(project, &ctx, mode, &opts, None).expect("install should succeed");
}

#[test]
fn update_preserves_existing_wiki_content_and_installs_missing_scaffold() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let project = dir.path();

    install(
        project,
        InstallMode::Init,
        InitOptions::new(BTreeSet::new(), false, false),
    );

    let overview = project.join(".ito/wiki/overview.md");
    let config = project.join(".ito/wiki/_meta/config.yaml");
    let status = project.join(".ito/wiki/_meta/status.md");
    let overview_contents = "# Project-owned overview\n\nKeep this synthesis.\n";
    let config_contents = "version: 99\nwrite_policy:\n  default: project-owned\n";

    std::fs::write(&overview, overview_contents).expect("overview write should succeed");
    std::fs::write(&config, config_contents).expect("config write should succeed");
    std::fs::remove_file(&status).expect("status removal should succeed");

    install(
        project,
        InstallMode::Update,
        InitOptions::new(BTreeSet::new(), false, true),
    );

    assert_eq!(
        std::fs::read_to_string(&overview).expect("overview read should succeed"),
        overview_contents
    );
    assert_eq!(
        std::fs::read_to_string(&config).expect("config read should succeed"),
        config_contents
    );
    assert!(
        status.exists(),
        "missing wiki scaffold file should be installed"
    );
}

#[test]
fn init_upgrade_preserves_existing_wiki_content() {
    let dir = tempfile::tempdir().expect("tempdir should succeed");
    let project = dir.path();

    install(
        project,
        InstallMode::Init,
        InitOptions::new(BTreeSet::new(), false, false),
    );

    let index = project.join(".ito/wiki/index.md");
    let index_contents = "# Project-owned wiki index\n";
    std::fs::write(&index, index_contents).expect("index write should succeed");

    install(
        project,
        InstallMode::Init,
        InitOptions::new_upgrade(BTreeSet::new()),
    );

    assert_eq!(
        std::fs::read_to_string(&index).expect("index read should succeed"),
        index_contents
    );
}
