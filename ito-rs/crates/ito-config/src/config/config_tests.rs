use super::*;

#[test]
fn cascading_project_config_merges_sources_in_order_with_scalar_override() {
    let repo = tempfile::tempdir().unwrap();

    std::fs::write(
        repo.path().join("ito.json"),
        "{\"obj\":{\"a\":1},\"arr\":[1],\"x\":\"repo\"}",
    )
    .unwrap();
    std::fs::write(
        repo.path().join(".ito.json"),
        "{\"obj\":{\"b\":2},\"arr\":[2],\"y\":\"dot\"}",
    )
    .unwrap();

    let project_dir = tempfile::tempdir().unwrap();
    std::fs::write(
        project_dir.path().join("config.json"),
        "{\"obj\":{\"c\":3},\"x\":\"project_dir\"}",
    )
    .unwrap();

    let ctx = ConfigContext {
        xdg_config_home: None,
        home_dir: None,
        project_dir: Some(project_dir.path().to_path_buf()),
    };
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);
    std::fs::create_dir_all(&ito_path).unwrap();
    std::fs::write(
        ito_path.join("config.json"),
        "{\"obj\":{\"a\":9},\"z\":\"ito_dir\"}",
    )
    .unwrap();

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);

    assert_eq!(
        r.merged.get("obj").unwrap(),
        &serde_json::json!({"a": 9, "b": 2, "c": 3})
    );
    assert_eq!(r.merged.get("arr").unwrap(), &serde_json::json!([2]));
    assert_eq!(
        r.merged.get("x").unwrap(),
        &serde_json::json!("project_dir")
    );
    assert_eq!(r.merged.get("y").unwrap(), &serde_json::json!("dot"));
    assert_eq!(r.merged.get("z").unwrap(), &serde_json::json!("ito_dir"));

    // Defaults are present.
    assert!(r.merged.get("cache").is_some());
    assert!(r.merged.get("harnesses").is_some());

    assert_eq!(
        r.loaded_from,
        vec![
            repo.path().join("ito.json"),
            repo.path().join(".ito.json"),
            ito_path.join("config.json"),
            project_dir.path().join("config.json"),
        ]
    );

    // The new `layers` field is precedence-ordered (low → high) and
    // each entry carries the un-merged value of that layer alone.
    let layer_paths: Vec<_> = r.layers.iter().map(|l| &l.path).collect();
    assert_eq!(
        layer_paths,
        vec![
            &repo.path().join("ito.json"),
            &repo.path().join(".ito.json"),
            &ito_path.join("config.json"),
            &project_dir.path().join("config.json"),
        ]
    );

    // The first layer (`ito.json`) only set `obj.a = 1`, `arr = [1]`,
    // and `x = "repo"` — its raw value must NOT include the
    // higher-precedence overrides applied later.
    let first = &r.layers[0].value;
    assert_eq!(first.get("x").unwrap(), &serde_json::json!("repo"));
    assert_eq!(first.get("obj").unwrap(), &serde_json::json!({"a": 1}));
    assert!(
        first.get("z").is_none(),
        "layer 0 must not see ito_dir keys"
    );
}

#[test]
fn cascading_project_config_ignores_invalid_json_sources() {
    let repo = tempfile::tempdir().unwrap();

    std::fs::write(repo.path().join("ito.json"), "{\"a\":1}").unwrap();
    std::fs::write(repo.path().join(".ito.json"), "not-json").unwrap();

    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    assert_eq!(r.merged.get("a").unwrap(), &serde_json::json!(1));
    assert!(r.merged.get("cache").is_some());

    assert_eq!(r.loaded_from, vec![repo.path().join("ito.json")]);
}

#[test]
fn cascading_project_config_ignores_schema_ref_key() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("ito.json"),
        "{\"$schema\":\"./config.schema.json\",\"a\":1}",
    )
    .unwrap();

    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    assert_eq!(r.merged.get("a").unwrap(), &serde_json::json!(1));
    assert!(r.merged.get("$schema").is_none());
}

#[test]
fn global_config_path_prefers_xdg() {
    let ctx = ConfigContext {
        xdg_config_home: Some(PathBuf::from("/tmp/xdg")),
        home_dir: Some(PathBuf::from("/tmp/home")),
        project_dir: None,
    };
    #[cfg(not(windows))]
    assert_eq!(
        global_config_path(&ctx).unwrap(),
        PathBuf::from("/tmp/xdg/ito/config.json")
    );
}

#[test]
fn ito_config_dir_prefers_xdg() {
    let ctx = ConfigContext {
        xdg_config_home: Some(PathBuf::from("/tmp/xdg")),
        home_dir: Some(PathBuf::from("/tmp/home")),
        project_dir: None,
    };
    #[cfg(not(windows))]
    assert_eq!(ito_config_dir(&ctx).unwrap(), PathBuf::from("/tmp/xdg/ito"));
}

#[test]
fn worktrees_config_has_defaults_in_cascading_config() {
    let repo = tempfile::tempdir().unwrap();
    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let wt = r
        .merged
        .get("worktrees")
        .expect("worktrees key should exist");

    assert_eq!(wt.get("enabled").and_then(|v| v.as_bool()), Some(false));
    assert_eq!(
        wt.get("strategy").and_then(|v| v.as_str()),
        Some("checkout_subdir")
    );
    assert_eq!(
        wt.get("default_branch").and_then(|v| v.as_str()),
        Some("main")
    );

    let layout = wt.get("layout").unwrap();
    assert_eq!(
        layout.get("dir_name").and_then(|v| v.as_str()),
        Some("ito-worktrees")
    );

    let apply = wt.get("apply").unwrap();
    assert_eq!(apply.get("enabled").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(
        apply.get("integration_mode").and_then(|v| v.as_str()),
        Some("commit_pr")
    );

    let copy = apply
        .get("copy_from_main")
        .and_then(|v| v.as_array())
        .unwrap();
    assert_eq!(copy.len(), 3);
}

#[test]
fn removed_tools_tmux_key_is_ignored_by_cascading_config() {
    let repo = tempfile::tempdir().unwrap();
    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);
    std::fs::create_dir_all(&ito_path).unwrap();
    std::fs::write(
        ito_path.join("config.json"),
        r#"{"tools":{"tmux":{"enabled":false}}}"#,
    )
    .unwrap();

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    assert!(
        r.merged.pointer("/tools/tmux/enabled").is_none(),
        "removed tools.tmux.enabled must have no runtime effect"
    );
    assert!(
        r.layers[0].value.pointer("/tools/tmux/enabled").is_none(),
        "resolved layers must not advertise the removed key"
    );
}

#[test]
fn legacy_worktree_default_branch_key_migrates() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("ito.json"),
        r#"{"worktrees":{"defaultBranch":"develop"}}"#,
    )
    .unwrap();

    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let wt = r.merged.get("worktrees").unwrap();

    assert_eq!(
        wt.get("default_branch").and_then(|v| v.as_str()),
        Some("develop")
    );
    assert!(wt.get("defaultBranch").is_none());
}

#[test]
fn legacy_worktree_local_files_key_migrates() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("ito.json"),
        r#"{"worktrees":{"localFiles":[".env",".secrets"]}}"#,
    )
    .unwrap();

    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let wt = r.merged.get("worktrees").unwrap();
    let apply = wt.get("apply").unwrap();
    let copy = apply
        .get("copy_from_main")
        .and_then(|v| v.as_array())
        .unwrap();

    assert_eq!(copy.len(), 2);
    assert_eq!(copy[0].as_str(), Some(".env"));
    assert_eq!(copy[1].as_str(), Some(".secrets"));
    assert!(wt.get("localFiles").is_none());
}

#[test]
fn new_worktree_keys_take_precedence_over_legacy() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
            repo.path().join("ito.json"),
            r#"{"worktrees":{"defaultBranch":"legacy","default_branch":"new-main","localFiles":[".old"],"apply":{"copy_from_main":[".new"]}}}"#,
        )
        .unwrap();

    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let wt = r.merged.get("worktrees").unwrap();

    assert_eq!(
        wt.get("default_branch").and_then(|v| v.as_str()),
        Some("new-main")
    );

    let apply = wt.get("apply").unwrap();
    let copy = apply
        .get("copy_from_main")
        .and_then(|v| v.as_array())
        .unwrap();
    assert_eq!(copy.len(), 1);
    assert_eq!(copy[0].as_str(), Some(".new"));
}

#[test]
fn coordination_branch_defaults_exist_in_cascading_config() {
    let repo = tempfile::tempdir().unwrap();
    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let changes = r.merged.get("changes").expect("changes key should exist");
    let coordination = changes
        .get("coordination_branch")
        .expect("coordination_branch key should exist");

    assert_eq!(
        coordination.get("enabled").and_then(|v| v.as_bool()),
        Some(true)
    );
    assert_eq!(
        coordination.get("name").and_then(|v| v.as_str()),
        Some("ito/internal/changes")
    );
}

#[test]
fn coordination_branch_defaults_can_be_overridden() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("ito.json"),
        r#"{"changes":{"coordination_branch":{"enabled":false,"name":"team/internal/coord"}}}"#,
    )
    .unwrap();

    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let changes = r.merged.get("changes").expect("changes key should exist");
    let coordination = changes
        .get("coordination_branch")
        .expect("coordination_branch key should exist");

    assert_eq!(
        coordination.get("enabled").and_then(|v| v.as_bool()),
        Some(false)
    );
    assert_eq!(
        coordination.get("name").and_then(|v| v.as_str()),
        Some("team/internal/coord")
    );
}

#[test]
fn proposal_integration_mode_defaults_in_cascading_config() {
    let repo = tempfile::tempdir().unwrap();
    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let resolved = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let mode = resolved
        .merged
        .pointer("/changes/proposal/integration_mode")
        .and_then(|value| value.as_str());

    assert_eq!(mode, Some("pull_request"));
}

#[test]
fn proposal_integration_mode_cascades_with_later_override() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("ito.json"),
        r#"{"changes":{"proposal":{"integration_mode":"pull_request"}}}"#,
    )
    .unwrap();

    let ito_path = repo.path().join(".ito");
    std::fs::create_dir_all(&ito_path).unwrap();
    std::fs::write(
        ito_path.join("config.json"),
        r#"{"changes":{"proposal":{"integration_mode":"direct_merge"}}}"#,
    )
    .unwrap();

    let resolved = load_cascading_project_config(repo.path(), &ito_path, &ConfigContext::default());
    let typed: types::ItoConfig = serde_json::from_value(resolved.merged).unwrap();

    assert_eq!(
        typed.changes.proposal.integration_mode,
        types::ProposalIntegrationMode::DirectMerge
    );
}

#[test]
fn proposal_integration_mode_accepts_only_supported_values() {
    for (value, expected) in [
        ("pull_request", types::ProposalIntegrationMode::PullRequest),
        ("direct_merge", types::ProposalIntegrationMode::DirectMerge),
    ] {
        let config: types::ItoConfig = serde_json::from_value(serde_json::json!({
            "changes": {"proposal": {"integration_mode": value}}
        }))
        .unwrap();
        assert_eq!(config.changes.proposal.integration_mode, expected);
    }

    let error = serde_json::from_value::<types::ItoConfig>(serde_json::json!({
        "changes": {"proposal": {"integration_mode": "merge_when_green"}}
    }))
    .unwrap_err()
    .to_string();

    assert!(error.contains("unknown variant"));
    assert!(error.contains("pull_request"));
    assert!(error.contains("direct_merge"));
}

#[test]
fn audit_mirror_defaults_exist_in_cascading_config() {
    let repo = tempfile::tempdir().unwrap();
    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let audit = r.merged.get("audit").expect("audit key should exist");
    let mirror = audit.get("mirror").expect("audit.mirror key should exist");

    assert_eq!(mirror.get("enabled").and_then(|v| v.as_bool()), Some(false));
    assert_eq!(
        mirror.get("branch").and_then(|v| v.as_str()),
        Some("ito/internal/audit")
    );
}

#[test]
fn audit_mirror_defaults_can_be_overridden() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("ito.json"),
        r#"{"audit":{"mirror":{"enabled":true,"branch":"team/internal/audit"}}}"#,
    )
    .unwrap();

    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let audit = r.merged.get("audit").expect("audit key should exist");
    let mirror = audit.get("mirror").expect("audit.mirror key should exist");

    assert_eq!(mirror.get("enabled").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(
        mirror.get("branch").and_then(|v| v.as_str()),
        Some("team/internal/audit")
    );
}

#[test]
fn load_global_ito_config_reads_backend_server_auth() {
    let home = tempfile::tempdir().unwrap();
    let config_dir = home.path().join(".config/ito");
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(
        config_dir.join("config.json"),
        r#"{
                "backendServer": {
                    "auth": {
                        "adminTokens": ["test-admin-token"],
                        "tokenSeed": "test-seed"
                    }
                }
            }"#,
    )
    .unwrap();

    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    let config = load_global_ito_config(&ctx);
    assert_eq!(
        config.backend_server.auth.admin_tokens,
        vec!["test-admin-token"]
    );
    assert_eq!(
        config.backend_server.auth.token_seed,
        Some("test-seed".to_string())
    );
}

#[test]
fn load_global_ito_config_returns_defaults_when_no_file() {
    let home = tempfile::tempdir().unwrap();
    let ctx = ConfigContext {
        home_dir: Some(home.path().to_path_buf()),
        xdg_config_home: None,
        project_dir: None,
    };

    let config = load_global_ito_config(&ctx);
    assert!(config.backend_server.auth.admin_tokens.is_empty());
    assert!(config.backend_server.auth.token_seed.is_none());
}

#[test]
fn logging_invalid_commands_defaults_exist_in_cascading_config() {
    let repo = tempfile::tempdir().unwrap();
    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let logging = r.merged.get("logging").expect("logging key should exist");
    let invalid_commands = logging
        .get("invalidCommands")
        .expect("logging.invalidCommands key should exist");

    assert_eq!(
        invalid_commands.get("enabled").and_then(|v| v.as_bool()),
        Some(false)
    );
}

#[test]
fn logging_invalid_commands_can_be_enabled() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("ito.json"),
        r#"{"logging":{"invalidCommands":{"enabled":true}}}"#,
    )
    .unwrap();

    let ctx = ConfigContext::default();
    let ito_path = crate::ito_dir::get_ito_path(repo.path(), &ctx);

    let r = load_cascading_project_config(repo.path(), &ito_path, &ctx);
    let cfg: types::ItoConfig = serde_json::from_value(r.merged).expect("should deserialize");

    assert!(cfg.logging.invalid_commands.enabled);
}

// ito_dir tests live in crate::ito_dir.
