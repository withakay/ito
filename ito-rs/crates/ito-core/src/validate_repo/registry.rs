//! Rule registry holding the built-in rule set.
//!
//! The registry is stateless and cheap to construct; callers typically build
//! one per `ito validate repo` invocation via [`RuleRegistry::built_in`].
//!
//! Wave 1 ships an empty built-in registry plus the introspection helper
//! [`list_active_rules`]. Subsequent waves register concrete rules:
//!
//! - Wave 2: `coordination/*`, `worktrees/*`, plus pre-commit detection.
//! - Change `011-06`: `audit/*`, `repository/*`, `backend/*`.

use ito_config::types::ItoConfig;

use super::rule::{Rule, RuleId, RuleSeverity};

/// Snapshot of a rule's activation state for introspection.
///
/// Returned by [`list_active_rules`] and by the
/// `ito validate repo --list-rules` CLI handler.
///
/// Marked `#[non_exhaustive]` so additional metadata fields can be added in
/// later waves without breaking external consumers.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ActiveRule {
    /// Stable rule identifier.
    pub rule_id: RuleId,
    /// Nominal severity declared by the rule.
    pub severity: RuleSeverity,
    /// Short human-readable description of what the rule checks.
    pub description: &'static str,
    /// Whether the rule is active for the resolved [`ItoConfig`].
    pub active: bool,
    /// Optional description of the activation gate
    /// (e.g. `"changes.coordination_branch.storage == worktree"`). `None`
    /// when the rule is unconditionally active.
    pub gate: Option<&'static str>,
}

/// Container for the set of built-in [`Rule`]s.
///
/// The registry owns trait objects so concrete rule structs are kept
/// crate-private; consumers interact only with [`Rule`] and
/// [`ActiveRule`].
#[derive(Default)]
pub struct RuleRegistry {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleRegistry {
    /// Construct an empty registry.
    ///
    /// Useful for tests; production code should call [`Self::built_in`].
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Construct a registry pre-populated with every built-in rule.
    ///
    /// Order of registration does not matter — [`list_active_rules`] sorts
    /// by `RuleId` for deterministic output.
    ///
    /// Built-in rules:
    ///
    /// - `coordination/*` and `worktrees/*` — change 011-05.
    /// - `audit/*`, `repository/*`, `backend/*` — change 011-06.
    #[must_use]
    pub fn built_in() -> Self {
        use super::audit_rules::{MirrorBranchDistinctRule, MirrorBranchSetRule};
        use super::backend_rules::{
            ProjectOrgRepoSetRule, TokenNotCommittedRule, UrlSchemeValidRule,
        };
        use super::coordination_rules::{
            BranchNameSetRule, GitignoreEntriesRule, StagedSymlinkedPathsRule, SymlinksWiredRule,
        };
        use super::repository_rules::{SqliteDbNotCommittedRule, SqliteDbPathSetRule};
        use super::worktrees_rules::{LayoutConsistentRule, NoWriteOnControlRule};

        Self::empty()
            // 011-05: coordination/*, worktrees/*
            .with_rule(Box::new(SymlinksWiredRule))
            .with_rule(Box::new(GitignoreEntriesRule))
            .with_rule(Box::new(StagedSymlinkedPathsRule))
            .with_rule(Box::new(BranchNameSetRule))
            .with_rule(Box::new(NoWriteOnControlRule))
            .with_rule(Box::new(LayoutConsistentRule))
            // 011-06: audit/*, repository/*, backend/*
            .with_rule(Box::new(MirrorBranchSetRule))
            .with_rule(Box::new(MirrorBranchDistinctRule))
            .with_rule(Box::new(SqliteDbPathSetRule))
            .with_rule(Box::new(SqliteDbNotCommittedRule))
            .with_rule(Box::new(TokenNotCommittedRule))
            .with_rule(Box::new(UrlSchemeValidRule))
            .with_rule(Box::new(ProjectOrgRepoSetRule))
    }

    /// Register a rule with this registry.
    ///
    /// Builder-style API used internally by [`Self::built_in`] and by tests.
    #[must_use]
    pub fn with_rule(mut self, rule: Box<dyn Rule>) -> Self {
        self.rules.push(rule);
        self
    }

    /// Iterate over registered rules in registration order.
    pub fn iter(&self) -> impl Iterator<Item = &dyn Rule> {
        self.rules.iter().map(Box::as_ref)
    }

    /// Number of registered rules.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// True if no rules are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

impl std::fmt::Debug for RuleRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuleRegistry")
            .field(
                "rules",
                &self
                    .rules
                    .iter()
                    .map(|r| r.id().as_str())
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

/// Return the set of registered rules with their activation state for the
/// given config, sorted lexicographically by [`RuleId`].
///
/// Equivalent to `list_active_rules_for(&RuleRegistry::built_in(), config)`.
#[must_use]
pub fn list_active_rules(config: &ItoConfig) -> Vec<ActiveRule> {
    list_active_rules_for(&RuleRegistry::built_in(), config)
}

/// Return the set of rules in `registry` with their activation state for
/// `config`, sorted lexicographically by [`RuleId`].
///
/// Exposed primarily for unit tests that construct ad-hoc registries; most
/// callers should use [`list_active_rules`].
#[must_use]
pub fn list_active_rules_for(registry: &RuleRegistry, config: &ItoConfig) -> Vec<ActiveRule> {
    let mut active: Vec<ActiveRule> = registry
        .iter()
        .map(|rule| ActiveRule {
            rule_id: rule.id(),
            severity: rule.severity(),
            description: rule.description(),
            active: rule.is_active(config),
            gate: rule.gate(),
        })
        .collect();
    active.sort_by_key(|item| item.rule_id);
    active
}

#[cfg(test)]
#[path = "registry_tests.rs"]
mod registry_tests;
