//! JSON configuration file CRUD operations.
//!
//! This module provides low-level functions for reading, writing, and
//! manipulating JSON configuration files with dot-delimited path navigation.

use std::path::Path;

use crate::errors::{CoreError, CoreResult};
use ito_config::ConfigContext;
use ito_config::load_cascading_project_config;
use ito_config::types::{IntegrationMode, WorktreeStrategy};

/// Read a JSON config file, returning an empty object if the file doesn't exist.
///
/// # Errors
///
/// Returns [`CoreError::Serde`] if the file contains invalid JSON or is not a JSON object.
pub fn read_json_config(path: &Path) -> CoreResult<serde_json::Value> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return Ok(serde_json::Value::Object(serde_json::Map::new()));
    };
    let v: serde_json::Value = serde_json::from_str(&contents).map_err(|e| {
        CoreError::serde(format!("Invalid JSON in {}", path.display()), e.to_string())
    })?;
    match v {
        serde_json::Value::Object(_) => Ok(v),
        _ => Err(CoreError::serde(
            format!("Expected JSON object in {}", path.display()),
            "root value is not an object",
        )),
    }
}

/// Write a JSON value to a config file (pretty-printed with trailing newline).
///
/// # Errors
///
/// Returns [`CoreError::Serde`] if serialization fails, or [`CoreError::Io`] if writing fails.
pub fn write_json_config(path: &Path, value: &serde_json::Value) -> CoreResult<()> {
    let mut bytes = serde_json::to_vec_pretty(value)
        .map_err(|e| CoreError::serde("Failed to serialize JSON config", e.to_string()))?;
    bytes.push(b'\n');
    ito_common::io::write_atomic_std(path, bytes)
        .map_err(|e| CoreError::io(format!("Failed to write config to {}", path.display()), e))?;
    Ok(())
}

/// Parse a CLI argument as a JSON value, falling back to a string if parsing fails.
///
/// If `force_string` is true, always returns a JSON string without attempting to parse.
pub fn parse_json_value_arg(raw: &str, force_string: bool) -> serde_json::Value {
    if force_string {
        return serde_json::Value::String(raw.to_string());
    }
    match serde_json::from_str::<serde_json::Value>(raw) {
        Ok(v) => v,
        Err(_) => serde_json::Value::String(raw.to_string()),
    }
}

/// Split a dot-delimited config key path into parts, trimming and filtering empty segments.
pub fn json_split_path(key: &str) -> Vec<&str> {
    let mut out: Vec<&str> = Vec::new();
    for part in key.split('.') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        out.push(part);
    }
    out
}

/// Navigate a JSON object by a slice of path parts, returning the value if found.
pub fn json_get_path<'a>(
    root: &'a serde_json::Value,
    parts: &[&str],
) -> Option<&'a serde_json::Value> {
    let mut cur = root;
    for p in parts {
        let serde_json::Value::Object(map) = cur else {
            return None;
        };
        let next = map.get(*p)?;
        cur = next;
    }
    Some(cur)
}

/// Set a value at a dot-delimited path in a JSON object, creating intermediate objects as needed.
///
/// # Errors
///
/// Returns [`CoreError::Validation`] if the path is empty or if setting the path fails.
#[allow(clippy::match_like_matches_macro)]
pub fn json_set_path(
    root: &mut serde_json::Value,
    parts: &[&str],
    value: serde_json::Value,
) -> CoreResult<()> {
    if parts.is_empty() {
        return Err(CoreError::validation("Invalid empty path"));
    }

    let mut cur = root;
    for (i, key) in parts.iter().enumerate() {
        let is_last = i + 1 == parts.len();

        let is_object = match cur {
            serde_json::Value::Object(_) => true,
            _ => false,
        };
        if !is_object {
            *cur = serde_json::Value::Object(serde_json::Map::new());
        }

        let serde_json::Value::Object(map) = cur else {
            return Err(CoreError::validation("Failed to set path"));
        };

        if is_last {
            map.insert((*key).to_string(), value);
            return Ok(());
        }

        let needs_object = match map.get(*key) {
            Some(serde_json::Value::Object(_)) => false,
            Some(_) => true,
            None => true,
        };
        if needs_object {
            map.insert(
                (*key).to_string(),
                serde_json::Value::Object(serde_json::Map::new()),
            );
        }

        let Some(next) = map.get_mut(*key) else {
            return Err(CoreError::validation("Failed to set path"));
        };
        cur = next;
    }

    Ok(())
}

/// Validate a config value for known keys that require enum values.
///
/// Returns `Ok(())` if the key is not constrained or the value is valid.
/// Returns `Err` with a descriptive message if the value is invalid.
///
/// # Errors
///
/// Returns [`CoreError::Validation`] if the value does not match the allowed enum values.
pub fn validate_config_value(parts: &[&str], value: &serde_json::Value) -> CoreResult<()> {
    let path = parts.join(".");
    match path.as_str() {
        "worktrees.strategy" => {
            let Some(s) = value.as_str() else {
                return Err(CoreError::validation(format!(
                    "Key '{}' requires a string value. Valid values: {}",
                    path,
                    WorktreeStrategy::ALL.join(", ")
                )));
            };
            if WorktreeStrategy::parse_value(s).is_none() {
                return Err(CoreError::validation(format!(
                    "Invalid value '{}' for key '{}'. Valid values: {}",
                    s,
                    path,
                    WorktreeStrategy::ALL.join(", ")
                )));
            }
        }
        "worktrees.apply.integration_mode" => {
            let Some(s) = value.as_str() else {
                return Err(CoreError::validation(format!(
                    "Key '{}' requires a string value. Valid values: {}",
                    path,
                    IntegrationMode::ALL.join(", ")
                )));
            };
            if IntegrationMode::parse_value(s).is_none() {
                return Err(CoreError::validation(format!(
                    "Invalid value '{}' for key '{}'. Valid values: {}",
                    s,
                    path,
                    IntegrationMode::ALL.join(", ")
                )));
            }
        }
        "changes.coordination_branch.name" => {
            let Some(s) = value.as_str() else {
                return Err(CoreError::validation(format!(
                    "Key '{}' requires a string value.",
                    path,
                )));
            };
            if !is_valid_branch_name(s) {
                return Err(CoreError::validation(format!(
                    "Invalid value '{}' for key '{}'. Provide a valid git branch name.",
                    s, path,
                )));
            }
        }
        _ => {}
    }
    Ok(())
}

fn is_valid_branch_name(value: &str) -> bool {
    if value.is_empty() || value.starts_with('-') || value.starts_with('/') || value.ends_with('/')
    {
        return false;
    }
    if value.contains("..")
        || value.contains("@{")
        || value.contains("//")
        || value.ends_with('.')
        || value.ends_with(".lock")
    {
        return false;
    }

    for ch in value.chars() {
        if ch.is_ascii_control() || ch == ' ' {
            return false;
        }

        match ch {
            '~' | '^' | ':' | '?' | '*' | '[' | '\\' => return false,
            _ => {}
        }
    }

    for segment in value.split('/') {
        if segment.is_empty()
            || segment.starts_with('.')
            || segment.ends_with('.')
            || segment.ends_with(".lock")
        {
            return false;
        }
    }

    true
}

/// Validate that a worktree strategy string is one of the supported values.
///
/// Returns `true` if valid, `false` otherwise.
pub fn is_valid_worktree_strategy(s: &str) -> bool {
    WorktreeStrategy::parse_value(s).is_some()
}

/// Validate that an integration mode string is one of the supported values.
///
/// Returns `true` if valid, `false` otherwise.
pub fn is_valid_integration_mode(s: &str) -> bool {
    IntegrationMode::parse_value(s).is_some()
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Resolved defaults used when rendering worktree-aware templates.
pub struct WorktreeTemplateDefaults {
    /// Worktree strategy (e.g., `checkout_subdir`).
    pub strategy: String,
    /// Directory name used by the strategy layout.
    pub layout_dir_name: String,
    /// Integration mode for applying changes.
    pub integration_mode: String,
    /// Default branch name.
    pub default_branch: String,
}

/// Resolve effective worktree defaults from cascading project configuration.
///
/// Falls back to built-in defaults when keys are not configured.
pub fn resolve_worktree_template_defaults(
    target_path: &Path,
    ctx: &ConfigContext,
) -> WorktreeTemplateDefaults {
    let ito_path = ito_config::ito_dir::get_ito_path(target_path, ctx);
    let merged = load_cascading_project_config(target_path, &ito_path, ctx).merged;

    let mut defaults = WorktreeTemplateDefaults {
        strategy: "checkout_subdir".to_string(),
        layout_dir_name: "ito-worktrees".to_string(),
        integration_mode: "commit_pr".to_string(),
        default_branch: "main".to_string(),
    };

    if let Some(wt) = merged.get("worktrees") {
        if let Some(v) = wt.get("strategy").and_then(|v| v.as_str())
            && !v.is_empty()
        {
            defaults.strategy = v.to_string();
        }

        if let Some(v) = wt.get("default_branch").and_then(|v| v.as_str())
            && !v.is_empty()
        {
            defaults.default_branch = v.to_string();
        }

        if let Some(layout) = wt.get("layout")
            && let Some(v) = layout.get("dir_name").and_then(|v| v.as_str())
            && !v.is_empty()
        {
            defaults.layout_dir_name = v.to_string();
        }

        if let Some(apply) = wt.get("apply")
            && let Some(v) = apply.get("integration_mode").and_then(|v| v.as_str())
            && !v.is_empty()
        {
            defaults.integration_mode = v.to_string();
        }
    }

    defaults
}

/// Remove a key at a dot-delimited path in a JSON object.
///
/// Returns `true` if a key was removed, `false` if the path didn't exist.
///
/// # Errors
///
/// Returns [`CoreError::Validation`] if the path is empty.
pub fn json_unset_path(root: &mut serde_json::Value, parts: &[&str]) -> CoreResult<bool> {
    if parts.is_empty() {
        return Err(CoreError::validation("Invalid empty path"));
    }

    let mut cur = root;
    for (i, p) in parts.iter().enumerate() {
        let is_last = i + 1 == parts.len();
        let serde_json::Value::Object(map) = cur else {
            return Ok(false);
        };

        if is_last {
            return Ok(map.remove(*p).is_some());
        }

        let Some(next) = map.get_mut(*p) else {
            return Ok(false);
        };
        cur = next;
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validate_config_value_accepts_valid_strategy() {
        let parts = ["worktrees", "strategy"];
        let value = json!("checkout_subdir");
        assert!(validate_config_value(&parts, &value).is_ok());

        let value = json!("checkout_siblings");
        assert!(validate_config_value(&parts, &value).is_ok());

        let value = json!("bare_control_siblings");
        assert!(validate_config_value(&parts, &value).is_ok());
    }

    #[test]
    fn validate_config_value_rejects_invalid_strategy() {
        let parts = ["worktrees", "strategy"];
        let value = json!("custom_layout");
        let err = validate_config_value(&parts, &value).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Invalid value"));
        assert!(msg.contains("custom_layout"));
    }

    #[test]
    fn validate_config_value_rejects_non_string_strategy() {
        let parts = ["worktrees", "strategy"];
        let value = json!(42);
        let err = validate_config_value(&parts, &value).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("requires a string value"));
    }

    #[test]
    fn validate_config_value_accepts_valid_integration_mode() {
        let parts = ["worktrees", "apply", "integration_mode"];
        let value = json!("commit_pr");
        assert!(validate_config_value(&parts, &value).is_ok());

        let value = json!("merge_parent");
        assert!(validate_config_value(&parts, &value).is_ok());
    }

    #[test]
    fn validate_config_value_rejects_invalid_integration_mode() {
        let parts = ["worktrees", "apply", "integration_mode"];
        let value = json!("squash_merge");
        let err = validate_config_value(&parts, &value).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Invalid value"));
        assert!(msg.contains("squash_merge"));
    }

    #[test]
    fn validate_config_value_accepts_unknown_keys() {
        let parts = ["worktrees", "enabled"];
        let value = json!(true);
        assert!(validate_config_value(&parts, &value).is_ok());

        let parts = ["some", "other", "key"];
        let value = json!("anything");
        assert!(validate_config_value(&parts, &value).is_ok());
    }

    #[test]
    fn is_valid_worktree_strategy_checks_correctly() {
        assert!(is_valid_worktree_strategy("checkout_subdir"));
        assert!(is_valid_worktree_strategy("checkout_siblings"));
        assert!(is_valid_worktree_strategy("bare_control_siblings"));
        assert!(!is_valid_worktree_strategy("custom"));
        assert!(!is_valid_worktree_strategy(""));
    }

    #[test]
    fn is_valid_integration_mode_checks_correctly() {
        assert!(is_valid_integration_mode("commit_pr"));
        assert!(is_valid_integration_mode("merge_parent"));
        assert!(!is_valid_integration_mode("squash"));
        assert!(!is_valid_integration_mode(""));
    }

    #[test]
    fn validate_config_value_accepts_valid_coordination_branch_name() {
        let parts = ["changes", "coordination_branch", "name"];
        let value = json!("ito/internal/changes");
        assert!(validate_config_value(&parts, &value).is_ok());
    }

    #[test]
    fn validate_config_value_rejects_invalid_coordination_branch_name() {
        let parts = ["changes", "coordination_branch", "name"];
        let value = json!("--ito-changes");
        let err = validate_config_value(&parts, &value).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Invalid value"));
        assert!(msg.contains("changes.coordination_branch.name"));
    }

    #[test]
    fn validate_config_value_rejects_lock_suffix_in_path_segment() {
        let parts = ["changes", "coordination_branch", "name"];
        let value = json!("foo.lock/bar");
        let err = validate_config_value(&parts, &value).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("Invalid value"));
        assert!(msg.contains("changes.coordination_branch.name"));
    }

    #[test]
    fn resolve_worktree_template_defaults_uses_defaults_when_missing() {
        let project = tempfile::tempdir().expect("tempdir should succeed");
        let ctx = ConfigContext {
            project_dir: Some(project.path().to_path_buf()),
            ..Default::default()
        };

        let resolved = resolve_worktree_template_defaults(project.path(), &ctx);
        assert_eq!(
            resolved,
            WorktreeTemplateDefaults {
                strategy: "checkout_subdir".to_string(),
                layout_dir_name: "ito-worktrees".to_string(),
                integration_mode: "commit_pr".to_string(),
                default_branch: "main".to_string(),
            }
        );
    }

    #[test]
    fn resolve_worktree_template_defaults_reads_overrides() {
        let project = tempfile::tempdir().expect("tempdir should succeed");
        let ito_dir = project.path().join(".ito");
        std::fs::create_dir_all(&ito_dir).expect("create .ito should succeed");
        std::fs::write(
            ito_dir.join("config.json"),
            r#"{
  "worktrees": {
    "strategy": "bare_control_siblings",
    "default_branch": "develop",
    "layout": { "dir_name": "wt" },
    "apply": { "integration_mode": "merge_parent" }
  }
}
"#,
        )
        .expect("write config should succeed");

        let ctx = ConfigContext {
            project_dir: Some(project.path().to_path_buf()),
            ..Default::default()
        };

        let resolved = resolve_worktree_template_defaults(project.path(), &ctx);
        assert_eq!(
            resolved,
            WorktreeTemplateDefaults {
                strategy: "bare_control_siblings".to_string(),
                layout_dir_name: "wt".to_string(),
                integration_mode: "merge_parent".to_string(),
                default_branch: "develop".to_string(),
            }
        );
    }
}
