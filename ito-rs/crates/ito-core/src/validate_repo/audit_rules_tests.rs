use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRequest, ProcessRunner};
use crate::validate_repo::staged::StagedFiles;
use ito_config::types::{
    AuditConfig, AuditMirrorConfig, ChangesConfig, CoordinationBranchConfig, CoordinationStorage,
    ItoConfig,
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
#[cfg(feature = "coordination-branch")]
fn mirror_branch_distinct_inactive_when_mirror_disabled() {
    let cfg = config(false, "x", CoordinationStorage::Worktree, "y");
    assert!(!MirrorBranchDistinctRule.is_active(&cfg));
}

#[test]
#[cfg(feature = "coordination-branch")]
fn mirror_branch_distinct_inactive_when_storage_embedded() {
    let cfg = config(true, "x", CoordinationStorage::Embedded, "y");
    assert!(!MirrorBranchDistinctRule.is_active(&cfg));
}

#[test]
#[cfg(feature = "coordination-branch")]
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
#[cfg(feature = "coordination-branch")]
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
#[cfg(feature = "coordination-branch")]
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
#[cfg(feature = "coordination-branch")]
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
