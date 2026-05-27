//! Setup/update coverage classification for Ito config fields.
//!
//! This module is deliberately small and explicit. The table is an audit
//! surface for `ito init` / `ito update`: when new config areas are added,
//! tests fail until the new path is classified as setup-managed,
//! update-refreshable, runtime-only, or intentionally excluded.

/// How a config path participates in project setup and refresh flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSetupCoverage {
    /// Managed by `ito init` setup prompts or flags.
    InitManaged,
    /// Safe for `ito update` to refresh through an explicit flag.
    UpdateRefreshable,
    /// Runtime behavior config, not part of setup/update UX.
    RuntimeOnly,
    /// Deliberately outside setup/update handling.
    IntentionallyExcluded,
}

/// A coverage entry for a config path or subtree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigSetupCoverageEntry {
    /// Dot-separated config path. Entries classify the path and its subtree.
    pub path: &'static str,
    /// Coverage classification for the path.
    pub coverage: ConfigSetupCoverage,
    /// Short explanation for why this path has this classification.
    pub reason: &'static str,
}

/// Explicit config setup/update coverage table.
pub const CONFIG_SETUP_COVERAGE: &[ConfigSetupCoverageEntry] = &[
    ConfigSetupCoverageEntry {
        path: "$schema",
        coverage: ConfigSetupCoverage::IntentionallyExcluded,
        reason: "editor metadata, not user setup behavior",
    },
    ConfigSetupCoverageEntry {
        path: "projectPath",
        coverage: ConfigSetupCoverage::InitManaged,
        reason: "project Ito directory selection belongs to first-run setup",
    },
    ConfigSetupCoverageEntry {
        path: "harnesses",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "agent model/provider tuning is edited directly or by harness-specific workflows",
    },
    ConfigSetupCoverageEntry {
        path: "cache",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "cache TTL is runtime behavior, not project setup",
    },
    ConfigSetupCoverageEntry {
        path: "defaults",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "workflow defaults are consumed by instructions and should not be reset by setup",
    },
    ConfigSetupCoverageEntry {
        path: "worktrees",
        coverage: ConfigSetupCoverage::InitManaged,
        reason: "worktree topology is selected during project setup and rendered into instructions",
    },
    ConfigSetupCoverageEntry {
        path: "tools",
        coverage: ConfigSetupCoverage::InitManaged,
        reason: "tool preferences exposed by setup inherit init-managed coverage unless narrowed",
    },
    ConfigSetupCoverageEntry {
        path: "tools.tmux.enabled",
        coverage: ConfigSetupCoverage::InitManaged,
        reason: "tmux preference is exposed through init flags/prompts",
    },
    ConfigSetupCoverageEntry {
        path: "changes",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "change coordination settings are operational storage behavior",
    },
    ConfigSetupCoverageEntry {
        path: "logging",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "diagnostic logging is runtime behavior",
    },
    ConfigSetupCoverageEntry {
        path: "audit",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "audit mirroring is operational behavior",
    },
    ConfigSetupCoverageEntry {
        path: "repository",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "repository persistence selection is an operational backend concern",
    },
    ConfigSetupCoverageEntry {
        path: "backend",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "backend client settings are runtime connectivity",
    },
    ConfigSetupCoverageEntry {
        path: "backendServer",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "backend server hosting settings are runtime service configuration",
    },
    ConfigSetupCoverageEntry {
        path: "memory",
        coverage: ConfigSetupCoverage::RuntimeOnly,
        reason: "agent memory providers are consumed by instruction rendering at runtime",
    },
];

/// Return the most specific coverage entry for a config path.
pub fn classify_config_path(path: &str) -> Option<&'static ConfigSetupCoverageEntry> {
    CONFIG_SETUP_COVERAGE
        .iter()
        .filter(|entry| {
            path == entry.path
                || path.starts_with(entry.path) && path[entry.path.len()..].starts_with('.')
        })
        .max_by_key(|entry| entry.path.len())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde_json::{Map, Value};

    use super::*;
    use crate::config::schema::config_schema_json;

    #[test]
    fn config_coverage_classifies_representative_setup_and_runtime_fields() {
        assert_eq!(
            classify_config_path("tools.tmux.enabled").map(|entry| entry.coverage),
            Some(ConfigSetupCoverage::InitManaged)
        );
        assert_eq!(
            classify_config_path("worktrees.strategy").map(|entry| entry.coverage),
            Some(ConfigSetupCoverage::InitManaged)
        );
        assert_eq!(
            classify_config_path("backend.url").map(|entry| entry.coverage),
            Some(ConfigSetupCoverage::RuntimeOnly)
        );
        assert_eq!(
            classify_config_path("$schema").map(|entry| entry.coverage),
            Some(ConfigSetupCoverage::IntentionallyExcluded)
        );
    }

    #[test]
    fn config_coverage_covers_all_schema_paths() {
        let schema = config_schema_json();
        let mut paths = BTreeSet::new();
        collect_property_paths(&schema, &schema, "", &mut paths);

        let missing: Vec<_> = paths
            .iter()
            .filter(|path| classify_config_path(path).is_none())
            .cloned()
            .collect();

        assert!(
            missing.is_empty(),
            "config paths missing setup/update coverage classification: {missing:#?}"
        );
    }

    fn collect_property_paths(
        root: &Value,
        schema: &Value,
        prefix: &str,
        paths: &mut BTreeSet<String>,
    ) {
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            if let Some(resolved) = resolve_local_ref(root, reference) {
                collect_property_paths(root, resolved, prefix, paths);
            }
            return;
        }

        for key in ["allOf", "anyOf", "oneOf"] {
            if let Some(items) = schema.get(key).and_then(Value::as_array) {
                for item in items {
                    collect_property_paths(root, item, prefix, paths);
                }
            }
        }

        let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
            return;
        };

        for (name, property_schema) in properties {
            let path = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{prefix}.{name}")
            };
            paths.insert(path.clone());
            collect_property_paths(root, property_schema, &path, paths);
        }
    }

    fn resolve_local_ref<'a>(root: &'a Value, reference: &str) -> Option<&'a Value> {
        let path = reference.strip_prefix("#/")?;
        let mut current = root;
        for segment in path.split('/') {
            let object: &Map<String, Value> = current.as_object()?;
            current = object.get(segment)?;
        }
        Some(current)
    }
}
