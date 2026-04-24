use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;

use ito_config::types::{WorktreeInitConfig, WorktreeSetupConfig, WorktreesConfig};

use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRequest, ProcessRunner};

// ── Stub runner for testing setup commands ────────────────────────────────────

struct StubRunner {
    outputs: RefCell<VecDeque<Result<ProcessOutput, ProcessExecutionError>>>,
    calls: RefCell<Vec<(String, Vec<String>)>>,
}

impl StubRunner {
    fn with_outputs(outputs: Vec<Result<ProcessOutput, ProcessExecutionError>>) -> Self {
        Self {
            outputs: RefCell::new(outputs.into()),
            calls: RefCell::new(Vec::new()),
        }
    }
}

impl ProcessRunner for StubRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
        self.calls
            .borrow_mut()
            .push((request.program.clone(), request.args.clone()));
        self.outputs
            .borrow_mut()
            .pop_front()
            .expect("StubRunner ran out of queued outputs")
    }

    fn run_with_timeout(
        &self,
        _request: &ProcessRequest,
        _timeout: std::time::Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        unreachable!("not used in worktree_init tests")
    }
}

fn ok_output() -> Result<ProcessOutput, ProcessExecutionError> {
    Ok(ProcessOutput {
        exit_code: 0,
        success: true,
        stdout: String::new(),
        stderr: String::new(),
        timed_out: false,
    })
}

fn fail_output(stderr: &str) -> Result<ProcessOutput, ProcessExecutionError> {
    Ok(ProcessOutput {
        exit_code: 1,
        success: false,
        stdout: String::new(),
        stderr: stderr.to_string(),
        timed_out: false,
    })
}

#[test]
fn parse_worktree_include_file_strips_comments_and_blanks() {
    let content = "# This is a comment\n\
                   .env\n\
                   \n\
                   # Another comment\n\
                   .envrc\n\
                   \n";
    let patterns = parse_worktree_include_file(content);
    assert_eq!(patterns, vec![".env", ".envrc"]);
}

#[test]
fn parse_worktree_include_file_trims_whitespace() {
    let content = "  .env  \n  # comment \n  .envrc  \n";
    let patterns = parse_worktree_include_file(content);
    assert_eq!(patterns, vec![".env", ".envrc"]);
}

#[test]
fn parse_worktree_include_file_empty_content() {
    let patterns = parse_worktree_include_file("");
    assert!(patterns.is_empty());
}

#[test]
fn parse_worktree_include_file_comments_only() {
    let content = "# comment\n# another\n";
    let patterns = parse_worktree_include_file(content);
    assert!(patterns.is_empty());
}

#[test]
fn resolve_include_files_config_only() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    // Create files in the source root
    fs::write(root.join(".env"), "SECRET=abc").unwrap();
    fs::write(root.join(".envrc"), "use nix").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string(), ".envrc".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".env"), PathBuf::from(".envrc")]);
}

#[test]
fn resolve_include_files_file_only() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join(".env"), "SECRET=abc").unwrap();
    fs::write(root.join(".worktree-include"), "# Copy env files\n.env\n").unwrap();

    let config = WorktreeInitConfig::default();

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".env")]);
}

#[test]
fn resolve_include_files_union_of_config_and_file() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join(".env"), "SECRET=abc").unwrap();
    fs::write(root.join(".envrc"), "use nix").unwrap();
    fs::write(root.join(".worktree-include"), ".envrc\n").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".env"), PathBuf::from(".envrc")]);
}

#[test]
fn resolve_include_files_deduplicates() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join(".env"), "SECRET=abc").unwrap();
    fs::write(root.join(".worktree-include"), ".env\n").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".env")]);
}

#[test]
fn resolve_include_files_missing_include_file_ok() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join(".env"), "SECRET=abc").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    // No .worktree-include file exists — should still work
    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".env")]);
}

#[test]
fn resolve_include_files_glob_expansion() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::write(root.join("app.local.toml"), "key=1").unwrap();
    fs::write(root.join("db.local.toml"), "key=2").unwrap();
    fs::write(root.join("app.toml"), "key=3").unwrap(); // should NOT match

    let config = WorktreeInitConfig {
        include: vec!["*.local.toml".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(
        files,
        vec![
            PathBuf::from("app.local.toml"),
            PathBuf::from("db.local.toml"),
        ]
    );
}

#[test]
fn resolve_include_files_no_match_returns_empty() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    // .env doesn't exist
    let files = resolve_include_files(&config, root).unwrap();
    assert!(files.is_empty());
}

#[test]
fn resolve_include_files_ignores_directories() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    fs::create_dir(root.join(".env")).unwrap(); // directory, not file
    fs::write(root.join(".envrc"), "use nix").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string(), ".envrc".to_string()],
        setup: None,
    };

    let files = resolve_include_files(&config, root).unwrap();
    assert_eq!(files, vec![PathBuf::from(".envrc")]);
}

#[test]
fn copy_include_files_copies_to_dest() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    fs::write(src.join(".env"), "SECRET=abc").unwrap();
    fs::write(src.join(".envrc"), "use nix").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string(), ".envrc".to_string()],
        setup: None,
    };

    let copied = copy_include_files(&config, src, dst).unwrap();
    assert_eq!(copied, vec![PathBuf::from(".env"), PathBuf::from(".envrc")]);

    assert_eq!(fs::read_to_string(dst.join(".env")).unwrap(), "SECRET=abc");
    assert_eq!(fs::read_to_string(dst.join(".envrc")).unwrap(), "use nix");
}

#[test]
fn copy_include_files_skips_existing_destination() {
    // Destination files that already exist are preserved (user edits protected).
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    fs::write(src.join(".env"), "NEW_SECRET").unwrap();
    fs::write(dst.join(".env"), "USER_EDIT").unwrap();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let copied = copy_include_files(&config, src, dst).unwrap();
    // Nothing should have been copied — destination already existed.
    assert!(
        copied.is_empty(),
        "expected no files copied, got: {copied:?}"
    );
    assert_eq!(
        fs::read_to_string(dst.join(".env")).unwrap(),
        "USER_EDIT",
        "existing destination file must not be overwritten"
    );
}

#[test]
fn copy_include_files_skips_missing_source() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    let config = WorktreeInitConfig {
        include: vec![".env".to_string()],
        setup: None,
    };

    let copied = copy_include_files(&config, src, dst).unwrap();
    assert!(copied.is_empty());
    assert!(!dst.join(".env").exists());
}

#[test]
fn copy_include_files_empty_config_and_no_file() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();

    let config = WorktreeInitConfig::default();

    let copied = copy_include_files(&config, src_dir.path(), dst_dir.path()).unwrap();
    assert!(copied.is_empty());
}

// ── Path traversal rejection tests ───────────────────────────────────────────

#[test]
fn resolve_include_files_rejects_path_traversal() {
    // Use a nested tempdir so the "outside" location is still isolated and
    // cleaned up automatically — no writes to the system temp root.
    let outer = tempfile::tempdir().unwrap();
    let parent = outer.path();
    let root = parent.join("inner");
    fs::create_dir(&root).unwrap();

    // Create a file outside the source root via parent traversal.
    fs::write(parent.join("secret.txt"), "password").unwrap();

    let config = WorktreeInitConfig {
        include: vec!["../secret.txt".to_string()],
        setup: None,
    };

    // The pattern resolves outside root — should not be included.
    let files = resolve_include_files(&config, &root).unwrap();
    assert!(
        files.is_empty(),
        "Path traversal via '../' should be rejected, got: {files:?}"
    );
}

#[test]
fn resolve_include_files_rejects_absolute_path_in_pattern() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let config = WorktreeInitConfig {
        include: vec!["/etc/passwd".to_string()],
        setup: None,
    };

    // Absolute paths should not be included (they resolve outside source_root).
    let files = resolve_include_files(&config, root).unwrap();
    assert!(files.is_empty());
}

// ── run_setup_with_runner tests ──────────────────────────────────────────────

#[test]
fn run_setup_no_config_is_noop() {
    let dir = tempfile::tempdir().unwrap();
    let config = WorktreesConfig::default();
    let runner = StubRunner::with_outputs(vec![]);

    run_setup_with_runner(&runner, dir.path(), &config).unwrap();
    assert!(runner.calls.borrow().is_empty());
}

#[test]
fn run_setup_single_command_invoked() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = WorktreesConfig::default();
    config.init.setup = Some(WorktreeSetupConfig::Single("make init".to_string()));

    let runner = StubRunner::with_outputs(vec![ok_output()]);

    run_setup_with_runner(&runner, dir.path(), &config).unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "sh");
    assert_eq!(calls[0].1, vec!["-c", "make init"]);
}

#[test]
fn run_setup_multiple_commands_run_in_order() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = WorktreesConfig::default();
    config.init.setup = Some(WorktreeSetupConfig::Multiple(vec![
        "npm ci".to_string(),
        "npm run build".to_string(),
    ]));

    let runner = StubRunner::with_outputs(vec![ok_output(), ok_output()]);

    run_setup_with_runner(&runner, dir.path(), &config).unwrap();

    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].1, vec!["-c", "npm ci"]);
    assert_eq!(calls[1].1, vec!["-c", "npm run build"]);
}

#[test]
fn run_setup_first_command_fails_stops_sequence() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = WorktreesConfig::default();
    config.init.setup = Some(WorktreeSetupConfig::Multiple(vec![
        "npm ci".to_string(),
        "npm run build".to_string(),
    ]));

    let runner = StubRunner::with_outputs(vec![fail_output("npm ERR!")]);

    let result = run_setup_with_runner(&runner, dir.path(), &config);
    assert!(result.is_err());

    // Only one command was attempted.
    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 1);
}

#[test]
fn run_setup_empty_single_command_is_noop() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = WorktreesConfig::default();
    config.init.setup = Some(WorktreeSetupConfig::Single(String::new()));

    let runner = StubRunner::with_outputs(vec![]);

    run_setup_with_runner(&runner, dir.path(), &config).unwrap();
    assert!(runner.calls.borrow().is_empty());
}

#[test]
fn run_setup_empty_multiple_commands_is_noop() {
    let dir = tempfile::tempdir().unwrap();
    let mut config = WorktreesConfig::default();
    config.init.setup = Some(WorktreeSetupConfig::Multiple(vec![]));

    let runner = StubRunner::with_outputs(vec![]);

    run_setup_with_runner(&runner, dir.path(), &config).unwrap();
    assert!(runner.calls.borrow().is_empty());
}

// ── init_worktree_with_runner tests ──────────────────────────────────────────

#[test]
fn init_worktree_copies_files_and_runs_setup() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    fs::write(src.join(".env"), "SECRET=abc").unwrap();

    let mut config = WorktreesConfig::default();
    config.init.include = vec![".env".to_string()];
    config.init.setup = Some(WorktreeSetupConfig::Single("echo done".to_string()));

    let runner = StubRunner::with_outputs(vec![ok_output()]);

    init_worktree_with_runner(&runner, src, dst, &config).unwrap();

    // File was copied.
    assert_eq!(fs::read_to_string(dst.join(".env")).unwrap(), "SECRET=abc");

    // Setup command was run.
    let calls = runner.calls.borrow();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].1, vec!["-c", "echo done"]);
}

#[test]
fn init_worktree_no_setup_copies_files_only() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    fs::write(src.join(".env"), "SECRET=abc").unwrap();

    let mut config = WorktreesConfig::default();
    config.init.include = vec![".env".to_string()];

    let runner = StubRunner::with_outputs(vec![]);

    init_worktree_with_runner(&runner, src, dst, &config).unwrap();

    assert_eq!(fs::read_to_string(dst.join(".env")).unwrap(), "SECRET=abc");
    assert!(runner.calls.borrow().is_empty());
}

#[test]
fn init_worktree_setup_failure_returns_error() {
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();

    let mut config = WorktreesConfig::default();
    config.init.setup = Some(WorktreeSetupConfig::Single("false".to_string()));

    let runner = StubRunner::with_outputs(vec![fail_output("setup failed")]);

    let result = init_worktree_with_runner(&runner, src_dir.path(), dst_dir.path(), &config);
    assert!(result.is_err());
}

#[test]
fn init_worktree_preserves_existing_destination_file() {
    // When a destination file already exists, init must not overwrite it.
    // This protects user edits during partial-init recovery.
    let src_dir = tempfile::tempdir().unwrap();
    let dst_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path();
    let dst = dst_dir.path();

    fs::write(src.join(".env"), "V2").unwrap();
    fs::write(dst.join(".env"), "USER_EDIT").unwrap();

    let config = WorktreesConfig {
        init: WorktreeInitConfig {
            include: vec![".env".to_string()],
            setup: None,
        },
        ..WorktreesConfig::default()
    };

    let runner = StubRunner::with_outputs(vec![]);

    init_worktree_with_runner(&runner, src, dst, &config).unwrap();

    // Destination file must retain the user's content.
    assert_eq!(
        fs::read_to_string(dst.join(".env")).unwrap(),
        "USER_EDIT",
        "init must not overwrite existing destination files"
    );
}
