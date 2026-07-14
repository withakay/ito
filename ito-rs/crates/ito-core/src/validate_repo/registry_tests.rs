use super::*;
use crate::errors::CoreError;
use crate::validate::ValidationIssue;
use crate::validate_repo::rule::RuleContext;

/// Minimal stub rule used to exercise the registry without depending on
/// any of the real rule modules (which land in Wave 2).
struct StubRule {
    id: RuleId,
    severity: RuleSeverity,
    active: bool,
    description: &'static str,
    gate: Option<&'static str>,
}

impl Rule for StubRule {
    fn id(&self) -> RuleId {
        self.id
    }
    fn severity(&self) -> RuleSeverity {
        self.severity
    }
    fn description(&self) -> &'static str {
        self.description
    }
    fn gate(&self) -> Option<&'static str> {
        self.gate
    }
    fn is_active(&self, _config: &ItoConfig) -> bool {
        self.active
    }
    fn check(&self, _ctx: &RuleContext<'_>) -> Result<Vec<ValidationIssue>, CoreError> {
        Ok(Vec::new())
    }
}

fn stub(id: &'static str, active: bool) -> Box<dyn Rule> {
    Box::new(StubRule {
        id: RuleId::new(id),
        severity: RuleSeverity::Warning,
        active,
        description: "always available",
        gate: None,
    })
}

fn gated_stub(id: &'static str, active: bool, gate: &'static str) -> Box<dyn Rule> {
    Box::new(StubRule {
        id: RuleId::new(id),
        severity: RuleSeverity::Error,
        active,
        description: "gated",
        gate: Some(gate),
    })
}

#[test]
fn empty_registry_has_no_rules() {
    let registry = RuleRegistry::empty();
    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
    assert!(registry.iter().next().is_none());
}

#[test]
fn built_in_registry_contains_every_built_in_rule() {
    let registry = RuleRegistry::built_in();
    // Eight ordinary diagnostics ship in every build. Five coordination
    // diagnostics join them only when the experimental feature is compiled.
    // - 011-05: coordination/{symlinks-wired,gitignore-entries,
    //   staged-symlinked-paths,branch-name-set} + worktrees/{
    //   no-write-on-control,layout-consistent} = 6.
    // - 011-06: audit/{mirror-branch-set,
    //   mirror-branch-distinct-from-coordination} +
    //   repository/{sqlite-db-path-set,sqlite-db-not-committed} +
    //   backend/{token-not-committed,url-scheme-valid,
    //   project-org-repo-set} = 7.
    let ids: Vec<_> = registry.iter().map(|r| r.id().as_str()).collect();
    let expected = vec![
        "audit/mirror-branch-set",
        "backend/project-org-repo-set",
        "backend/token-not-committed",
        "backend/url-scheme-valid",
        "repository/sqlite-db-not-committed",
        "repository/sqlite-db-path-set",
        "worktrees/layout-consistent",
        "worktrees/no-write-on-control",
    ];
    #[cfg(feature = "coordination-branch")]
    let expected = {
        let mut expected = expected;
        expected.extend([
            "audit/mirror-branch-distinct-from-coordination",
            "coordination/branch-name-set",
            "coordination/gitignore-entries",
            "coordination/staged-symlinked-paths",
            "coordination/symlinks-wired",
        ]);
        expected
    };

    assert_eq!(
        ids.len(),
        expected.len(),
        "unexpected built-in rules: {ids:?}"
    );
    for expected in expected {
        assert!(
            ids.contains(&expected),
            "built-in registry missing `{expected}`; have: {ids:?}",
        );
    }
}

#[test]
fn list_active_rules_for_empty_registry_returns_empty() {
    let config = ItoConfig::default();
    assert!(list_active_rules_for(&RuleRegistry::empty(), &config).is_empty());
}

#[test]
fn list_active_rules_for_single_active_rule_reports_active_true() {
    let config = ItoConfig::default();
    let registry = RuleRegistry::empty().with_rule(stub("test/always", true));

    let rules = list_active_rules_for(&registry, &config);
    assert_eq!(rules.len(), 1);
    let only = &rules[0];
    assert_eq!(only.rule_id.as_str(), "test/always");
    assert_eq!(only.severity, RuleSeverity::Warning);
    assert!(only.active);
    assert_eq!(only.description, "always available");
    assert_eq!(only.gate, None);
}

#[test]
fn list_active_rules_for_inactive_rule_reports_active_false() {
    let config = ItoConfig::default();
    let registry = RuleRegistry::empty().with_rule(stub("test/never", false));

    let rules = list_active_rules_for(&registry, &config);
    assert_eq!(rules.len(), 1);
    assert!(!rules[0].active);
}

#[test]
fn list_active_rules_for_returns_rules_sorted_by_id() {
    let config = ItoConfig::default();
    let registry = RuleRegistry::empty()
        .with_rule(stub("zeta/last", true))
        .with_rule(stub("alpha/first", false))
        .with_rule(stub("mu/middle", true));

    let ids: Vec<_> = list_active_rules_for(&registry, &config)
        .into_iter()
        .map(|r| r.rule_id.as_str())
        .collect();

    assert_eq!(ids, vec!["alpha/first", "mu/middle", "zeta/last"]);
}

#[test]
fn list_active_rules_for_surfaces_gate_metadata() {
    let config = ItoConfig::default();
    let registry = RuleRegistry::empty().with_rule(gated_stub(
        "coordination/example",
        true,
        "changes.coordination_branch.storage == worktree",
    ));

    let rules = list_active_rules_for(&registry, &config);
    assert_eq!(
        rules[0].gate,
        Some("changes.coordination_branch.storage == worktree"),
        "gate metadata should be surfaced verbatim",
    );
}

/// Activation matrix for the full built-in rule set.
///
/// Each row is a config permutation and the rule ids that should be
/// `active = true` for it. Rules not listed are expected to be
/// inactive. This test catches accidental gate changes by enforcing
/// the entire matrix at once rather than one rule at a time.
#[cfg(feature = "coordination-branch")]
#[test]
fn list_active_rules_matrix_matches_specification() {
    use ito_config::types::{
        AuditConfig, AuditMirrorConfig, BackendApiConfig, BackendProjectConfig, ChangesConfig,
        CoordinationBranchConfig, CoordinationStorage, RepositoryPersistenceMode,
        RepositoryRuntimeConfig, RepositorySqliteConfig, WorktreesConfig,
    };

    struct Case {
        label: &'static str,
        mutate: fn(&mut ItoConfig),
        expected_active: &'static [&'static str],
    }

    // Always-active rule applies to every row.
    const ALWAYS: &[&str] = &["coordination/branch-name-set"];

    let cases = [
        Case {
            label: "minimal: embedded coord, worktrees off, fs repo, backend/audit off",
            mutate: |c| {
                c.changes.coordination_branch.storage = CoordinationStorage::Embedded;
                c.worktrees.enabled = false;
                c.repository.mode = RepositoryPersistenceMode::Filesystem;
                c.audit.mirror.enabled = false;
                c.backend.enabled = false;
            },
            expected_active: ALWAYS,
        },
        Case {
            label: "coordination_worktree only",
            mutate: |c| {
                c.changes.coordination_branch.storage = CoordinationStorage::Worktree;
                c.worktrees.enabled = false;
                c.repository.mode = RepositoryPersistenceMode::Filesystem;
                c.audit.mirror.enabled = false;
                c.backend.enabled = false;
            },
            expected_active: &[
                "coordination/branch-name-set",
                "coordination/gitignore-entries",
                "coordination/staged-symlinked-paths",
                "coordination/symlinks-wired",
            ],
        },
        Case {
            label: "worktrees enabled only",
            mutate: |c| {
                c.changes.coordination_branch.storage = CoordinationStorage::Embedded;
                c.worktrees.enabled = true;
                c.repository.mode = RepositoryPersistenceMode::Filesystem;
                c.audit.mirror.enabled = false;
                c.backend.enabled = false;
            },
            expected_active: &[
                "coordination/branch-name-set",
                "worktrees/layout-consistent",
                "worktrees/no-write-on-control",
            ],
        },
        Case {
            label: "audit mirror enabled, embedded coord (distinct rule still skipped)",
            mutate: |c| {
                c.changes.coordination_branch.storage = CoordinationStorage::Embedded;
                c.worktrees.enabled = false;
                c.repository.mode = RepositoryPersistenceMode::Filesystem;
                c.audit.mirror.enabled = true;
                c.backend.enabled = false;
            },
            expected_active: &["audit/mirror-branch-set", "coordination/branch-name-set"],
        },
        Case {
            label: "audit mirror + worktree coord (both audit rules active)",
            mutate: |c| {
                c.changes.coordination_branch.storage = CoordinationStorage::Worktree;
                c.worktrees.enabled = false;
                c.repository.mode = RepositoryPersistenceMode::Filesystem;
                c.audit.mirror.enabled = true;
                c.backend.enabled = false;
            },
            expected_active: &[
                "audit/mirror-branch-distinct-from-coordination",
                "audit/mirror-branch-set",
                "coordination/branch-name-set",
                "coordination/gitignore-entries",
                "coordination/staged-symlinked-paths",
                "coordination/symlinks-wired",
            ],
        },
        Case {
            label: "sqlite repo only",
            mutate: |c| {
                c.changes.coordination_branch.storage = CoordinationStorage::Embedded;
                c.worktrees.enabled = false;
                c.repository.mode = RepositoryPersistenceMode::Sqlite;
                c.audit.mirror.enabled = false;
                c.backend.enabled = false;
            },
            expected_active: &[
                "coordination/branch-name-set",
                "repository/sqlite-db-not-committed",
                "repository/sqlite-db-path-set",
            ],
        },
        Case {
            label: "backend enabled only",
            mutate: |c| {
                c.changes.coordination_branch.storage = CoordinationStorage::Embedded;
                c.worktrees.enabled = false;
                c.repository.mode = RepositoryPersistenceMode::Filesystem;
                c.audit.mirror.enabled = false;
                c.backend.enabled = true;
            },
            expected_active: &[
                "backend/project-org-repo-set",
                "backend/token-not-committed",
                "backend/url-scheme-valid",
                "coordination/branch-name-set",
            ],
        },
        Case {
            label: "everything on (all 13 rules active)",
            mutate: |c| {
                c.changes.coordination_branch.storage = CoordinationStorage::Worktree;
                c.worktrees.enabled = true;
                c.repository.mode = RepositoryPersistenceMode::Sqlite;
                c.audit.mirror.enabled = true;
                c.backend.enabled = true;
            },
            expected_active: &[
                "audit/mirror-branch-distinct-from-coordination",
                "audit/mirror-branch-set",
                "backend/project-org-repo-set",
                "backend/token-not-committed",
                "backend/url-scheme-valid",
                "coordination/branch-name-set",
                "coordination/gitignore-entries",
                "coordination/staged-symlinked-paths",
                "coordination/symlinks-wired",
                "repository/sqlite-db-not-committed",
                "repository/sqlite-db-path-set",
                "worktrees/layout-consistent",
                "worktrees/no-write-on-control",
            ],
        },
    ];

    // Suppress unused warnings when the matrix references types that
    // some compilation units may not exercise.
    let _ = (
        AuditConfig::default(),
        AuditMirrorConfig::default(),
        BackendApiConfig::default(),
        BackendProjectConfig::default(),
        ChangesConfig::default(),
        CoordinationBranchConfig::default(),
        RepositoryRuntimeConfig::default(),
        RepositorySqliteConfig::default(),
        WorktreesConfig::default(),
    );

    for case in &cases {
        let mut cfg = ItoConfig::default();
        (case.mutate)(&mut cfg);

        let active_ids: Vec<_> = list_active_rules(&cfg)
            .into_iter()
            .filter(|r| r.active)
            .map(|r| r.rule_id.as_str())
            .collect();

        let expected: Vec<&str> = case.expected_active.to_vec();
        assert_eq!(
            active_ids,
            expected,
            "case `{label}`: active set mismatch",
            label = case.label,
        );
    }
}

#[test]
fn public_list_active_rules_delegates_to_built_in_registry() {
    let config = ItoConfig::default();
    // After Wave 2 the built-in registry is non-empty and rules are
    // sorted by id.
    let rules = list_active_rules(&config);
    assert!(!rules.is_empty(), "built-in registry should be non-empty");

    // Rules are sorted lexicographically by RuleId.
    let mut sorted_ids: Vec<_> = rules.iter().map(|r| r.rule_id.as_str()).collect();
    let original = sorted_ids.clone();
    sorted_ids.sort();
    assert_eq!(
        original, sorted_ids,
        "list_active_rules must return sorted output"
    );
}
