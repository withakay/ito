//! Rules under the `audit/*` namespace.
//!
//! Two rules live here:
//!
//! - `audit/mirror-branch-set` — when audit mirroring is enabled, the
//!   configured mirror branch must be non-empty and SHOULD live under
//!   `ito/internal/`.
//! - `audit/mirror-branch-distinct-from-coordination` — the mirror branch
//!   must not collide with the coordination branch when both worktree-based
//!   features are enabled.

use ito_config::types::{CoordinationStorage, ItoConfig};

use crate::errors::CoreError;
use crate::validate::{ValidationIssue, error, warning, with_metadata, with_rule_id};

use super::rule::{Rule, RuleContext, RuleId, RuleSeverity};

const MIRROR_BRANCH_SET_ID: RuleId = RuleId::new("audit/mirror-branch-set");
const MIRROR_BRANCH_DISTINCT_ID: RuleId =
    RuleId::new("audit/mirror-branch-distinct-from-coordination");

const ITO_INTERNAL_PREFIX: &str = "ito/internal/";

/// True when coordination storage is `worktree`.
fn coordination_is_worktree(config: &ItoConfig) -> bool {
    match config.changes.coordination_branch.storage {
        CoordinationStorage::Worktree => true,
        CoordinationStorage::Embedded => false,
    }
}

// ── audit/mirror-branch-set ──────────────────────────────────────────────

/// `audit/mirror-branch-set` — mirror branch must be non-empty and follow
/// the `ito/internal/` convention.
pub(crate) struct MirrorBranchSetRule;

impl Rule for MirrorBranchSetRule {
    fn id(&self) -> RuleId {
        MIRROR_BRANCH_SET_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    fn description(&self) -> &'static str {
        "Audit mirror branch is non-empty and follows the `ito/internal/` convention."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("audit.mirror.enabled == true")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        config.audit.mirror.enabled
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let branch = ctx.config.audit.mirror.branch.trim();
        let mut issues = Vec::new();

        if branch.is_empty() {
            let issue = warning(".ito/config.json", "`audit.mirror.branch` is empty.");
            let issue = with_rule_id(issue, MIRROR_BRANCH_SET_ID.as_str());
            issues.push(with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!(
                        "Set `audit.mirror.branch` to a value under `{ITO_INTERNAL_PREFIX}` (for example `{ITO_INTERNAL_PREFIX}audit`)."
                    ),
                    "config_key": "audit.mirror.branch",
                }),
            ));
            return Ok(issues);
        }

        if !branch.starts_with(ITO_INTERNAL_PREFIX) {
            let issue = warning(
                ".ito/config.json",
                format!(
                    "`audit.mirror.branch = \"{branch}\"` does not follow the \
                     `{ITO_INTERNAL_PREFIX}*` convention.",
                ),
            );
            let issue = with_rule_id(issue, MIRROR_BRANCH_SET_ID.as_str());
            issues.push(with_metadata(
                issue,
                serde_json::json!({
                    "fix": format!(
                        "Rename the branch under `{ITO_INTERNAL_PREFIX}` (for example \
                         `{ITO_INTERNAL_PREFIX}audit`) to keep audit mirrors in the \
                         workspace's internal namespace."
                    ),
                    "current": branch,
                }),
            ));
        }

        Ok(issues)
    }
}

// ── audit/mirror-branch-distinct-from-coordination ───────────────────────

/// `audit/mirror-branch-distinct-from-coordination` — the audit mirror
/// branch must not be the same as the coordination branch.
pub(crate) struct MirrorBranchDistinctRule;

impl Rule for MirrorBranchDistinctRule {
    fn id(&self) -> RuleId {
        MIRROR_BRANCH_DISTINCT_ID
    }

    fn severity(&self) -> RuleSeverity {
        RuleSeverity::Error
    }

    fn description(&self) -> &'static str {
        "Audit mirror branch is distinct from the coordination branch."
    }

    fn gate(&self) -> Option<&'static str> {
        Some("audit.mirror.enabled == true && changes.coordination_branch.storage == worktree")
    }

    fn is_active(&self, config: &ItoConfig) -> bool {
        config.audit.mirror.enabled && coordination_is_worktree(config)
    }

    fn check(&self, ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        let mirror = ctx.config.audit.mirror.branch.trim();
        let coord = ctx.config.changes.coordination_branch.name.trim();

        if mirror.is_empty() || coord.is_empty() || mirror != coord {
            return Ok(Vec::new());
        }

        let issue = error(
            ".ito/config.json",
            format!(
                "`audit.mirror.branch` and `changes.coordination_branch.name` are both \
                 set to `{mirror}`. A single git branch cannot store both audit events \
                 and coordination state — pushes from each subsystem would clobber the \
                 other.",
            ),
        );
        let issue = with_rule_id(issue, MIRROR_BRANCH_DISTINCT_ID.as_str());
        let issue = with_metadata(
            issue,
            serde_json::json!({
                "fix": format!(
                    "Use distinct branch names — for example `{ITO_INTERNAL_PREFIX}changes` \
                     for coordination and `{ITO_INTERNAL_PREFIX}audit` for the mirror."
                ),
                "shared_branch": mirror,
            }),
        );
        Ok(vec![issue])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRequest, ProcessRunner};
    use crate::validate_repo::staged::StagedFiles;
    use ito_config::types::{
        AuditConfig, AuditMirrorConfig, ChangesConfig, CoordinationBranchConfig,
        CoordinationStorage, ItoConfig,
    };
    use std::time::Duration;
    use tempfile::TempDir;

    struct NoopRunner;

    impl ProcessRunner for NoopRunner {
        fn run(&self, _request: &ProcessRequest) -> Result<ProcessOutput, ProcessExecutionError> {
            Ok(ProcessOutput {
                exit_code: 0,
                success: true,
                stdout: String::new(),
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

    fn config(
        mirror_enabled: bool,
        mirror_branch: &str,
        storage: CoordinationStorage,
        coord_branch: &str,
    ) -> ItoConfig {
        ItoConfig {
            audit: AuditConfig {
                mirror: AuditMirrorConfig {
                    enabled: mirror_enabled,
                    branch: mirror_branch.to_string(),
                },
            },
            changes: ChangesConfig {
                coordination_branch: CoordinationBranchConfig {
                    storage,
                    name: coord_branch.to_string(),
                    ..CoordinationBranchConfig::default()
                },
                ..ChangesConfig::default()
            },
            ..ItoConfig::default()
        }
    }

    // ── activation tests ─────────────────────────────────────────────────

    #[test]
    fn mirror_branch_set_inactive_when_mirror_disabled() {
        let cfg = config(
            false,
            "ito/internal/audit",
            CoordinationStorage::Worktree,
            "ito/internal/changes",
        );
        assert!(!MirrorBranchSetRule.is_active(&cfg));
    }

    #[test]
    fn mirror_branch_set_active_when_mirror_enabled() {
        let cfg = config(
            true,
            "ito/internal/audit",
            CoordinationStorage::Worktree,
            "ito/internal/changes",
        );
        assert!(MirrorBranchSetRule.is_active(&cfg));
    }

    #[test]
    fn mirror_branch_distinct_inactive_when_mirror_disabled() {
        let cfg = config(false, "x", CoordinationStorage::Worktree, "y");
        assert!(!MirrorBranchDistinctRule.is_active(&cfg));
    }

    #[test]
    fn mirror_branch_distinct_inactive_when_storage_embedded() {
        let cfg = config(true, "x", CoordinationStorage::Embedded, "y");
        assert!(!MirrorBranchDistinctRule.is_active(&cfg));
    }

    #[test]
    fn mirror_branch_distinct_active_when_both_enabled() {
        let cfg = config(true, "x", CoordinationStorage::Worktree, "y");
        assert!(MirrorBranchDistinctRule.is_active(&cfg));
    }

    // ── audit/mirror-branch-set ──────────────────────────────────────────

    fn ctx_for<'a>(
        cfg: &'a ItoConfig,
        tmp: &'a TempDir,
        runner: &'a NoopRunner,
        staged: &'a StagedFiles,
    ) -> RuleContext<'a> {
        RuleContext::new(cfg, tmp.path(), staged, runner)
    }

    #[test]
    fn mirror_branch_set_passes_for_canonical_name() {
        let cfg = config(
            true,
            "ito/internal/audit",
            CoordinationStorage::Worktree,
            "ito/internal/changes",
        );
        let tmp = TempDir::new().unwrap();
        let runner = NoopRunner;
        let staged = StagedFiles::empty();
        let ctx = ctx_for(&cfg, &tmp, &runner, &staged);

        let issues = MirrorBranchSetRule.check(&ctx).unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn mirror_branch_set_warns_on_empty_branch() {
        let cfg = config(
            true,
            "",
            CoordinationStorage::Worktree,
            "ito/internal/changes",
        );
        let tmp = TempDir::new().unwrap();
        let runner = NoopRunner;
        let staged = StagedFiles::empty();
        let ctx = ctx_for(&cfg, &tmp, &runner, &staged);

        let issues = MirrorBranchSetRule.check(&ctx).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].level, "WARNING");
        assert!(issues[0].message.contains("empty"));
    }

    #[test]
    fn mirror_branch_set_warns_on_non_conventional_name() {
        let cfg = config(
            true,
            "mirror/audit",
            CoordinationStorage::Worktree,
            "ito/internal/changes",
        );
        let tmp = TempDir::new().unwrap();
        let runner = NoopRunner;
        let staged = StagedFiles::empty();
        let ctx = ctx_for(&cfg, &tmp, &runner, &staged);

        let issues = MirrorBranchSetRule.check(&ctx).unwrap();
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("ito/internal/"));
    }

    // ── audit/mirror-branch-distinct ─────────────────────────────────────

    #[test]
    fn mirror_branch_distinct_passes_when_branches_differ() {
        let cfg = config(
            true,
            "ito/internal/audit",
            CoordinationStorage::Worktree,
            "ito/internal/changes",
        );
        let tmp = TempDir::new().unwrap();
        let runner = NoopRunner;
        let staged = StagedFiles::empty();
        let ctx = ctx_for(&cfg, &tmp, &runner, &staged);

        let issues = MirrorBranchDistinctRule.check(&ctx).unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn mirror_branch_distinct_fails_when_branches_match() {
        let shared = "ito/internal/changes";
        let cfg = config(true, shared, CoordinationStorage::Worktree, shared);
        let tmp = TempDir::new().unwrap();
        let runner = NoopRunner;
        let staged = StagedFiles::empty();
        let ctx = ctx_for(&cfg, &tmp, &runner, &staged);

        let issues = MirrorBranchDistinctRule.check(&ctx).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].level, "ERROR");
        assert_eq!(
            issues[0].rule_id.as_deref(),
            Some(MIRROR_BRANCH_DISTINCT_ID.as_str()),
        );
        assert!(
            issues[0].message.contains(shared),
            "error should name the shared branch; got: {}",
            issues[0].message,
        );
    }

    #[test]
    fn mirror_branch_distinct_passes_when_either_branch_empty() {
        // Empty branches are flagged separately by `mirror-branch-set` /
        // `coordination/branch-name-set`; the distinct rule should not
        // double-report.
        let cfg = config(
            true,
            "",
            CoordinationStorage::Worktree,
            "ito/internal/changes",
        );
        let tmp = TempDir::new().unwrap();
        let runner = NoopRunner;
        let staged = StagedFiles::empty();
        let ctx = ctx_for(&cfg, &tmp, &runner, &staged);

        let issues = MirrorBranchDistinctRule.check(&ctx).unwrap();
        assert!(issues.is_empty());
    }
}
