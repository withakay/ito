use super::*;
use crate::process::{ProcessExecutionError, ProcessOutput, ProcessRequest, ProcessRunner};
use crate::validate_repo::staged::StagedFiles;
use ito_config::types::{ChangesConfig, CoordinationBranchConfig, CoordinationStorage, ItoConfig};
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

/// Process runner that always reports success with empty stdout.
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

fn config_with_storage(storage: CoordinationStorage) -> ItoConfig {
    ItoConfig {
        changes: ChangesConfig {
            coordination_branch: CoordinationBranchConfig {
                storage,
                ..CoordinationBranchConfig::default()
            },
            ..ChangesConfig::default()
        },
        ..ItoConfig::default()
    }
}

// ── activation tests ─────────────────────────────────────────────────

#[test]
fn rules_inactive_when_storage_is_embedded() {
    let cfg = config_with_storage(CoordinationStorage::Embedded);
    assert!(!SymlinksWiredRule.is_active(&cfg));
    assert!(!GitignoreEntriesRule.is_active(&cfg));
    assert!(!StagedSymlinkedPathsRule.is_active(&cfg));
    // branch-name-set is always active.
    assert!(BranchNameSetRule.is_active(&cfg));
}

#[test]
fn rules_active_when_storage_is_worktree() {
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    assert!(SymlinksWiredRule.is_active(&cfg));
    assert!(GitignoreEntriesRule.is_active(&cfg));
    assert!(StagedSymlinkedPathsRule.is_active(&cfg));
    assert!(BranchNameSetRule.is_active(&cfg));
}

// ── coordination/gitignore-entries ───────────────────────────────────

#[test]
fn gitignore_entries_passes_when_all_canonical_lines_present() {
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    let tmp = TempDir::new().unwrap();
    let body = gitignore_entries()
        .iter()
        .map(|s| (*s).to_string())
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(tmp.path().join(".gitignore"), format!("{body}\n")).unwrap();

    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = GitignoreEntriesRule.check(&ctx).unwrap();
    assert!(issues.is_empty(), "all entries present => no issues");
}

#[test]
fn gitignore_entries_warns_on_each_missing_canonical_line() {
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    let tmp = TempDir::new().unwrap();
    // Write a gitignore that only includes `.ito/changes` and `.ito/specs`
    // — the other three canonical entries are missing.
    std::fs::write(tmp.path().join(".gitignore"), ".ito/changes\n.ito/specs\n").unwrap();

    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = GitignoreEntriesRule.check(&ctx).unwrap();
    assert_eq!(
        issues.len(),
        3,
        "three entries missing => three warnings, got {issues:?}"
    );
    for issue in &issues {
        assert_eq!(issue.level, "WARNING");
        assert_eq!(
            issue.rule_id.as_deref(),
            Some(GITIGNORE_ENTRIES_ID.as_str()),
        );
    }
}

#[test]
fn gitignore_entries_warns_when_gitignore_missing_entirely() {
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    let tmp = TempDir::new().unwrap();
    // No .gitignore at all.

    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = GitignoreEntriesRule.check(&ctx).unwrap();
    assert_eq!(
        issues.len(),
        gitignore_entries().len(),
        "missing gitignore => one warning per canonical entry, got {issues:?}",
    );
}

// ── coordination/staged-symlinked-paths ──────────────────────────────

#[test]
fn staged_symlinked_paths_passes_when_no_staged_files() {
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    let tmp = TempDir::new().unwrap();
    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = StagedSymlinkedPathsRule.check(&ctx).unwrap();
    assert!(issues.is_empty());
}

#[test]
fn staged_symlinked_paths_passes_when_staged_paths_outside_coordination_dirs() {
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    let tmp = TempDir::new().unwrap();
    let staged = StagedFiles::from_paths(vec![
        PathBuf::from("README.md"),
        PathBuf::from("ito-rs/src/lib.rs"),
        PathBuf::from(".ito/config.json"),
    ]);
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = StagedSymlinkedPathsRule.check(&ctx).unwrap();
    assert!(
        issues.is_empty(),
        "non-coordination paths pass; got {issues:?}"
    );
}

#[test]
fn staged_symlinked_paths_fails_for_each_path_under_coordination_dir() {
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    let tmp = TempDir::new().unwrap();
    // Cover every coordination directory plus an unrelated file so
    // that adding a new entry to COORDINATION_DIRS without updating
    // the rule logic would be visible here.
    let staged = StagedFiles::from_paths(vec![
        PathBuf::from(".ito/changes/011-05_foo/proposal.md"),
        PathBuf::from(".ito/specs/foo/spec.md"),
        PathBuf::from(".ito/modules/011/module.md"),
        PathBuf::from(".ito/workflows/spec-driven.md"),
        PathBuf::from(".ito/audit/local.jsonl"),
        PathBuf::from("README.md"),
        PathBuf::from(".ito/config.json"),
        PathBuf::from(".ito-extra/file.txt"),
    ]);
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = StagedSymlinkedPathsRule.check(&ctx).unwrap();
    assert_eq!(
        issues.len(),
        5,
        "five coordination paths => five errors, got {issues:?}",
    );
    for issue in &issues {
        assert_eq!(issue.level, "ERROR");
        assert_eq!(
            issue.rule_id.as_deref(),
            Some(STAGED_SYMLINKED_PATHS_ID.as_str()),
        );
    }

    // Sanity: ensure each coordination dir surfaces in some issue path.
    let paths: Vec<&str> = issues.iter().map(|i| i.path.as_str()).collect();
    for dir in &["changes", "specs", "modules", "workflows", "audit"] {
        assert!(
            paths.iter().any(|p| p.contains(dir)),
            "no issue path contains `{dir}`; paths: {paths:?}",
        );
    }
}

#[test]
fn staged_symlinked_paths_skips_dot_ito_itself() {
    // Staging `.ito` as a single path is not a coordination-dir
    // violation — only sub-paths under `.ito/<canonical-dir>/...`
    // qualify.
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    let tmp = TempDir::new().unwrap();
    let staged = StagedFiles::from_paths(vec![PathBuf::from(".ito")]);
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = StagedSymlinkedPathsRule.check(&ctx).unwrap();
    assert!(issues.is_empty());
}

// ── coordination/symlinks-wired ──────────────────────────────────────

#[test]
fn symlinks_wired_emits_resolution_error_when_no_remote_or_backend_project() {
    // Default config has worktree storage but no `origin` remote and
    // no `backend.project.{org,repo}` set. With
    // `allow_local_fallback=false`, the rule must surface a single,
    // targeted error rather than emitting a confusing
    // "WorktreeMissing" against a phantom path.
    let cfg = config_with_storage(CoordinationStorage::Worktree);
    let tmp = TempDir::new().unwrap();
    // Create `.ito/` so `resolved_coordination_worktree_path` proceeds
    // past any short-circuit on its absence.
    std::fs::create_dir_all(tmp.path().join(".ito")).unwrap();

    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SymlinksWiredRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1, "expected one error, got {issues:?}");
    let only = &issues[0];
    assert_eq!(only.level, "ERROR");
    assert_eq!(only.rule_id.as_deref(), Some(SYMLINKS_WIRED_ID.as_str()));
    assert!(
        only.message
            .contains("Cannot resolve the coordination worktree path"),
        "expected resolution-failure message; got: {}",
        only.message,
    );
    assert!(
        only.message.contains("Why:"),
        "expected What/Why/Fix style message; got: {}",
        only.message,
    );
}

#[test]
fn symlinks_wired_message_includes_why_clause_when_health_check_fails() {
    // Configure with an explicit worktree_path so resolution succeeds;
    // then point it at a non-existent directory. `check_coordination_health`
    // returns `WorktreeMissing` and our wrapper enriches the message
    // with a Why: prefix.
    let mut cfg = config_with_storage(CoordinationStorage::Worktree);
    cfg.changes.coordination_branch.worktree_path =
        Some("/nonexistent/coordination/worktree".to_string());

    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join(".ito")).unwrap();

    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = SymlinksWiredRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert!(
        issues[0].message.contains("Why:"),
        "wrapper should add Why: clause; got: {}",
        issues[0].message,
    );
    assert!(
        issues[0].message.contains("symlink"),
        "wrapper should mention symlink wiring; got: {}",
        issues[0].message,
    );
}

// ── coordination/branch-name-set ─────────────────────────────────────

#[test]
fn branch_name_set_passes_for_canonical_name() {
    let mut cfg = config_with_storage(CoordinationStorage::Worktree);
    cfg.changes.coordination_branch.name = "ito/internal/changes".to_string();

    let tmp = TempDir::new().unwrap();
    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = BranchNameSetRule.check(&ctx).unwrap();
    assert!(issues.is_empty());
}

#[test]
fn branch_name_set_warns_on_empty_name() {
    let mut cfg = config_with_storage(CoordinationStorage::Worktree);
    cfg.changes.coordination_branch.name = String::new();

    let tmp = TempDir::new().unwrap();
    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = BranchNameSetRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].level, "WARNING");
    assert!(issues[0].message.contains("empty"));
}

#[test]
fn branch_name_set_warns_on_non_conventional_name() {
    let mut cfg = config_with_storage(CoordinationStorage::Worktree);
    cfg.changes.coordination_branch.name = "coordination/foo".to_string();

    let tmp = TempDir::new().unwrap();
    let staged = StagedFiles::empty();
    let runner = NoopRunner;
    let ctx = RuleContext::new(&cfg, tmp.path(), &staged, &runner);

    let issues = BranchNameSetRule.check(&ctx).unwrap();
    assert_eq!(issues.len(), 1);
    assert!(
        issues[0].message.contains("ito/internal/"),
        "warning should reference the convention; got: {}",
        issues[0].message,
    );
}
