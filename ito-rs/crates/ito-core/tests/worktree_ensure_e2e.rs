//! End-to-end smoke tests for the worktree ensure + init flow.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Create a minimal git repository with a single commit.
fn init_git_repo(path: &Path) {
    fs::create_dir_all(path).unwrap();
    run_git(path, &["init", "--initial-branch=main"]);
    run_git(path, &["config", "user.email", "test@example.com"]);
    run_git(path, &["config", "user.name", "Test"]);
    fs::write(path.join("README.md"), "# Test").unwrap();
    run_git(path, &["add", "."]);
    run_git(path, &["commit", "-m", "initial"]);
}

fn run_git(cwd: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .unwrap_or_else(|e| panic!("Failed to run git {:?}: {e}", args));
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn ensure_worktree_creates_and_initializes_with_include_files() {
    use ito_config::types::{
        ItoConfig, WorktreeInitConfig, WorktreeLayoutConfig, WorktreeStrategy,
    };
    use ito_core::repo_paths::{
        GitRepoKind, ResolvedEnv, ResolvedWorktreePaths, WorktreeFeature,
    };
    use ito_core::worktree_ensure::ensure_worktree;

    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path().join("repo");
    init_git_repo(&project_root);

    // Create an .env file in the project root (not committed — it's a local-only file).
    fs::write(project_root.join(".env"), "SECRET=test123").unwrap();

    let worktrees_root = tmp.path().join("ito-worktrees");

    let mut config = ItoConfig::default();
    config.worktrees.enabled = true;
    config.worktrees.strategy = WorktreeStrategy::CheckoutSiblings;
    config.worktrees.layout = WorktreeLayoutConfig {
        base_dir: None,
        dir_name: "ito-worktrees".to_string(),
    };
    config.worktrees.init = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let env = ResolvedEnv {
        worktree_root: project_root.clone(),
        project_root: project_root.clone(),
        ito_root: project_root.join(".ito"),
        git_repo_kind: GitRepoKind::NonBare,
    };

    let paths = ResolvedWorktreePaths {
        feature: WorktreeFeature::Enabled,
        strategy: WorktreeStrategy::CheckoutSiblings,
        worktrees_root: Some(worktrees_root.clone()),
        main_worktree_root: Some(project_root.clone()),
    };

    // First call: creates the worktree.
    let result = ensure_worktree("test-change", &config, &env, &paths, &project_root);
    let wt_path = result.unwrap();

    assert_eq!(wt_path, worktrees_root.join("test-change"));
    assert!(wt_path.is_dir());

    // Verify include file was copied.
    let env_file = wt_path.join(".env");
    assert!(env_file.exists(), ".env should be copied to worktree");
    assert_eq!(fs::read_to_string(&env_file).unwrap(), "SECRET=test123");

    // Second call: idempotent — returns the same path without error.
    let result2 = ensure_worktree("test-change", &config, &env, &paths, &project_root);
    assert_eq!(result2.unwrap(), wt_path);
}

#[test]
fn ensure_worktree_disabled_returns_cwd() {
    use ito_config::types::ItoConfig;
    use ito_core::repo_paths::{
        GitRepoKind, ResolvedEnv, ResolvedWorktreePaths, WorktreeFeature,
    };
    use ito_core::worktree_ensure::ensure_worktree;

    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();

    let config = ItoConfig::default();
    let env = ResolvedEnv {
        worktree_root: cwd.to_path_buf(),
        project_root: cwd.to_path_buf(),
        ito_root: cwd.join(".ito"),
        git_repo_kind: GitRepoKind::NonBare,
    };
    let paths = ResolvedWorktreePaths {
        feature: WorktreeFeature::Disabled,
        strategy: ito_config::types::WorktreeStrategy::CheckoutSubdir,
        worktrees_root: None,
        main_worktree_root: None,
    };

    let result = ensure_worktree("any-change", &config, &env, &paths, cwd);
    assert_eq!(result.unwrap(), cwd.to_path_buf());
}

#[test]
fn ensure_worktree_with_setup_script() {
    use ito_config::types::{
        ItoConfig, WorktreeInitConfig, WorktreeLayoutConfig, WorktreeSetupConfig,
        WorktreeStrategy,
    };
    use ito_core::repo_paths::{
        GitRepoKind, ResolvedEnv, ResolvedWorktreePaths, WorktreeFeature,
    };
    use ito_core::worktree_ensure::ensure_worktree;

    let tmp = tempfile::tempdir().unwrap();
    let project_root = tmp.path().join("repo");
    init_git_repo(&project_root);

    let worktrees_root = tmp.path().join("ito-worktrees");

    let mut config = ItoConfig::default();
    config.worktrees.enabled = true;
    config.worktrees.strategy = WorktreeStrategy::CheckoutSiblings;
    config.worktrees.layout = WorktreeLayoutConfig {
        base_dir: None,
        dir_name: "ito-worktrees".to_string(),
    };
    // Setup command creates a sentinel file to prove it ran.
    config.worktrees.init = WorktreeInitConfig {
        include: vec![],
        setup: Some(WorktreeSetupConfig::Single(
            "touch .setup-complete".to_string(),
        )),
    };

    let env = ResolvedEnv {
        worktree_root: project_root.clone(),
        project_root: project_root.clone(),
        ito_root: project_root.join(".ito"),
        git_repo_kind: GitRepoKind::NonBare,
    };

    let paths = ResolvedWorktreePaths {
        feature: WorktreeFeature::Enabled,
        strategy: WorktreeStrategy::CheckoutSiblings,
        worktrees_root: Some(worktrees_root.clone()),
        main_worktree_root: Some(project_root.clone()),
    };

    let result = ensure_worktree("setup-test", &config, &env, &paths, &project_root);
    let wt_path = result.unwrap();

    // Verify the setup command ran by checking for the sentinel file.
    assert!(
        wt_path.join(".setup-complete").exists(),
        "Setup command should have created .setup-complete in the worktree"
    );
}
