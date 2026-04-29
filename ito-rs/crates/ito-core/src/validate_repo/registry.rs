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
    /// Wave 1 returns an empty registry; later waves push their rules in.
    #[must_use]
    pub fn built_in() -> Self {
        // Rules are registered here in subsequent waves. Order does not
        // matter — `list_active_rules` sorts by `RuleId` for deterministic
        // output.
        Self::empty()
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
mod tests {
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
    fn built_in_registry_compiles_against_empty_rule_list() {
        let registry = RuleRegistry::built_in();
        // Wave 1 invariant: built-in registry is empty until Wave 2 lands.
        // Subsequent tasks update this assertion as they register rules.
        assert!(registry.is_empty());
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

    #[test]
    fn public_list_active_rules_delegates_to_built_in_registry() {
        let config = ItoConfig::default();
        // Built-in registry is empty until Wave 2; this test pins the
        // current behaviour and serves as a canary when rules are added.
        assert!(list_active_rules(&config).is_empty());
    }
}
