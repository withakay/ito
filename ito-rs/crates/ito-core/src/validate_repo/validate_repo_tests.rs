//! Engine smoke tests.
//!
//! Per-rule behaviour is exercised in each rule module's own `tests`
//! submodule; these tests focus on the engine's gate-filtering and
//! report-merging behaviour.

use super::*;
use crate::process::SystemProcessRunner;
use ito_config::types::{CoordinationStorage, ItoConfig};
use std::path::Path;

fn embedded_config() -> ItoConfig {
    // Worktree-storage gates leave most rules inactive in this
    // configuration; only the always-active rules (e.g.
    // `coordination/branch-name-set`) can emit.
    let mut cfg = ItoConfig::default();
    cfg.changes.coordination_branch.storage = CoordinationStorage::Embedded;
    cfg.worktrees.enabled = false;
    cfg
}

#[test]
fn run_repo_validation_skips_inactive_rules() {
    let config = embedded_config();
    let runner = SystemProcessRunner;
    let staged = StagedFiles::empty();

    let report = run_repo_validation(&config, Path::new("/"), &staged, &runner, false);

    // The default coordination branch name `ito/internal/changes`
    // satisfies `coordination/branch-name-set`, and every other rule
    // is gated off by the embedded/disabled config. Result: a clean
    // report.
    assert!(
        report.valid,
        "expected valid report; issues: {:?}",
        report.issues
    );
    assert!(report.issues.is_empty());
}

#[test]
fn run_repo_validation_strict_promotes_warnings_to_errors() {
    // Branch name does not start with `ito/internal/` → branch-name-set
    // emits a WARNING. With `strict = true`, the report should be
    // invalid because warnings count as errors.
    let mut config = embedded_config();
    config.changes.coordination_branch.name = "coordination/foo".to_string();
    let runner = SystemProcessRunner;
    let staged = StagedFiles::empty();

    let lenient = run_repo_validation(&config, Path::new("/"), &staged, &runner, false);
    assert!(!lenient.issues.is_empty(), "warning expected");
    assert!(
        lenient.valid,
        "lenient mode: warning should not invalidate the report"
    );

    let strict = run_repo_validation(&config, Path::new("/"), &staged, &runner, true);
    assert!(
        !strict.valid,
        "strict mode: warning should invalidate the report"
    );
}
