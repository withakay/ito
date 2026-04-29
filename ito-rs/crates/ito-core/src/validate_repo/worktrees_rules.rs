//! Rules under the `worktrees/*` namespace.
//!
//! These rules only run when `worktrees.enabled = true`; otherwise the
//! engine reports them as skipped via [`super::list_active_rules`].
//!
//! Two rules live here:
//!
//! - `worktrees/no-write-on-control` — fails when the current branch matches
//!   `worktrees.default_branch` and the staged-files snapshot is non-empty.
//!   This is the "default-branch worktree" check from the spec; combined
//!   with the bare-control-siblings layout used by Ito itself this is also
//!   sufficient to catch "writing to main".
//! - `worktrees/layout-consistent` — emits warnings about layout drift
//!   relative to the resolved configuration.

use std::path::Path;

use ito_config::types::{ItoConfig, WorktreeStrategy};

use crate::errors::CoreError;
use crate::process::{ProcessRequest, ProcessRunner};
use crate::validate::{ValidationIssue, error, warning, with_metadata, with_rule_id};

use super::rule::{Rule, RuleContext, RuleId, RuleSeverity};

const NO_WRITE_ON_CONTROL_ID: RuleId = RuleId::new("worktrees/no-write-on-control");
const LAYOUT_CONSISTENT_ID: RuleId = RuleId::new("worktrees/layout-consistent");

/// `worktrees/no-write-on-control` — flag staged commits in the control checkout.
pub(crate) struct NoWriteOnControlRule;

impl Rule for NoWriteOnControlRule {
    fn id(&self) -> RuleId {
        NO_WRITE_ON_CONTROL_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "Reject commits made directly in the control / default-branch worktree."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("worktrees.enabled == true")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        config.worktrees.enabled
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        // Skip cheaply when there is nothing staged. The engine treats the
        // empty result as "rule passed".
        if ctx.staged.is_empty() {
            return Ok(Vec::new());
        }

        let Some(branch) = current_branch(ctx.runner, ctx.project_root)? else {
            // Detached HEAD or git failure — nothing to do.
            return Ok(Vec::new());
        };

        let default_branch = ctx.config.worktrees.default_branch.trim();
        if default_branch.is_empty() || branch != default_branch {
            return Ok(Vec::new());
        }

        let issue = error(
            ".",
            format!(
                "Staged commits detected on the control / default-branch worktree (branch `{branch}`). \
                 Why: Ito's worktree workflow expects writes to live in change-specific worktrees so \
                 the control checkout stays clean and history stays separable per change. \
                 Fix: move the staged changes to a change worktree before committing.",
            ),
        );
        let issue = with_rule_id(issue, NO_WRITE_ON_CONTROL_ID.as_str());
        let issue = with_metadata(
            issue,
            serde_json::json!({
                "fix": "Run `ito worktree ensure --change <change-id>` and re-stage there.",
                "default_branch": branch,
                "staged_count": ctx.staged.len(),
            }),
        );

        Ok(vec![issue])
    }
}

/// Read the current branch name via `git rev-parse --abbrev-ref HEAD`.
///
/// Returns `Ok(None)` for detached HEAD (`HEAD`) or when the underlying
/// command fails non-fatally (rule passes silently in those cases).
/// Returns an error only when `git` cannot be spawned at all.
fn current_branch(
    runner: &dyn ProcessRunner,
    project_root: &Path,
) -> Result<Option<String>, CoreError> {
    let request = ProcessRequest::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(project_root);

    let output = runner.run(&request).map_err(|err| {
        CoreError::process(format!(
            "Cannot determine the current git branch.\n\
             Git command failed to run: {err}\n\
             Fix: ensure git is installed and `{root}` is a git repository.",
            root = project_root.display(),
        ))
    })?;

    if !output.success {
        return Ok(None);
    }

    let trimmed = output.stdout.trim();
    if trimmed.is_empty() || trimmed == "HEAD" {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}

/// `worktrees/layout-consistent` — minimal layout drift checks.
pub(crate) struct LayoutConsistentRule;

impl Rule for LayoutConsistentRule {
    fn id(&self) -> RuleId {
        LAYOUT_CONSISTENT_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Worktree layout configuration matches the resolved strategy and gitignore."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("worktrees.enabled == true")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        config.worktrees.enabled
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let mut issues = Vec::new();

        let dir_name = ctx.config.worktrees.layout.dir_name.trim();

        if dir_name.is_empty() {
            let issue = warning(
                ".ito/config.json",
                "`worktrees.layout.dir_name` is empty; worktree directory placement is undefined.",
            );
            let issue = with_rule_id(issue, LAYOUT_CONSISTENT_ID.as_str());
            issues.push(with_metadata(
                issue,
                serde_json::json!({
                    "fix": "Set `worktrees.layout.dir_name` to a non-empty directory name (default: `ito-worktrees`).",
                }),
            ));
        }

        if matches!(
            ctx.config.worktrees.strategy,
            WorktreeStrategy::CheckoutSubdir,
        ) && !dir_name.is_empty()
            && !gitignore_contains_dir(ctx.project_root, dir_name)?
        {
            let issue = warning(
                ".gitignore",
                format!(
                    "`worktrees.strategy = checkout_subdir` but `.gitignore` does not list `{dir_name}/`. \
                     Untracked worktree files will appear in `git status`.",
                ),
            );
            let issue = with_rule_id(issue, LAYOUT_CONSISTENT_ID.as_str());
            issues.push(with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!("Append `{dir_name}/` to `.gitignore`."),
                }),
            ));
        }

        Ok(issues)
    }
}

/// True when `.gitignore` at `project_root` contains a line matching
/// `{dir_name}/` or `{dir_name}` (trimmed).
fn gitignore_contains_dir(project_root: &Path, dir_name: &str) -> Result<bool, CoreError> {
    let gitignore = project_root.join(".gitignore");
    if !gitignore.exists() {
        return Ok(false);
    }
    let content = std::fs::read_to_string(&gitignore).map_err(|e| {
        CoreError::io(
            format!(
                "Cannot read `{path}` to check `worktrees/layout-consistent`.\n\
                 Why: filesystem error.\n\
                 Fix: confirm read permissions on `{path}`.",
                path = gitignore.display(),
            ),
            e,
        )
    })?;

    let with_slash = format!("{dir_name}/");
    Ok(content
        .lines()
        .map(str::trim)
        .any(|line| line == dir_name || line == with_slash))
}

#[cfg(test)]
mod tests {
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
}
