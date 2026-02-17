use super::*;
use std::path::Path;

fn read_json(path: &Path) -> serde_json::Value {
    let s = std::fs::read_to_string(path).expect("read config");
    serde_json::from_str(&s).expect("parse json")
}

#[test]
fn persist_worktree_config_writes_disabled_and_preserves_other_keys() {
    let td = tempfile::tempdir().expect("tempdir");
    let config_path = td.path().join(".ito/config.json");
    std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    std::fs::write(&config_path, r#"{"foo":1}"#).unwrap();

    let result = WorktreeWizardResult {
        ran: true,
        enabled: false,
        strategy: None,
        integration_mode: None,
    };
    persist_worktree_config(&config_path, &result).expect("persist");

    let v = read_json(&config_path);
    assert_eq!(v["foo"], 1);
    assert_eq!(v["worktrees"]["enabled"], false);
}

#[test]
fn persist_worktree_config_writes_enabled_settings() {
    let td = tempfile::tempdir().expect("tempdir");
    let config_path = td.path().join(".ito/config.json");
    std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    std::fs::write(&config_path, "{}\n").unwrap();

    let result = WorktreeWizardResult {
        ran: true,
        enabled: true,
        strategy: Some("checkout_subdir".to_string()),
        integration_mode: Some("commit_pr".to_string()),
    };
    persist_worktree_config(&config_path, &result).expect("persist");

    let v = read_json(&config_path);
    assert_eq!(v["worktrees"]["enabled"], true);
    assert_eq!(v["worktrees"]["strategy"], "checkout_subdir");
    assert_eq!(v["worktrees"]["apply"]["integration_mode"], "commit_pr");
}

#[test]
fn persist_worktree_config_errors_when_enabled_missing_fields() {
    let td = tempfile::tempdir().expect("tempdir");
    let config_path = td.path().join(".ito/config.json");
    std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    std::fs::write(&config_path, "{}\n").unwrap();

    let missing_strategy = WorktreeWizardResult {
        ran: true,
        enabled: true,
        strategy: None,
        integration_mode: Some("commit_pr".to_string()),
    };
    let err = persist_worktree_config(&config_path, &missing_strategy).unwrap_err();
    assert!(err.to_string().contains("missing strategy"));

    let missing_mode = WorktreeWizardResult {
        ran: true,
        enabled: true,
        strategy: Some("checkout_subdir".to_string()),
        integration_mode: None,
    };
    let err = persist_worktree_config(&config_path, &missing_mode).unwrap_err();
    assert!(err.to_string().contains("missing integration_mode"));
}

#[test]
fn is_worktree_configured_detects_strategy_key() {
    let td = tempfile::tempdir().expect("tempdir");
    let config_path = td.path().join(".ito/config.json");
    assert!(!is_worktree_configured(&config_path));

    std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    std::fs::write(
        &config_path,
        r#"{"worktrees":{"strategy":"checkout_subdir"}}"#,
    )
    .unwrap();
    assert!(is_worktree_configured(&config_path));
}

#[test]
fn load_worktree_result_from_config_returns_expected_defaults_and_values() {
    let td = tempfile::tempdir().expect("tempdir");
    let config_path = td.path().join(".ito/config.json");

    let r = load_worktree_result_from_config(&config_path);
    assert!(!r.ran);
    assert!(!r.enabled);
    assert!(r.strategy.is_none());
    assert!(r.integration_mode.is_none());

    std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    std::fs::write(
        &config_path,
        r#"{
  "worktrees": {
    "enabled": true,
    "strategy": "bare_control_siblings",
    "apply": {"integration_mode": "merge_parent"}
  }
}"#,
    )
    .unwrap();

    let r = load_worktree_result_from_config(&config_path);
    assert!(!r.ran);
    assert!(r.enabled);
    assert_eq!(r.strategy.as_deref(), Some("bare_control_siblings"));
    assert_eq!(r.integration_mode.as_deref(), Some("merge_parent"));
}

#[test]
fn save_worktree_config_writes_config_and_runs_print_paths() {
    let td = tempfile::tempdir().expect("tempdir");
    let config_path = td.path().join(".ito/config.json");

    let disabled = WorktreeWizardResult {
        ran: true,
        enabled: false,
        strategy: None,
        integration_mode: None,
    };
    save_worktree_config(&config_path, &disabled).expect("save disabled");
    let v = read_json(&config_path);
    assert_eq!(v["worktrees"]["enabled"], false);

    let enabled = WorktreeWizardResult {
        ran: true,
        enabled: true,
        strategy: Some("checkout_subdir".to_string()),
        integration_mode: Some("commit_pr".to_string()),
    };
    save_worktree_config(&config_path, &enabled).expect("save enabled");
    let v = read_json(&config_path);
    assert_eq!(v["worktrees"]["enabled"], true);
    assert_eq!(v["worktrees"]["strategy"], "checkout_subdir");
    assert_eq!(v["worktrees"]["apply"]["integration_mode"], "commit_pr");
}
