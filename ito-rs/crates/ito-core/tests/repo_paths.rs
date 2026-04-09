use ito_config::ConfigContext;
use ito_config::types::CoordinationBranchConfig;
use ito_core::repo_paths::{
    GitRepoKind, ResolvedEnv, WorktreeFeature, WorktreeSelector, coordination_worktree_path,
    resolve_env_from_cwd, resolve_worktree_paths,
};
use std::path::{Path, PathBuf};

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
    // Hold ENV_MUTEX so this test is not interleaved with tests that mutate
    // HOME/XDG_DATA_HOME — git reads HOME to locate global config.
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
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
    // Hold ENV_MUTEX so this test is not interleaved with tests that mutate
    // HOME/XDG_DATA_HOME — git reads HOME to locate global config.
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
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

// ── coordination_worktree_path ────────────────────────────────────────────────
//
// These tests manipulate environment variables. Each test uses a unique env-var
// guard pattern: set the variable, run assertions, then restore the original
// value. Tests are not run in parallel within this file (Rust integration tests
// run each `#[test]` in its own thread but share the process environment), so
// we serialise the env-touching tests with a mutex.

use std::sync::Mutex;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

/// RAII guard that restores a single environment variable on drop, even if the
/// enclosed closure panics.
///
/// `restore_to` is the value to restore:
/// - `Some(v)` → set the variable back to `v`
/// - `None` → remove the variable (it was absent before the test)
///
/// The caller is responsible for holding `ENV_MUTEX` for the lifetime of this
/// guard. `Drop` does **not** re-acquire the mutex — doing so would deadlock
/// because Rust drops locals in reverse declaration order (guard drops before
/// the mutex lock is released).
struct EnvRestore {
    var: &'static str,
    restore_to: Option<String>,
}

impl Drop for EnvRestore {
    fn drop(&mut self) {
        // SAFETY: caller holds ENV_MUTEX for the duration of this guard's
        // lifetime, so no other thread can modify this env var concurrently.
        match &self.restore_to {
            Some(v) => unsafe { std::env::set_var(self.var, v) },
            None => unsafe { std::env::remove_var(self.var) },
        }
    }
}

/// Temporarily set `XDG_DATA_HOME` to `value` for the duration of `f`, then
/// restore the previous value (or remove the variable if it was not set).
/// Uses an [`EnvRestore`] drop guard so env changes are reverted even on panic.
fn with_xdg_data_home<F: FnOnce()>(value: &str, f: F) {
    // `_restore_xdg` drops before `_lock` (reverse declaration order), so the
    // mutex is still held when the env var is restored.
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
    let _restore_xdg = EnvRestore {
        var: "XDG_DATA_HOME",
        restore_to: std::env::var("XDG_DATA_HOME").ok(),
    };
    // SAFETY: guarded by ENV_MUTEX; no other thread modifies env vars concurrently.
    unsafe { std::env::set_var("XDG_DATA_HOME", value) };
    f();
}

/// Temporarily remove `XDG_DATA_HOME` and set `HOME` to `value` for the
/// duration of `f`, then restore both variables.
/// Uses [`EnvRestore`] drop guards so env changes are reverted even on panic.
fn with_home_only<F: FnOnce()>(home_value: &str, f: F) {
    // Guards drop before `_lock` (reverse declaration order), so the mutex is
    // still held when env vars are restored.
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
    let _restore_xdg = EnvRestore {
        var: "XDG_DATA_HOME",
        restore_to: std::env::var("XDG_DATA_HOME").ok(),
    };
    let _restore_home = EnvRestore {
        var: "HOME",
        restore_to: std::env::var("HOME").ok(),
    };
    // SAFETY: guarded by ENV_MUTEX; no other thread modifies env vars concurrently.
    unsafe {
        std::env::remove_var("XDG_DATA_HOME");
        std::env::set_var("HOME", home_value);
    }
    f();
}

#[test]
fn coordination_worktree_path_uses_explicit_worktree_path_when_set() {
    let config = CoordinationBranchConfig {
        worktree_path: Some("/custom/path/to/worktree".to_string()),
        ..CoordinationBranchConfig::default()
    };
    let ito_path = Path::new("/project/.ito");
    let result = coordination_worktree_path(&config, ito_path, "acme", "widget");
    assert_eq!(result, PathBuf::from("/custom/path/to/worktree"));
}

#[test]
fn coordination_worktree_path_ignores_xdg_when_explicit_path_set() {
    with_xdg_data_home("/xdg/data", || {
        let config = CoordinationBranchConfig {
            worktree_path: Some("/explicit/override".to_string()),
            ..CoordinationBranchConfig::default()
        };
        let ito_path = Path::new("/project/.ito");
        let result = coordination_worktree_path(&config, ito_path, "acme", "widget");
        assert_eq!(result, PathBuf::from("/explicit/override"));
    });
}

#[test]
fn coordination_worktree_path_uses_xdg_data_home_when_set() {
    with_xdg_data_home("/xdg/data", || {
        let config = CoordinationBranchConfig::default();
        let ito_path = Path::new("/project/.ito");
        let result = coordination_worktree_path(&config, ito_path, "acme", "widget");
        assert_eq!(result, PathBuf::from("/xdg/data/ito/acme/widget"));
    });
}

#[test]
fn coordination_worktree_path_falls_back_to_local_share_when_xdg_unset() {
    with_home_only("/home/alice", || {
        let config = CoordinationBranchConfig::default();
        let ito_path = Path::new("/project/.ito");
        let result = coordination_worktree_path(&config, ito_path, "acme", "widget");
        assert_eq!(
            result,
            PathBuf::from("/home/alice/.local/share/ito/acme/widget")
        );
    });
}

#[test]
fn coordination_worktree_path_correct_structure_with_xdg() {
    with_xdg_data_home("/data", || {
        let config = CoordinationBranchConfig::default();
        let ito_path = Path::new("/project/.ito");
        let result = coordination_worktree_path(&config, ito_path, "withakay", "ito");
        // Must be: <XDG_DATA_HOME>/ito/<org>/<repo>
        assert_eq!(result, PathBuf::from("/data/ito/withakay/ito"));
    });
}

#[test]
fn coordination_worktree_path_correct_structure_with_home_fallback() {
    with_home_only("/home/bob", || {
        let config = CoordinationBranchConfig::default();
        let ito_path = Path::new("/project/.ito");
        let result = coordination_worktree_path(&config, ito_path, "withakay", "ito");
        // Must be: ~/.local/share/ito/<org>/<repo>
        assert_eq!(
            result,
            PathBuf::from("/home/bob/.local/share/ito/withakay/ito")
        );
    });
}

#[test]
fn coordination_worktree_path_last_resort_uses_ito_path() {
    // When both HOME and XDG_DATA_HOME are absent, the fallback must be
    // <ito_path>/coordination-worktree — an absolute, project-scoped path.
    let _lock = ENV_MUTEX.lock().unwrap_or_else(|p| p.into_inner());
    let _restore_xdg = EnvRestore {
        var: "XDG_DATA_HOME",
        restore_to: std::env::var("XDG_DATA_HOME").ok(),
    };
    let _restore_home = EnvRestore {
        var: "HOME",
        restore_to: std::env::var("HOME").ok(),
    };
    let _restore_userprofile = EnvRestore {
        var: "USERPROFILE",
        restore_to: std::env::var("USERPROFILE").ok(),
    };
    // SAFETY: guarded by ENV_MUTEX; no other thread modifies env vars concurrently.
    unsafe {
        std::env::remove_var("XDG_DATA_HOME");
        std::env::remove_var("HOME");
        std::env::remove_var("USERPROFILE");
    }

    let config = CoordinationBranchConfig::default();
    let ito_path = Path::new("/absolute/project/.ito");
    let result = coordination_worktree_path(&config, ito_path, "acme", "widget");
    assert_eq!(
        result,
        PathBuf::from("/absolute/project/.ito/coordination-worktree")
    );
}
