use std::path::Path;

use std::ffi::OsString;

use ito_core::audit::{AuditEvent, AuditWriter, EventContext, FsAuditWriter};

fn run_git(repo: &Path, args: &[&str]) {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git should run");
    assert!(
        output.status.success(),
        "git command failed: git {}\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_git_repo(repo: &Path) {
    run_git(repo, &["init"]);
    run_git(repo, &["config", "user.email", "test@example.com"]);
    run_git(repo, &["config", "user.name", "Test"]);
    // Tests must be hermetic; disable GPG signing so user/global configs (e.g. 1Password)
    // cannot break `git commit`.
    run_git(repo, &["config", "commit.gpgSign", "false"]);
}

fn make_event() -> AuditEvent {
    AuditEvent {
        v: ito_domain::audit::event::SCHEMA_VERSION,
        ts: "2026-02-27T00:00:00.000Z".to_string(),
        entity: "task".to_string(),
        entity_id: "1.1".to_string(),
        scope: Some("test-change".to_string()),
        op: "status_change".to_string(),
        from: Some("pending".to_string()),
        to: Some("in-progress".to_string()),
        actor: "cli".to_string(),
        by: "@test".to_string(),
        meta: None,
        ctx: EventContext {
            session_id: "sid".to_string(),
            harness_session_id: None,
            branch: None,
            worktree: None,
            commit: None,
        },
    }
}

fn read_bare_file(bare_repo: &Path, git_ref: &str, path: &str) -> String {
    let output = std::process::Command::new("git")
        .args([
            "--git-dir",
            bare_repo.to_string_lossy().as_ref(),
            "show",
            &format!("{git_ref}:{path}"),
        ])
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git show should run");
    assert!(
        output.status.success(),
        "git show failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn branch_exists(bare_repo: &Path, git_ref: &str) -> bool {
    let output = std::process::Command::new("git")
        .args([
            "--git-dir",
            bare_repo.to_string_lossy().as_ref(),
            "show-ref",
            "--verify",
            "--quiet",
            git_ref,
        ])
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .output()
        .expect("git show-ref should run");
    output.status.success()
}

struct EnvVarGuard {
    key: &'static str,
    old: Option<OsString>,
}

impl EnvVarGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let old = std::env::var_os(key);
        // Safety: test-only env mutation; restored on drop.
        unsafe {
            std::env::set_var(key, value);
        }
        Self { key, old }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match &self.old {
            Some(value) => {
                // Safety: test-only env restoration.
                unsafe {
                    std::env::set_var(self.key, value);
                }
            }
            None => {
                // Safety: test-only env restoration.
                unsafe {
                    std::env::remove_var(self.key);
                }
            }
        }
    }
}

#[test]
fn audit_mirror_disabled_does_not_create_remote_branch() {
    let repo = tempfile::tempdir().expect("repo");
    let bare = tempfile::tempdir().expect("bare");

    init_git_repo(repo.path());
    run_git(bare.path(), &["init", "--bare"]);
    run_git(
        repo.path(),
        &[
            "remote",
            "add",
            "origin",
            bare.path().to_string_lossy().as_ref(),
        ],
    );
    std::fs::write(repo.path().join("README.md"), "hi\n").unwrap();
    run_git(repo.path(), &["add", "README.md"]);
    run_git(repo.path(), &["commit", "-m", "init"]);

    let ito_path = repo.path().join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();
    std::fs::write(
        ito_path.join("config.json"),
        r#"{"audit":{"mirror":{"enabled":false,"branch":"ito/internal/audit"}}}"#,
    )
    .unwrap();

    let ctx_home = tempfile::tempdir().expect("home");
    let _home_guard = EnvVarGuard::set("HOME", &ctx_home.path().to_string_lossy());
    let _xdg_guard = EnvVarGuard::set("XDG_CONFIG_HOME", &ctx_home.path().to_string_lossy());

    let writer = FsAuditWriter::new(&ito_path);
    writer.append(&make_event()).expect("append");

    assert!(!branch_exists(bare.path(), "refs/heads/ito/internal/audit"));
}

#[test]
fn audit_mirror_enabled_pushes_to_configured_branch() {
    let repo = tempfile::tempdir().expect("repo");
    let bare = tempfile::tempdir().expect("bare");

    init_git_repo(repo.path());
    run_git(bare.path(), &["init", "--bare"]);
    run_git(
        repo.path(),
        &[
            "remote",
            "add",
            "origin",
            bare.path().to_string_lossy().as_ref(),
        ],
    );
    std::fs::write(repo.path().join("README.md"), "hi\n").unwrap();
    run_git(repo.path(), &["add", "README.md"]);
    run_git(repo.path(), &["commit", "-m", "init"]);

    let ito_path = repo.path().join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();
    std::fs::write(
        ito_path.join("config.json"),
        r#"{"audit":{"mirror":{"enabled":true,"branch":"ito/internal/audit"}}}"#,
    )
    .unwrap();

    let ctx_home = tempfile::tempdir().expect("home");
    let _home_guard = EnvVarGuard::set("HOME", &ctx_home.path().to_string_lossy());
    let _xdg_guard = EnvVarGuard::set("XDG_CONFIG_HOME", &ctx_home.path().to_string_lossy());

    let writer = FsAuditWriter::new(&ito_path);
    writer.append(&make_event()).expect("append");

    assert!(branch_exists(bare.path(), "refs/heads/ito/internal/audit"));
    let contents = read_bare_file(
        bare.path(),
        "refs/heads/ito/internal/audit",
        ".ito/.state/audit/events.jsonl",
    );
    assert!(contents.contains("\"entity_id\":\"1.1\""));
}

#[test]
fn audit_mirror_enabled_ignores_git_hook_env_vars() {
    let repo = tempfile::tempdir().expect("repo");
    let bare = tempfile::tempdir().expect("bare");
    let unrelated_repo = tempfile::tempdir().expect("unrelated");

    init_git_repo(repo.path());
    init_git_repo(unrelated_repo.path());
    run_git(bare.path(), &["init", "--bare"]);
    run_git(
        repo.path(),
        &[
            "remote",
            "add",
            "origin",
            bare.path().to_string_lossy().as_ref(),
        ],
    );
    std::fs::write(repo.path().join("README.md"), "hi\n").unwrap();
    run_git(repo.path(), &["add", "README.md"]);
    run_git(repo.path(), &["commit", "-m", "init"]);

    let ito_path = repo.path().join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();
    std::fs::write(
        ito_path.join("config.json"),
        r#"{"audit":{"mirror":{"enabled":true,"branch":"ito/internal/audit"}}}"#,
    )
    .unwrap();

    let unrelated_git_dir = unrelated_repo.path().join(".git");
    let _git_dir_guard = EnvVarGuard::set("GIT_DIR", &unrelated_git_dir.to_string_lossy());
    let _git_work_tree_guard =
        EnvVarGuard::set("GIT_WORK_TREE", &unrelated_repo.path().to_string_lossy());

    let ctx_home = tempfile::tempdir().expect("home");
    let _home_guard = EnvVarGuard::set("HOME", &ctx_home.path().to_string_lossy());
    let _xdg_guard = EnvVarGuard::set("XDG_CONFIG_HOME", &ctx_home.path().to_string_lossy());

    let writer = FsAuditWriter::new(&ito_path);
    writer.append(&make_event()).expect("append");

    assert!(branch_exists(bare.path(), "refs/heads/ito/internal/audit"));
}

#[test]
fn audit_mirror_failures_do_not_break_local_append() {
    let repo = tempfile::tempdir().expect("repo");

    init_git_repo(repo.path());
    std::fs::write(repo.path().join("README.md"), "hi\n").unwrap();
    run_git(repo.path(), &["add", "README.md"]);
    run_git(repo.path(), &["commit", "-m", "init"]);

    // Configure an invalid origin to simulate offline/remote failure.
    run_git(repo.path(), &["remote", "add", "origin", "/does/not/exist"]);

    let ito_path = repo.path().join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();
    std::fs::write(
        ito_path.join("config.json"),
        r#"{"audit":{"mirror":{"enabled":true,"branch":"ito/internal/audit"}}}"#,
    )
    .unwrap();

    let ctx_home = tempfile::tempdir().expect("home");
    let _home_guard = EnvVarGuard::set("HOME", &ctx_home.path().to_string_lossy());
    let _xdg_guard = EnvVarGuard::set("XDG_CONFIG_HOME", &ctx_home.path().to_string_lossy());

    let writer = FsAuditWriter::new(&ito_path);
    // Append MUST succeed even if mirror fails.
    writer.append(&make_event()).expect("append");
    assert!(writer.log_path().exists());
}
