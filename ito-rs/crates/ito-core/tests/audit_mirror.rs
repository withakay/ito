use std::path::Path;

use ito_core::audit::{AuditEvent, AuditWriter, EventContext, FsAuditWriter, default_audit_store};

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

fn run_git_stdout(repo: &Path, args: &[&str]) -> String {
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
    String::from_utf8_lossy(&output.stdout).trim().to_string()
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
        count: 1,
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
    // Safety: tests run single-threaded with respect to env changes here.
    unsafe {
        std::env::set_var("HOME", ctx_home.path());
        std::env::set_var("XDG_CONFIG_HOME", ctx_home.path());
    }

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
    // Safety: tests run single-threaded with respect to env changes here.
    unsafe {
        std::env::set_var("HOME", ctx_home.path());
        std::env::set_var("XDG_CONFIG_HOME", ctx_home.path());
    }

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
    // Safety: tests run single-threaded with respect to env changes here.
    unsafe {
        std::env::set_var("HOME", ctx_home.path());
        std::env::set_var("XDG_CONFIG_HOME", ctx_home.path());
    }

    let writer = FsAuditWriter::new(&ito_path);
    // Append MUST succeed even if mirror fails.
    writer.append(&make_event()).expect("append");
    assert!(writer.log_path().exists());
}

#[test]
fn audit_mirror_default_local_store_writes_to_internal_branch_without_worktree_log() {
    let repo = tempfile::tempdir().expect("repo");

    init_git_repo(repo.path());
    std::fs::write(repo.path().join("README.md"), "hi\n").unwrap();
    run_git(repo.path(), &["add", "README.md"]);
    run_git(repo.path(), &["commit", "-m", "init"]);

    let ito_path = repo.path().join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();

    let store = default_audit_store(&ito_path);
    store.append(&make_event()).expect("append");

    assert!(branch_exists(
        repo.path().join(".git").as_path(),
        "refs/heads/ito/internal/audit"
    ));
    assert!(
        !ito_path.join(".state/audit/events.jsonl").exists(),
        "working tree audit log should not be created"
    );

    let contents = read_bare_file(
        repo.path().join(".git").as_path(),
        "refs/heads/ito/internal/audit",
        ".ito/.state/audit/events.jsonl",
    );
    assert!(contents.contains("\"entity_id\":\"1.1\""));
}

#[test]
fn audit_mirror_default_local_store_falls_back_without_creating_worktree_log() {
    let repo = tempfile::tempdir().expect("repo");
    let ito_path = repo.path().join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();

    let store = default_audit_store(&ito_path);
    store.append(&make_event()).expect("append");

    assert!(
        !ito_path.join(".state/audit/events.jsonl").exists(),
        "fallback should not recreate the tracked worktree log"
    );

    let events = store.read_all();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].entity_id, "1.1");
}

#[test]
fn local_store_does_not_fall_back_when_internal_branch_exists_without_log_file() {
    let repo = tempfile::tempdir().expect("repo");

    init_git_repo(repo.path());
    let starting_branch = run_git_stdout(repo.path(), &["branch", "--show-current"]);
    std::fs::write(repo.path().join("README.md"), "hi\n").unwrap();
    run_git(repo.path(), &["add", "README.md"]);
    run_git(repo.path(), &["commit", "-m", "init"]);
    run_git(repo.path(), &["checkout", "--orphan", "ito/internal/audit"]);
    run_git(repo.path(), &["rm", "-rf", "."]);
    std::fs::write(repo.path().join("placeholder.txt"), "placeholder\n").unwrap();
    run_git(repo.path(), &["add", "placeholder.txt"]);
    run_git(repo.path(), &["commit", "-m", "placeholder"]);
    run_git(repo.path(), &["checkout", &starting_branch]);

    let ito_path = repo.path().join(".ito");
    std::fs::create_dir_all(ito_path.join(".state-local/audit")).unwrap();
    std::fs::write(
        ito_path.join(".state-local/audit/events.jsonl"),
        serde_json::to_string(&make_event()).unwrap() + "\n",
    )
    .unwrap();

    let store = default_audit_store(&ito_path);
    let events = store.read_all();

    assert!(
        events.is_empty(),
        "expected empty branch history, not fallback events"
    );
}
