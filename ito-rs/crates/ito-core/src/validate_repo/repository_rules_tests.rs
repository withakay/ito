use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput};
use crate::validate_repo::staged::StagedFiles;
use ito_config::types::{ItoConfig, RepositoryRuntimeConfig, RepositorySqliteConfig};
use std::cell::RefCell;
use std::time::Duration;
use tempfile::TempDir;

/// Test runner that returns canned exit codes per `(program, args[0..N])` prefix.
///
/// The first matching rule wins; falls back to a default if no rule matches.
struct ScriptedRunner {
    rules: Vec<(Vec<&'static str>, ProcessOutput)>,
    default: ProcessOutput,
    seen: RefCell<Vec<ProcessRequest>>,
}

fn ok_output(success: bool, exit_code: i32) -> ProcessOutput {
    ProcessOutput {
        exit_code,
        success,
        stdout: String::new(),
        stderr: String::new(),
        timed_out: false,
    }
}

impl ScriptedRunner {
    fn new(default: ProcessOutput) -> Self {
        Self {
            rules: Vec::new(),
            default,
            seen: RefCell::new(Vec::new()),
        }
    }

    fn with_rule(mut self, args_prefix: &[&'static str], output: ProcessOutput) -> Self {
        self.rules.push((args_prefix.to_vec(), output));
        self
    }
}

impl ProcessRunner for ScriptedRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
        self.seen.borrow_mut().push(request.clone());
        for (prefix, output) in &self.rules {
            if request.args.len() >= prefix.len()
                && request.args.iter().zip(prefix.iter()).all(|(a, b)| a == b)
            {
                return Ok(output.clone());
            }
        }
        Ok(self.default.clone())
    }
    fn run_with_timeout(
        &self,
        request: &ProcessRequest,
        _timeout: Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        self.run(request)
    }
}

fn config(mode: RepositoryPersistenceMode, db_path: Option<&str>) -> ItoConfig {
    ItoConfig {
        repository: RepositoryRuntimeConfig {
            mode,
            sqlite: RepositorySqliteConfig {
                db_path: db_path.map(str::to_string),
            },
        },
        ..ItoConfig::default()
    }
}

// ── activation ───────────────────────────────────────────────────────

#[test]
fn rules_inactive_in_filesystem_mode() {
    let cfg = config(
        RepositoryPersistenceMode::Filesystem,
        Some(".ito/state/ito.db"),
    );
    assert!(!SqliteDbPathSetRule.is_active(&cfg));
    assert!(!SqliteDbNotCommittedRule.is_active(&cfg));
}

#[test]
fn rules_active_in_sqlite_mode() {
    let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
    assert!(SqliteDbPathSetRule.is_active(&cfg));
    assert!(SqliteDbNotCommittedRule.is_active(&cfg));
}

// ── repository/sqlite-db-path-set ────────────────────────────────────

#[test]
fn sqlite_db_path_set_errors_when_path_unset() {
    let cfg = config(RepositoryPersistenceMode::Sqlite, None);
    let tmp = TempDir::new().unwrap();
    let runner = ScriptedRunner::new(ok_output(false, 1));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].level, "ERROR");
    assert!(issues[0].message.contains("empty or unset"));
}

#[test]
fn sqlite_db_path_set_errors_when_path_outside_project_root() {
    let cfg = config(RepositoryPersistenceMode::Sqlite, Some("/var/tmp/ito.db"));
    let tmp = TempDir::new().unwrap();
    let runner = ScriptedRunner::new(ok_output(false, 1));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert!(issues[0].message.contains("outside the project root"));
}

#[test]
fn sqlite_db_path_set_errors_when_path_escapes_root_via_dotdot() {
    // Regression test for a `..` traversal bypass in path_inside_root:
    // `Path::starts_with` is component-based and does not resolve `..`,
    // so a path of `.ito/state/../../../etc/passwd` could appear to
    // "start with" the project root while actually escaping it.
    let cfg = config(
        RepositoryPersistenceMode::Sqlite,
        Some(".ito/state/../../../etc/passwd"),
    );
    let tmp = TempDir::new().unwrap();
    let runner = ScriptedRunner::new(ok_output(false, 1));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
    assert_eq!(
        issues.len(),
        1,
        "expected one error for `..` traversal escape; got {issues:?}",
    );
    assert!(
        issues[0].message.contains("outside the project root"),
        "expected outside-root message; got: {}",
        issues[0].message,
    );
}

#[test]
fn sqlite_db_path_set_warns_when_parent_directory_missing() {
    let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
    let tmp = TempDir::new().unwrap();
    // Don't create `.ito/state/`.
    let runner = ScriptedRunner::new(ok_output(false, 1));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].level, "WARNING");
    assert!(issues[0].message.contains("parent directory"));
}

#[test]
fn sqlite_db_path_set_passes_when_parent_exists() {
    let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".ito/state")).unwrap();
    let runner = ScriptedRunner::new(ok_output(false, 1));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbPathSetRule.check(&ctx).unwrap();
    assert!(issues.is_empty(), "expected no issues; got {issues:?}");
}

// ── repository/sqlite-db-not-committed ───────────────────────────────

#[test]
fn sqlite_db_not_committed_errors_when_file_is_tracked() {
    let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
    let tmp = TempDir::new().unwrap();
    // ls-files --error-unmatch returns success → tracked.
    let runner = ScriptedRunner::new(ok_output(false, 1))
        .with_rule(&["ls-files", "--error-unmatch"], ok_output(true, 0));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbNotCommittedRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].level, "ERROR");
    assert!(issues[0].message.contains("tracked by git"));
    let metadata = issues[0].metadata.as_ref().expect("metadata");
    let untrack = metadata
        .get("untrack_command")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(untrack.starts_with("git rm --cached"));
}

#[test]
fn sqlite_db_not_committed_warns_when_untracked_and_not_ignored() {
    let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
    let tmp = TempDir::new().unwrap();
    // ls-files --error-unmatch fails (untracked); check-ignore fails (not ignored).
    let runner = ScriptedRunner::new(ok_output(false, 1));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbNotCommittedRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].level, "WARNING");
    assert!(issues[0].message.contains("not covered by `.gitignore`"));
}

#[test]
fn sqlite_db_not_committed_passes_when_untracked_and_ignored() {
    let cfg = config(RepositoryPersistenceMode::Sqlite, Some(".ito/state/ito.db"));
    let tmp = TempDir::new().unwrap();
    // ls-files fails (untracked); check-ignore succeeds (ignored).
    let runner =
        ScriptedRunner::new(ok_output(false, 1)).with_rule(&["check-ignore"], ok_output(true, 0));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbNotCommittedRule.check(&ctx).unwrap();
    assert!(issues.is_empty());
}

#[test]
fn sqlite_db_not_committed_silent_when_path_unset() {
    // Companion rule `sqlite-db-path-set` reports the missing path.
    let cfg = config(RepositoryPersistenceMode::Sqlite, None);
    let tmp = TempDir::new().unwrap();
    let runner = ScriptedRunner::new(ok_output(false, 1));
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SqliteDbNotCommittedRule.check(&ctx).unwrap();
    assert!(issues.is_empty());
}
