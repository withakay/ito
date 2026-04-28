use super::*;
use serde_json::json;

#[test]
fn worktree_config_defaults_when_no_worktrees_key() {
    let merged = json!({});
    let cfg = worktree_config_from_merged(&merged, None);

    assert!(!cfg.enabled);
    assert_eq!(cfg.strategy, WorktreeStrategy::CheckoutSubdir);
    assert!(cfg.layout_base_dir.is_none());
    assert_eq!(cfg.layout_dir_name, "ito-worktrees");
    assert!(cfg.apply_enabled);
    assert_eq!(cfg.integration_mode, "commit_pr");
    assert_eq!(
        cfg.copy_from_main,
        vec![".env", ".envrc", ".mise.local.toml"]
    );
    assert!(cfg.setup_commands.is_empty());
    assert_eq!(cfg.default_branch, "main");
    assert!(cfg.project_root.is_none());
}

#[test]
fn worktree_config_parses_all_fields() {
    let merged = json!({
        "worktrees": {
            "enabled": true,
            "strategy": "checkout_siblings",
            "default_branch": "develop",
            "layout": {
                "base_dir": "/tmp/worktrees",
                "dir_name": "my-trees"
            },
            "apply": {
                "enabled": false,
                "integration_mode": "merge",
                "copy_from_main": [".env.local"],
                "setup_commands": ["npm install", "cargo build"]
            }
        }
    });
    let cfg = worktree_config_from_merged(&merged, None);

    assert!(cfg.enabled);
    assert_eq!(cfg.strategy, WorktreeStrategy::CheckoutSiblings);
    assert_eq!(cfg.default_branch, "develop");
    assert_eq!(cfg.layout_base_dir.as_deref(), Some("/tmp/worktrees"));
    assert_eq!(cfg.layout_dir_name, "my-trees");
    assert!(!cfg.apply_enabled);
    assert_eq!(cfg.integration_mode, "merge");
    assert_eq!(cfg.copy_from_main, vec![".env.local"]);
    assert_eq!(cfg.setup_commands, vec!["npm install", "cargo build"]);
}

#[test]
fn worktree_config_ignores_empty_strings() {
    let merged = json!({
        "worktrees": {
            "strategy": "",
            "default_branch": "",
            "layout": {
                "base_dir": "",
                "dir_name": ""
            },
            "apply": {
                "integration_mode": ""
            }
        }
    });
    let cfg = worktree_config_from_merged(&merged, None);

    assert_eq!(cfg.strategy, WorktreeStrategy::CheckoutSubdir);
    assert_eq!(cfg.default_branch, "main");
    assert!(cfg.layout_base_dir.is_none());
    assert_eq!(cfg.layout_dir_name, "ito-worktrees");
    assert_eq!(cfg.integration_mode, "commit_pr");
}

#[test]
fn worktree_config_checkout_subdir_sets_project_root() {
    let merged = json!({
        "worktrees": {
            "strategy": "checkout_subdir"
        }
    });
    let root = Path::new("/fake/project");
    let cfg = worktree_config_from_merged(&merged, Some(root));

    assert_eq!(cfg.project_root.as_deref(), Some("/fake/project"));
}

#[test]
fn worktree_config_checkout_siblings_sets_project_root() {
    let merged = json!({
        "worktrees": {
            "strategy": "checkout_siblings"
        }
    });
    let root = Path::new("/fake/project");
    let cfg = worktree_config_from_merged(&merged, Some(root));

    assert_eq!(cfg.project_root.as_deref(), Some("/fake/project"));
}

#[test]
fn worktree_config_bare_control_siblings_calls_resolve() {
    let merged = json!({
        "worktrees": {
            "strategy": "bare_control_siblings"
        }
    });
    let cfg = worktree_config_from_merged(&merged, None);
    assert!(cfg.project_root.is_none());

    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected = resolve_bare_repo_root(project_root)
        .expect("CARGO_MANIFEST_DIR should resolve to the bare repo root in this repo");

    let cfg = worktree_config_from_merged(&merged, Some(project_root));
    assert_eq!(
        cfg.project_root.as_deref(),
        Some(expected.to_string_lossy().as_ref())
    );
}

#[test]
fn worktree_config_no_project_root_when_none_passed() {
    let merged = json!({
        "worktrees": {
            "strategy": "checkout_siblings"
        }
    });
    let cfg = worktree_config_from_merged(&merged, None);
    assert!(cfg.project_root.is_none());
}

#[test]
fn json_get_traverses_nested_keys() {
    let root = json!({
        "a": {
            "b": {
                "c": 42
            }
        }
    });

    let result = json_get(&root, &["a", "b", "c"]);
    assert_eq!(result, Some(&json!(42)));
}

#[test]
fn json_get_returns_none_for_missing_key() {
    let root = json!({"a": {"b": 1}});
    assert!(json_get(&root, &["a", "x"]).is_none());
}

#[test]
fn json_get_returns_none_for_non_object_intermediate() {
    let root = json!({"a": 42});
    assert!(json_get(&root, &["a", "b"]).is_none());
}

#[test]
fn json_get_empty_keys_returns_root() {
    let root = json!({"a": 1});
    assert_eq!(json_get(&root, &[]), Some(&root));
}

#[test]
fn collect_tracking_diagnostic_counts_none_input() {
    let (errors, warnings) = collect_tracking_diagnostic_counts(None);
    assert!(errors.is_none());
    assert!(warnings.is_none());
}

#[test]
fn collect_tracking_diagnostic_counts_empty_slice() {
    let diags: Vec<core_templates::TaskDiagnostic> = vec![];
    let (errors, warnings) = collect_tracking_diagnostic_counts(Some(&diags));
    assert!(errors.is_none());
    assert!(warnings.is_none());
}

#[test]
fn collect_tracking_diagnostic_counts_mixed_levels() {
    let diags = vec![
        core_templates::TaskDiagnostic {
            level: "error".to_string(),
            message: "e1".to_string(),
            task_id: None,
        },
        core_templates::TaskDiagnostic {
            level: "warning".to_string(),
            message: "w1".to_string(),
            task_id: None,
        },
        core_templates::TaskDiagnostic {
            level: "error".to_string(),
            message: "e2".to_string(),
            task_id: None,
        },
        core_templates::TaskDiagnostic {
            level: "info".to_string(),
            message: "i1".to_string(),
            task_id: None,
        },
    ];
    let (errors, warnings) = collect_tracking_diagnostic_counts(Some(&diags));
    assert_eq!(errors, Some(2));
    assert_eq!(warnings, Some(1));
}

#[test]
fn worktree_config_parses_bare_control_siblings_strategy() {
    let merged = json!({
        "worktrees": {
            "strategy": "bare_control_siblings"
        }
    });
    let cfg = worktree_config_from_merged(&merged, None);
    assert_eq!(cfg.strategy, WorktreeStrategy::BareControlSiblings);
}

#[test]
fn collect_context_files_preserves_order() {
    let mut map = BTreeMap::new();
    map.insert("alpha".to_string(), "path/a".to_string());
    map.insert("beta".to_string(), "path/b".to_string());

    let entries = collect_context_files(&map);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].id, "alpha");
    assert_eq!(entries[0].path, "path/a");
    assert_eq!(entries[1].id, "beta");
    assert_eq!(entries[1].path, "path/b");
}

#[test]
fn backend_instruction_is_cli_first_for_remote_mode() {
    let instruction = generate_backend_instruction().expect("backend template should render");

    assert!(instruction.contains("do not create markdown manually"));
    assert!(instruction.contains("ito show specs"));
    assert!(instruction.contains("ito patch change <change-id> <proposal|design|tasks>"));
    assert!(instruction.contains("ito write change <change-id> spec <capability>"));
    assert!(instruction.contains("ito tasks sync pull <change-id>"));
    assert!(instruction.contains("ito archive <change-id>"));
}
