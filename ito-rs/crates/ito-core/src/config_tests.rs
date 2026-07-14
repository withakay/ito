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
fn validate_config_value_accepts_valid_repository_mode() {
    let parts = ["repository", "mode"];
    let value = json!("filesystem");
    assert!(validate_config_value(&parts, &value).is_ok());

    let value = json!("sqlite");
    assert!(validate_config_value(&parts, &value).is_ok());
}

#[test]
fn validate_config_value_rejects_invalid_repository_mode() {
    let parts = ["repository", "mode"];
    let value = json!("remote");
    let err = validate_config_value(&parts, &value).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Invalid value"));
    assert!(msg.contains("repository.mode"));
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
fn is_valid_repository_mode_checks_correctly() {
    assert!(is_valid_repository_mode("filesystem"));
    assert!(is_valid_repository_mode("sqlite"));
    assert!(!is_valid_repository_mode("remote"));
    assert!(!is_valid_repository_mode(""));
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
fn validate_config_value_accepts_positive_sync_interval() {
    let parts = ["changes", "coordination_branch", "sync_interval_seconds"];
    let value = json!(120);
    assert!(validate_config_value(&parts, &value).is_ok());
}

#[test]
fn validate_config_value_rejects_zero_sync_interval() {
    let parts = ["changes", "coordination_branch", "sync_interval_seconds"];
    let value = json!(0);
    let err = validate_config_value(&parts, &value).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("positive integer"));
    assert!(msg.contains("changes.coordination_branch.sync_interval_seconds"));
}

#[test]
fn validate_config_value_accepts_archive_main_integration_mode() {
    let parts = ["changes", "archive", "main_integration_mode"];
    let value = json!("pull_request_auto_merge");
    assert!(validate_config_value(&parts, &value).is_ok());
}

#[test]
fn validate_config_value_rejects_invalid_archive_main_integration_mode() {
    let parts = ["changes", "archive", "main_integration_mode"];
    let value = json!("always_merge");
    let err = validate_config_value(&parts, &value).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Invalid value"));
    assert!(msg.contains("changes.archive.main_integration_mode"));
}

#[test]
fn validate_config_value_accepts_proposal_integration_modes() {
    let parts = ["changes", "proposal", "integration_mode"];
    assert!(validate_config_value(&parts, &json!("pull_request")).is_ok());
    assert!(validate_config_value(&parts, &json!("direct_merge")).is_ok());
}

#[test]
fn validate_config_value_rejects_invalid_proposal_integration_mode_with_path() {
    let parts = ["changes", "proposal", "integration_mode"];
    let error = validate_config_value(&parts, &json!("merge_when_green"))
        .unwrap_err()
        .to_string();

    assert!(error.contains("changes.proposal.integration_mode"));
    assert!(error.contains("pull_request"));
    assert!(error.contains("direct_merge"));
}

#[test]
fn validate_config_value_accepts_valid_audit_mirror_branch_name() {
    let parts = ["audit", "mirror", "branch"];
    let value = json!("ito/internal/audit");
    assert!(validate_config_value(&parts, &value).is_ok());
}

#[test]
fn validate_config_value_rejects_invalid_audit_mirror_branch_name() {
    let parts = ["audit", "mirror", "branch"];
    let value = json!("--ito-audit");
    let err = validate_config_value(&parts, &value).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Invalid value"));
    assert!(msg.contains("audit.mirror.branch"));
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

// ---- memory config validation ------------------------------------------------

#[test]
fn validate_config_value_rejects_unknown_memory_kind() {
    let parts = ["memory", "capture", "kind"];
    let value = json!("delegate");
    let err = validate_config_value(&parts, &value).expect_err("expected error");
    let msg = err.to_string();
    assert!(msg.contains("memory.capture.kind"), "msg = {msg}");
    assert!(msg.contains("skill") && msg.contains("command"));
}

#[test]
fn validate_config_value_accepts_valid_memory_kind() {
    for op in ["capture", "search", "query"] {
        let parts = ["memory", op, "kind"];
        for kind in ["skill", "command"] {
            assert!(
                validate_config_value(&parts, &json!(kind)).is_ok(),
                "expected memory.{op}.kind = {kind} to validate"
            );
        }
    }
}

#[test]
fn validate_config_value_rejects_empty_memory_skill_id() {
    let parts = ["memory", "search", "skill"];
    let err = validate_config_value(&parts, &json!("   ")).expect_err("expected error");
    assert!(
        err.to_string().contains("memory.search.skill"),
        "msg = {err}"
    );
}

#[test]
fn validate_config_value_rejects_empty_memory_command_template() {
    let parts = ["memory", "query", "command"];
    let err = validate_config_value(&parts, &json!("")).expect_err("expected error");
    assert!(
        err.to_string().contains("memory.query.command"),
        "msg = {err}"
    );
}

#[test]
fn validate_config_value_rejects_unknown_memory_op_key() {
    let parts = ["memory"];
    let value = json!({
        "curate": { "kind": "command", "command": "noop" }
    });
    let err = validate_config_value(&parts, &value).expect_err("expected error");
    let msg = err.to_string();
    assert!(msg.contains("Unknown memory operation"), "msg = {msg}");
    assert!(msg.contains("curate"), "msg = {msg}");
}

#[test]
fn validate_config_value_rejects_memory_op_missing_required_field() {
    let parts = ["memory", "capture"];

    let err = validate_config_value(&parts, &json!({ "kind": "skill" }))
        .expect_err("skill variant requires `skill`");
    assert!(err.to_string().contains("memory.capture.skill"));

    let err = validate_config_value(&parts, &json!({ "kind": "command" }))
        .expect_err("command variant requires `command`");
    assert!(err.to_string().contains("memory.capture.command"));
}

#[test]
fn validate_config_value_rejects_memory_op_unknown_kind() {
    let parts = ["memory", "search"];
    let err = validate_config_value(&parts, &json!({ "kind": "magic", "command": "noop" }))
        .expect_err("expected error");
    let msg = err.to_string();
    assert!(msg.contains("Invalid 'kind' value 'magic'"), "msg = {msg}");
    assert!(msg.contains("memory.search"), "msg = {msg}");
}

#[test]
fn validate_memory_config_passes_when_no_skill_provider() {
    let config = MemoryConfig {
        capture: Some(MemoryOpConfig::Command {
            command: "brv curate \"{context}\"".to_string(),
        }),
        search: None,
        query: None,
    };
    validate_memory_config(&config, &[]).expect("command-only config should validate");
}

#[test]
fn validate_memory_config_passes_when_skill_resolves_in_flat_layout() {
    let tmp = tempfile::TempDir::new().unwrap();
    let skill_dir = tmp.path().join(".claude/skills/my-skill");
    std::fs::create_dir_all(&skill_dir).unwrap();
    std::fs::write(skill_dir.join("SKILL.md"), "stub").unwrap();

    let config = MemoryConfig {
        capture: Some(MemoryOpConfig::Skill {
            skill: "my-skill".to_string(),
            options: None,
        }),
        search: None,
        query: None,
    };
    let paths = known_skills_search_paths(tmp.path());
    validate_memory_config(&config, &paths).expect("flat skill should resolve");
}

#[test]
fn validate_memory_config_passes_when_skill_resolves_in_grouped_layout() {
    let tmp = tempfile::TempDir::new().unwrap();
    let skill_dir = tmp
        .path()
        .join(".agents/skills/byterover/byterover-explore");
    std::fs::create_dir_all(&skill_dir).unwrap();
    std::fs::write(skill_dir.join("SKILL.md"), "stub").unwrap();

    let config = MemoryConfig {
        capture: Some(MemoryOpConfig::Skill {
            skill: "byterover-explore".to_string(),
            options: None,
        }),
        search: None,
        query: None,
    };
    let paths = known_skills_search_paths(tmp.path());
    validate_memory_config(&config, &paths)
        .expect("grouped skill (.agents/skills/<group>/<id>) should resolve");
}

#[test]
fn validate_memory_config_rejects_missing_skill() {
    let tmp = tempfile::TempDir::new().unwrap();
    let config = MemoryConfig {
        capture: None,
        search: Some(MemoryOpConfig::Skill {
            skill: "nonexistent".to_string(),
            options: None,
        }),
        query: None,
    };
    let paths = known_skills_search_paths(tmp.path());
    let err =
        validate_memory_config(&config, &paths).expect_err("missing skill should fail validation");
    let msg = err.to_string();
    assert!(msg.contains("memory.search"), "msg = {msg}");
    assert!(msg.contains("nonexistent"), "msg = {msg}");
    // Searched-paths list should include at least one of the conventional dirs.
    assert!(msg.contains(".agents/skills") || msg.contains(".claude/skills"));
}

#[test]
fn skill_id_resolves_returns_false_when_no_paths_exist() {
    let tmp = tempfile::TempDir::new().unwrap();
    let paths = known_skills_search_paths(tmp.path());
    assert!(!skill_id_resolves("anything", &paths));
}
