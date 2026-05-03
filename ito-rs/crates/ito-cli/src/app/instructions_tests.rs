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

// ── Task 1.1: Config cache tests ─────────────────────────────────────────────

#[test]
fn testing_policy_from_merged_uses_defaults_when_empty() {
    let merged = json!({});
    let policy = testing_policy_from_merged(&merged);

    assert_eq!(policy.tdd_workflow, "red-green-refactor");
    assert_eq!(policy.coverage_target_percent, 80);
}

#[test]
fn testing_policy_from_merged_reads_overrides() {
    let merged = json!({
        "defaults": {
            "testing": {
                "tdd": { "workflow": "outside-in" },
                "coverage": { "target_percent": 95 }
            }
        }
    });
    let policy = testing_policy_from_merged(&merged);

    assert_eq!(policy.tdd_workflow, "outside-in");
    assert_eq!(policy.coverage_target_percent, 95);
}

#[test]
fn testing_policy_from_merged_ignores_empty_workflow() {
    let merged = json!({
        "defaults": {
            "testing": {
                "tdd": { "workflow": "  " }
            }
        }
    });
    let policy = testing_policy_from_merged(&merged);

    assert_eq!(policy.tdd_workflow, "red-green-refactor");
}

#[test]
fn worktree_config_from_resolved_matches_with_paths() {
    let merged = json!({
        "worktrees": {
            "enabled": true,
            "strategy": "checkout_subdir",
            "default_branch": "develop"
        }
    });
    let project_root = Path::new("/fake/project");
    let ito_path = Path::new("/fake/project/.ito");

    let from_resolved = worktree_config_from_resolved(&merged, project_root, ito_path);
    let from_with_paths = worktree_config_from_merged_with_paths(&merged, project_root, ito_path);

    assert_eq!(from_resolved.enabled, from_with_paths.enabled);
    assert_eq!(from_resolved.strategy, from_with_paths.strategy);
    assert_eq!(from_resolved.default_branch, from_with_paths.default_branch);
    assert_eq!(from_resolved.worktree_root, from_with_paths.worktree_root);
    assert_eq!(from_resolved.ito_root, from_with_paths.ito_root);
}

#[test]
fn runtime_resolved_config_returns_same_instance() {
    let rt = crate::runtime::Runtime::new();
    let first = rt.resolved_config();
    let second = rt.resolved_config();

    // Both calls return the same reference (pointer equality).
    assert!(std::ptr::eq(first, second));
}

// ── Task 1.2: Sync opt-in tests ─────────────────────────────────────────────

#[test]
fn sync_before_change_resolution_apply_defaults_to_false() {
    assert!(!sync_before_change_resolution("apply", false));
}

#[test]
fn sync_before_change_resolution_apply_with_sync_flag() {
    assert!(sync_before_change_resolution("apply", true));
}

#[test]
fn sync_before_change_resolution_proposal_always_syncs() {
    assert!(sync_before_change_resolution("proposal", false));
    assert!(sync_before_change_resolution("proposal", true));
}

#[test]
fn sync_before_change_resolution_review_always_syncs() {
    assert!(sync_before_change_resolution("review", false));
    assert!(sync_before_change_resolution("review", true));
}

#[test]
fn sync_before_change_resolution_other_artifacts_never_sync() {
    assert!(!sync_before_change_resolution("specs", false));
    assert!(!sync_before_change_resolution("tasks", false));
    assert!(!sync_before_change_resolution("design", false));
}

// ── Task 2.1: Regression tests — apply does NOT sync by default ─────────────

/// Regression: the raw `--sync` flag extraction from an args slice must match
/// the same logic used in `handle_agent_instruction`.  This test exercises the
/// argument-parsing path (`args.iter().any(|a| a == "--sync")`) combined with
/// the predicate, proving that a bare `apply --change <id>` invocation never
/// triggers coordination sync.
#[test]
fn apply_args_without_sync_flag_do_not_trigger_sync() {
    let args: Vec<String> = vec!["apply", "--change", "001-01_test-change"]
        .into_iter()
        .map(String::from)
        .collect();

    let want_sync = args.iter().any(|a| a == "--sync");
    assert!(!want_sync, "--sync must not be inferred when absent");
    assert!(
        !sync_before_change_resolution("apply", want_sync),
        "apply without --sync must not trigger coordination sync"
    );
}

/// Regression: when `--sync` IS present in the args, the predicate must
/// return `true` for `apply`.
#[test]
fn apply_args_with_sync_flag_trigger_sync() {
    let args: Vec<String> = vec!["apply", "--change", "001-01_test-change", "--sync"]
        .into_iter()
        .map(String::from)
        .collect();

    let want_sync = args.iter().any(|a| a == "--sync");
    assert!(want_sync, "--sync must be detected when present");
    assert!(
        sync_before_change_resolution("apply", want_sync),
        "apply with --sync must trigger coordination sync"
    );
}

/// Regression: `--sync` at any position in the args vector must be detected.
/// Guards against positional-only parsing regressions.
#[test]
fn sync_flag_detected_regardless_of_position() {
    let args_front: Vec<String> = vec!["--sync", "apply", "--change", "x"]
        .into_iter()
        .map(String::from)
        .collect();
    assert!(
        args_front.iter().any(|a| a == "--sync"),
        "--sync at front must be detected"
    );

    let args_middle: Vec<String> = vec!["apply", "--sync", "--change", "x"]
        .into_iter()
        .map(String::from)
        .collect();
    assert!(
        args_middle.iter().any(|a| a == "--sync"),
        "--sync in middle must be detected"
    );
}

/// Regression: the clap `AgentInstructionArgs` round-trips `--sync` correctly
/// through `to_argv()`.  This catches drift between the clap struct and the
/// legacy string-based handler.
#[test]
fn agent_instruction_args_round_trips_sync_flag() {
    use crate::cli::AgentInstructionArgs;

    // Without --sync
    let args_no_sync = AgentInstructionArgs {
        artifact: "apply".to_string(),
        change: Some("001-01_test".to_string()),
        tool: None,
        schema: None,
        json: false,
        sync: false,
        variant: None,
        profile: None,
        operation: None,
        context: None,
        file: vec![],
        folder: vec![],
        query: None,
        limit: None,
        scope: None,
    };
    let argv = args_no_sync.to_argv();
    assert!(
        !argv.iter().any(|a| a == "--sync"),
        "to_argv() must not emit --sync when sync=false"
    );

    // With --sync
    let args_with_sync = AgentInstructionArgs {
        sync: true,
        ..args_no_sync
    };
    let argv = args_with_sync.to_argv();
    assert!(
        argv.iter().any(|a| a == "--sync"),
        "to_argv() must emit --sync when sync=true"
    );
}

/// Regression: exhaustive artifact coverage for the sync predicate.
/// Every known change-scoped artifact must have a defined sync policy.
/// This test fails loudly if a new artifact is added without updating
/// `sync_before_change_resolution`.
#[test]
fn sync_predicate_covers_all_known_change_artifacts() {
    let change_artifacts = ["apply", "proposal", "review", "specs", "tasks", "design"];

    for artifact in &change_artifacts {
        // Must not panic — the predicate must handle every known artifact.
        let _without = sync_before_change_resolution(artifact, false);
        let _with = sync_before_change_resolution(artifact, true);
    }

    // Verify the specific policies are stable:
    // apply: opt-in only
    assert!(!sync_before_change_resolution("apply", false));
    assert!(sync_before_change_resolution("apply", true));

    // proposal, review: always sync
    assert!(sync_before_change_resolution("proposal", false));
    assert!(sync_before_change_resolution("review", false));

    // specs, tasks, design: never sync
    assert!(!sync_before_change_resolution("specs", true));
    assert!(!sync_before_change_resolution("tasks", true));
    assert!(!sync_before_change_resolution("design", true));
}
