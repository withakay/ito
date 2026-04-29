//! Repository-level validation rules driven by `ito_config::types::ItoConfig`.
//!
//! Unlike `crate::validate`, which checks individual on-disk artifacts
//! (proposals, specs, tasks, audit logs), this module validates the
//! **repository as a whole**: gitignore wiring, coordination worktree
//! symlinks, staged file policy, worktree layout, and so on.
//!
//! The engine is config-aware: each `Rule` declares whether it is active
//! for a given configuration via `Rule::is_active`. Rules that are inactive
//! are skipped silently (they emit no issues and report `active: false` in
//! `list_active_rules` output).
//!
//! Output is rendered via the existing `crate::validate::ValidationReport`
//! envelope so callers in `ito-cli` and elsewhere can reuse rendering and
//! JSON serialization.
//!
//! # Module layout
//!
//! - `rule` — `Rule` trait, `RuleId`, `RuleSeverity`, `RuleContext`.
//! - `registry` — `RuleRegistry` holding the built-in rule list and the
//!   `list_active_rules` introspection helper.
//! - `staged` — `StagedFiles` snapshot reader.
//!
//! Subsequent waves add rule implementations:
//!
//! - `coordination_rules` — Wave 2.
//! - `worktrees_rules` — Wave 2.
//! - `pre_commit_detect` — Wave 2.
//! - `audit_rules`, `repository_rules`, `backend_rules` — change `011-06`.

use std::path::Path;

use ito_config::types::ItoConfig;

use crate::process::ProcessRunner;
use crate::validate::{ValidationReport, report, with_rule_id};

pub mod registry;
pub mod rule;
pub mod staged;

pub use registry::{ActiveRule, RuleRegistry, list_active_rules, list_active_rules_for};
pub use rule::{Rule, RuleContext, RuleId, RuleSeverity};
pub use staged::StagedFiles;

/// Run repository validation against the given config and project root.
///
/// Iterates the built-in [`RuleRegistry`], skipping rules that report
/// `is_active(config) == false`, and merges the resulting issues into a
/// single [`ValidationReport`].
///
/// # Parameters
///
/// - `config`: the resolved [`ItoConfig`] for the project. Rules use this to
///   gate themselves (e.g. coordination rules only run when
///   `changes.coordination_branch.storage == Worktree`).
/// - `project_root`: absolute path to the project root.
/// - `staged`: snapshot of the git index, used by rules that only fire on
///   staged paths (e.g. the pre-commit hook flow). Pass
///   [`StagedFiles::empty()`] for full-repo validation.
/// - `runner`: process runner used by rules that need to invoke `git` (e.g.
///   `git check-ignore`). Tests inject mock runners.
/// - `strict`: if `true`, warnings are promoted to errors in the resulting
///   [`ValidationReport`] (matches the existing `--strict` semantics in
///   [`crate::validate`]).
///
/// # Errors
///
/// Engine-level errors are converted to `ERROR`-level
/// [`crate::validate::ValidationIssue`] entries pointing at the failing
/// rule, rather than aborting the whole validation run. The engine itself
/// is infallible.
pub fn run_repo_validation(
    config: &ItoConfig,
    project_root: &Path,
    staged: &StagedFiles,
    runner: &dyn ProcessRunner,
    strict: bool,
) -> ValidationReport {
    let registry = RuleRegistry::built_in();
    let ctx = RuleContext::new(config, project_root, staged, runner);
    let mut builder = report(strict);

    for rule in registry.iter() {
        if !rule.is_active(config) {
            continue;
        }
        match rule.check(&ctx) {
            Ok(issues) => builder.extend(issues),
            Err(err) => {
                let mut issue = crate::validate::error(rule.id().as_str(), err.to_string());
                issue = with_rule_id(issue, rule.id().as_str());
                builder.push(issue);
            }
        }
    }

    builder.finish()
}

#[cfg(test)]
mod tests {
    //! Smoke tests for the engine scaffold.
    //!
    //! Wave 1 only verifies the module compiles and the stub returns a clean
    //! report when the built-in registry is empty. Per-rule tests live in
    //! subsequent waves alongside the rule modules they cover.

    use super::*;
    use crate::process::SystemProcessRunner;
    use ito_config::types::ItoConfig;
    use std::path::Path;

    #[test]
    fn run_repo_validation_with_empty_registry_returns_empty_report() {
        let config = ItoConfig::default();
        let runner = SystemProcessRunner;
        let staged = StagedFiles::empty();

        let report = run_repo_validation(&config, Path::new("/"), &staged, &runner, false);

        assert!(report.valid, "empty-registry report must be valid");
        assert!(
            report.issues.is_empty(),
            "empty-registry report must have no issues"
        );
        assert_eq!(report.summary.errors, 0);
        assert_eq!(report.summary.warnings, 0);
        assert_eq!(report.summary.info, 0);
    }

    #[test]
    fn run_repo_validation_strict_with_empty_registry_still_empty() {
        let config = ItoConfig::default();
        let runner = SystemProcessRunner;
        let staged = StagedFiles::empty();

        let report = run_repo_validation(&config, Path::new("/"), &staged, &runner, true);

        assert!(report.valid);
        assert!(report.issues.is_empty());
    }
}
