use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput};
use crate::validate_repo::staged::StagedFiles;
use ito_config::types::{ItoConfig, WorktreesConfig};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

/// Test runner that returns canned output for git commands.
struct CannedRunner {
    stdout: String,
    success: bool,
}

impl CannedRunner {
    fn branch(branch: &str) -> Self {
        Self {
            stdout: format!("{branch}\n"),
            success: true,
        }
    }

    fn detached() -> Self {
        Self {
            stdout: "HEAD\n".to_string(),
            success: true,
        }
    }

    fn failed() -> Self {
        Self {
            stdout: String::new(),
            success: false,
        }
    }
}

impl ProcessRunner for CannedRunner {
    fn run(&self, _request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
        Ok(ProcessOutput {
            exit_code: if self.success { 0 } else { 1 },
            success: self.success,
            stdout: self.stdout.clone(),
            stderr: String::new(),
            timed_out: false,
        })
    }
    fn run_with_timeout(
        &self,
        request: &ProcessRequest,
        _timeout: Duration,
    ) -> Result<ProcessOutput, ProcessExecutionError> {
        self.run(request)
    }
}

fn config_with_worktrees(enabled: bool, strategy: WorktreeStrategy) -> ItoConfig {
    ItoConfig {
        worktrees: WorktreesConfig {
            enabled,
            strategy,
            ..WorktreesConfig::default()
        },
        ..ItoConfig::default()
    }
}

#[test]
fn no_write_on_control_inactive_when_worktrees_disabled() {
    let cfg = config_with_worktrees(false, WorktreeStrategy::CheckoutSubdir);
    assert!(!NoWriteOnControlRule.is_active(&cfg));
}

#[test]
fn no_write_on_control_active_when_worktrees_enabled() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    assert!(NoWriteOnControlRule.is_active(&cfg));
}

#[test]
fn no_write_on_control_passes_when_no_staged_files() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    let tmp = TempDir::new().unwrap();
    let runner = CannedRunner::branch("main");
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = NoWriteOnControlRule.check(&ctx).unwrap();
    assert!(issues.is_empty(), "no staged files => rule passes");
}

#[test]
fn no_write_on_control_fails_when_on_default_branch_with_staged_files() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    // default_branch defaults to "main".
    assert_eq!(cfg.worktrees.default_branch, "main");

    let tmp = TempDir::new().unwrap();
    let runner = CannedRunner::branch("main");
    let staged = StagedFiles::from_paths(vec![PathBuf::from("README.md")]);
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = NoWriteOnControlRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1, "expected one error, got {issues:?}");
    assert_eq!(issues[0].level, "ERROR");
    assert_eq!(
        issues[0].rule_id.as_deref(),
        Some(NO_WRITE_ON_CONTROL_ID.as_str()),
    );
}

#[test]
fn no_write_on_control_passes_in_change_branch() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    let tmp = TempDir::new().unwrap();
    let runner = CannedRunner::branch("011-05_demo");
    let staged = StagedFiles::from_paths(vec![PathBuf::from("README.md")]);
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = NoWriteOnControlRule.check(&ctx).unwrap();
    assert!(
        issues.is_empty(),
        "change branch with staged files should pass; got {issues:?}",
    );
}

#[test]
fn no_write_on_control_passes_on_detached_head() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    let tmp = TempDir::new().unwrap();
    let runner = CannedRunner::detached();
    let staged = StagedFiles::from_paths(vec![PathBuf::from("README.md")]);
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = NoWriteOnControlRule.check(&ctx).unwrap();
    assert!(issues.is_empty(), "detached HEAD => rule passes silently");
}

#[test]
fn no_write_on_control_passes_when_git_command_fails() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    let tmp = TempDir::new().unwrap();
    let runner = CannedRunner::failed();
    let staged = StagedFiles::from_paths(vec![PathBuf::from("README.md")]);
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = NoWriteOnControlRule.check(&ctx).unwrap();
    assert!(issues.is_empty(), "git failure => rule passes silently");
}

#[test]
fn layout_consistent_inactive_when_worktrees_disabled() {
    let cfg = config_with_worktrees(false, WorktreeStrategy::CheckoutSubdir);
    assert!(!LayoutConsistentRule.is_active(&cfg));
}

#[test]
fn layout_consistent_warns_on_empty_dir_name() {
    let mut cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    cfg.worktrees.layout.dir_name = String::new();

    let tmp = TempDir::new().unwrap();
    let runner = CannedRunner::branch("main");
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = LayoutConsistentRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(
        issues[0].rule_id.as_deref(),
        Some(LAYOUT_CONSISTENT_ID.as_str()),
    );
    assert!(issues[0].message.contains("dir_name"));
}

#[test]
fn layout_consistent_warns_when_checkout_subdir_missing_gitignore_entry() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    let tmp = TempDir::new().unwrap();
    // .gitignore exists but does not contain ito-worktrees/.
    std::fs::write(tmp.path().join(".gitignore"), "target/\n").unwrap();

    let runner = CannedRunner::branch("main");
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = LayoutConsistentRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1, "expected one warning, got {issues:?}");
    assert!(
        issues[0].message.contains("ito-worktrees"),
        "warning should name the missing dir; got: {}",
        issues[0].message,
    );
}

#[test]
fn layout_consistent_quiet_when_gitignore_has_entry() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSubdir);
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join(".gitignore"), "ito-worktrees/\n").unwrap();

    let runner = CannedRunner::branch("main");
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = LayoutConsistentRule.check(&ctx).unwrap();
    assert!(issues.is_empty(), "gitignore has entry => no warning");
}

#[test]
fn layout_consistent_quiet_for_bare_control_siblings_strategy() {
    let cfg = config_with_worktrees(true, WorktreeStrategy::BareControlSiblings);
    let tmp = TempDir::new().unwrap();
    // No .gitignore at all — bare_control_siblings does not require one.
    let runner = CannedRunner::branch("main");
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = LayoutConsistentRule.check(&ctx).unwrap();
    assert!(
        issues.is_empty(),
        "bare_control_siblings should not require a gitignore entry; got {issues:?}",
    );
}

#[test]
fn layout_consistent_quiet_for_checkout_siblings_strategy() {
    // CheckoutSiblings places worktrees alongside the project (e.g.
    // `<parent>/<project>-ito-worktrees/`); the project's own
    // `.gitignore` therefore does not need a `<dir_name>/` entry.
    let cfg = config_with_worktrees(true, WorktreeStrategy::CheckoutSiblings);
    let tmp = TempDir::new().unwrap();
    // No .gitignore at all.
    let runner = CannedRunner::branch("main");
    let staged = StagedFiles::empty();
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = LayoutConsistentRule.check(&ctx).unwrap();
    assert!(
        issues.is_empty(),
        "checkout_siblings should not require a gitignore entry; got {issues:?}",
    );
}
