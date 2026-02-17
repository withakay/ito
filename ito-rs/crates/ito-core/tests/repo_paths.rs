use ito_config::ConfigContext;
use ito_core::repo_paths::{
    GitRepoKind, ResolvedEnv, WorktreeFeature, WorktreeSelector, resolve_env_from_cwd,
    resolve_worktree_paths,
};
use std::path::Path;

fn run(cmd: &str, args: &[&str], dir: &Path) {
    let mut command = std::process::Command::new(cmd);
    command.args(args);
    command.current_dir(dir);

    // Tests can run under coverage/hook environments where git env vars are set.
    // Clear them so subprocess `git` runs against our tempdir.
    for k in [
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_INDEX_FILE",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    ] {
        command.env_remove(k);
    }

    let status = command.status().expect("spawn command");
    assert!(status.success(), "{cmd} {:?} failed", args);
}

fn write_dir(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir should succeed");
}

#[test]
fn resolve_env_from_cwd_uses_nearest_ito_root_when_git_is_unavailable() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let root = td.path();
    write_dir(&root.join(".ito"));
    write_dir(&root.join("a").join("b"));

    let ctx = ConfigContext::default();
    let env = resolve_env_from_cwd(&root.join("a").join("b"), &ctx).expect("resolve_env");
    assert!(env.ito_root.ends_with(".ito"));
    assert_eq!(env.git_repo_kind, GitRepoKind::NonBare);
}

#[test]
fn resolve_env_from_cwd_prefers_git_toplevel() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let root = td.path();
    run("git", &["init"], root);
    write_dir(&root.join(".ito"));
    write_dir(&root.join("nested").join("dir"));

    let ctx = ConfigContext::default();
    let env = resolve_env_from_cwd(&root.join("nested").join("dir"), &ctx).expect("resolve");

    // macOS tempdirs can differ by `/private` prefix depending on the API.
    let root = std::fs::canonicalize(root).expect("canonicalize root");
    assert_eq!(
        std::fs::canonicalize(&env.worktree_root).expect("canonicalize worktree_root"),
        root
    );
    assert_eq!(
        std::fs::canonicalize(&env.project_root).expect("canonicalize project_root"),
        root
    );
    assert_eq!(
        std::fs::canonicalize(&env.ito_root).expect("canonicalize ito_root"),
        root.join(".ito")
    );
}

#[test]
fn resolve_env_from_cwd_errors_in_bare_repo_without_ito_dir() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let root = td.path();
    run("git", &["init", "--bare"], root);

    let ctx = ConfigContext::default();
    let err = resolve_env_from_cwd(root, &ctx).expect_err("bare repo should error");
    let msg = err.to_string();
    assert!(msg.contains("git worktree"));
}

#[test]
fn resolve_worktree_paths_respects_bare_control_siblings_strategy() {
    let td = tempfile::tempdir().expect("tempdir should succeed");
    let root = td.path();
    write_dir(&root.join(".ito"));
    std::fs::write(
        root.join(".ito").join("config.json"),
        r#"{
  "worktrees": {
    "enabled": true,
    "strategy": "bare_control_siblings",
    "default_branch": "main",
    "layout": {"dir_name": "ito-worktrees"}
  }
}"#,
    )
    .expect("write config");

    let env = ResolvedEnv {
        worktree_root: root.to_path_buf(),
        project_root: root.to_path_buf(),
        ito_root: root.join(".ito"),
        git_repo_kind: GitRepoKind::NonBare,
    };
    let ctx = ConfigContext {
        project_dir: Some(root.to_path_buf()),
        ..Default::default()
    };

    let wt = resolve_worktree_paths(&env, &ctx).expect("resolve_worktree_paths");
    assert!(wt.feature.is_enabled());
    assert_eq!(wt.feature, WorktreeFeature::Enabled);
    assert_eq!(
        wt.worktrees_root.as_ref().unwrap(),
        &root.join("ito-worktrees")
    );
    assert_eq!(wt.main_worktree_root.as_ref().unwrap(), &root.join("main"));

    let main = wt.path_for_selector(&WorktreeSelector::Main).unwrap();
    assert_eq!(main, root.join("main"));
    let branch = wt
        .path_for_selector(&WorktreeSelector::Branch("feat-x".to_string()))
        .unwrap();
    assert_eq!(branch, root.join("ito-worktrees").join("feat-x"));
    let change = wt
        .path_for_selector(&WorktreeSelector::Change("001-01_demo".to_string()))
        .unwrap();
    assert_eq!(change, root.join("ito-worktrees").join("001-01_demo"));
}
