//! [`Rule`] trait and supporting types.
//!
//! A [`Rule`] is a single named, config-gated check that runs against the
//! repository and produces zero or more [`crate::validate::ValidationIssue`]
//! values. The engine in [`super::run_repo_validation`] dispatches each
//! active rule and merges results into a single
//! [`crate::validate::ValidationReport`].
//!
//! Rules are deliberately small: they own their identifier, severity, an
//! activation predicate over [`ito_config::types::ItoConfig`], and a check
//! function. They do **not** own their own report builder — they just emit
//! raw issues.

use std::fmt;
use std::path::Path;

use ito_config::types::ItoConfig;

use crate::errors::CoreError;
use crate::process::ProcessRunner;
use crate::validate::ValidationIssue;

use super::staged::StagedFiles;

/// Stable identifier for a [`Rule`].
///
/// Identifiers follow a `category/kebab-case-name` convention
/// (e.g. `coordination/symlinks-wired`). They are stored as
/// `&'static str` because every built-in rule is known at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleId(&'static str);

impl RuleId {
    /// Construct a [`RuleId`] from a static string.
    ///
    /// Callers are expected to pass a literal that follows the
    /// `category/kebab-case-name` convention.
    #[must_use]
    pub const fn new(id: &'static str) -> Self {
        Self(id)
    }

    /// Return the underlying string slice.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl AsRef<str> for RuleId {
    fn as_ref(&self) -> &str {
        self.0
    }
}

/// Nominal severity declared by a [`Rule`].
///
/// This is metadata used by [`super::list_active_rules`] and hook output.
/// The severity actually applied to emitted issues is determined by the rule
/// itself when constructing the [`ValidationIssue`] (via the
/// [`crate::validate::error`] / [`crate::validate::warning`] /
/// [`crate::validate::info`] helpers). Strict mode applied at the report
/// layer can promote warnings to errors but never weakens an existing error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSeverity {
    /// The rule fails the repository when violated.
    Error,
    /// The rule reports a violation but does not fail the repository unless
    /// `--strict` is set.
    Warning,
    /// The rule reports informational findings only.
    Info,
}

impl RuleSeverity {
    /// Return the uppercase string form used by
    /// [`crate::validate::ValidationLevel`] (`"ERROR"`, `"WARNING"`,
    /// `"INFO"`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Warning => "WARNING",
            Self::Info => "INFO",
        }
    }
}

impl fmt::Display for RuleSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Inputs available to a [`Rule`] during a single check.
///
/// The context is borrowed for the duration of the check and never mutated
/// by rules. Rules SHOULD treat all fields as read-only; in particular they
/// MUST NOT spawn background tasks holding any of these references.
///
/// Marked `#[non_exhaustive]` so future fields (for example a resolved
/// worktree root for layout rules) can be added without breaking external
/// constructors.
#[non_exhaustive]
pub struct RuleContext<'a> {
    /// Resolved Ito configuration.
    pub config: &'a ItoConfig,
    /// Absolute path to the project root.
    pub project_root: &'a Path,
    /// Snapshot of files currently staged in the git index.
    pub staged: &'a StagedFiles,
    /// Process runner for shelling out to `git` and other tools.
    pub runner: &'a dyn ProcessRunner,
}

impl<'a> RuleContext<'a> {
    /// Construct a [`RuleContext`].
    #[must_use]
    pub fn new(
        config: &'a ItoConfig,
        project_root: &'a Path,
        staged: &'a StagedFiles,
        runner: &'a dyn ProcessRunner,
    ) -> Self {
        Self {
            config,
            project_root,
            staged,
            runner,
        }
    }
}

/// A repository validation rule.
///
/// Implementations are typically zero-sized structs — all per-call state
/// lives in the [`RuleContext`].
pub trait Rule: Send + Sync {
    /// Stable identifier for this rule (e.g. `coordination/symlinks-wired`).
    fn id(&self) -> RuleId;

    /// Nominal severity for this rule.
    fn severity(&self) -> RuleSeverity;

    /// Short human-readable description of what the rule checks.
    ///
    /// Used by `ito validate repo --list-rules` and `--explain`.
    fn description(&self) -> &'static str;

    /// Optional human-readable description of the activation gate, suitable
    /// for surfacing in `--list-rules` output.
    ///
    /// Examples:
    /// - `"changes.coordination_branch.storage == worktree"`
    /// - `"worktrees.enabled == true"`
    /// - `None` for rules that are always active.
    ///
    /// Defaults to `None`. Rules that gate themselves on configuration
    /// SHOULD override this so introspection output is informative.
    fn gate(&self) -> Option<&'static str> {
        None
    }

    /// Whether this rule is active for the given config.
    ///
    /// Rules SHOULD return `false` quickly when they have nothing to check
    /// (for example, coordination rules return `false` when storage is
    /// embedded). Inactive rules are skipped before [`Rule::check`] runs.
    fn is_active(&self, config: &ItoConfig) -> bool;

    /// Run the rule against the given [`RuleContext`] and return any issues.
    ///
    /// Returning an empty `Vec` means the rule passed. Rules SHOULD construct
    /// issues using the helpers in [`crate::validate`] so the rule id is
    /// consistently attached.
    ///
    /// # Errors
    ///
    /// Rules that need to invoke external tooling (for example
    /// `git check-ignore`) MAY return [`CoreError`] when the tool itself
    /// fails. The engine surfaces such errors as `ERROR`-level issues
    /// pointing at the rule, rather than aborting the whole validation run.
    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_id_round_trips_through_as_str() {
        let id = RuleId::new("coordination/symlinks-wired");
        assert_eq!(id.as_str(), "coordination/symlinks-wired");
        assert_eq!(format!("{id}"), "coordination/symlinks-wired");
        assert_eq!(id.as_ref(), "coordination/symlinks-wired");
    }

    #[test]
    fn rule_id_is_orderable_for_deterministic_output() {
        let mut ids = [
            RuleId::new("coordination/staged-symlinked-paths"),
            RuleId::new("coordination/symlinks-wired"),
            RuleId::new("coordination/branch-name-set"),
        ];
        ids.sort();
        assert_eq!(
            ids.iter().map(|id| id.as_str()).collect::<Vec<_>>(),
            vec![
                "coordination/branch-name-set",
                "coordination/staged-symlinked-paths",
                "coordination/symlinks-wired",
            ]
        );
    }

    #[test]
    fn rule_severity_string_matches_validation_levels() {
        assert_eq!(RuleSeverity::Error.as_str(), "ERROR");
        assert_eq!(RuleSeverity::Warning.as_str(), "WARNING");
        assert_eq!(RuleSeverity::Info.as_str(), "INFO");
    }
}
