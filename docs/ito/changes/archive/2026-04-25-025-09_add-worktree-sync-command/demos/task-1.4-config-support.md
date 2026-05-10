# Task 1.4: Sync interval and archive config support

*2026-04-24T06:21:12Z by Showboat 0.6.1*
<!-- showboat-id: ccf9ea7e-8745-4031-a841-31bfe8358c15 -->

Added changes.coordination_branch.sync_interval_seconds with a 120-second default, added changes.archive.main_integration_mode with a pull_request default, validated both via ito config, and regenerated the checked-in config schema.

```bash
cargo test -p ito-cli config -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.19s
     Running unittests src/main.rs (target/debug/deps/ito-6d4676fcba121558)

running 23 tests
test app::instructions::tests::worktree_config_checkout_siblings_sets_project_root ... ok
test app::instructions::tests::worktree_config_checkout_subdir_sets_project_root ... ok
test app::instructions::tests::worktree_config_defaults_when_no_worktrees_key ... ok
test app::instructions::tests::worktree_config_ignores_empty_strings ... ok
test app::instructions::tests::worktree_config_parses_bare_control_siblings_strategy ... ok
test app::instructions::tests::worktree_config_no_project_root_when_none_passed ... ok
test app::instructions::tests::worktree_config_parses_all_fields ... ok
test commands::serve_api::serve_api_tests::builds_config_with_defaults ... ok

Worktree mode disabled.
Config file: /var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpVRp84g/.ito/config.json
  worktrees.enabled = false


Worktree configuration saved.
Config file: /var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpVRp84g/.ito/config.json
  worktrees.enabled = true
  worktrees.strategy = checkout_subdir
  worktrees.apply.integration_mode = commit_pr

test app::worktree_wizard::worktree_wizard_tests::load_worktree_result_from_config_returns_expected_defaults_and_values ... ok
test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_errors_when_enabled_missing_fields ... ok
test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_enabled_settings ... ok
test commands::config::config_tests::config_schema_includes_archive_main_integration_mode_default ... ok
test commands::config::config_tests::json_render_value_renders_common_json_types ... ok
test app::worktree_wizard::worktree_wizard_tests::save_worktree_config_writes_config_and_runs_print_paths ... ok
test commands::config::config_tests::config_schema_includes_coordination_sync_interval_default ... ok
test commands::serve_api::serve_api_tests::load_backend_server_config_file_accepts_full_ito_json_config ... ok
test app::worktree_wizard::worktree_wizard_tests::is_worktree_configured_detects_strategy_key ... ok
test commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_unknown_json_fields ... ok
test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_disabled_and_preserves_other_keys ... ok
test commands::config::config_tests::handle_config_schema_writes_file_when_output_is_set ... ok
test commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_trailing_json_content ... ok
test commands::serve_api::serve_api_tests::load_backend_server_config_file_reads_toml ... ok
test app::instructions::tests::worktree_config_bare_control_siblings_calls_resolve ... ok

test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out; finished in 0.02s

     Running tests/agent_instruction_bootstrap.rs (target/debug/deps/agent_instruction_bootstrap-9d0a1e6df3a6997d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_context.rs (target/debug/deps/agent_instruction_context-8e07738d929c3d00)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_repo_sweep.rs (target/debug/deps/agent_instruction_repo_sweep-c89feac25192a721)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/agent_instruction_worktrees.rs (target/debug/deps/agent_instruction_worktrees-dd6ca92ee032fc44)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/aliases.rs (target/debug/deps/aliases-60641ba65fc52a06)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/archive_completed.rs (target/debug/deps/archive_completed-5c0ee0499a12130e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/archive_remote_mode.rs (target/debug/deps/archive_remote_mode-25ef0a1c271a7985)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/archive_smoke.rs (target/debug/deps/archive_smoke-768f7db9ce0360e3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/audit_more.rs (target/debug/deps/audit_more-9868b46e3eef7e21)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_remote_mode.rs (target/debug/deps/audit_remote_mode-f2cb0a563d8f0ac7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_import.rs (target/debug/deps/backend_import-b71f6eddf3cc7067)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/backend_qa_walkthrough.rs (target/debug/deps/backend_qa_walkthrough-99093e55c472f7c7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_serve.rs (target/debug/deps/backend_serve-e0450e45a263fa0a)

running 2 tests
test backend_serve_service_mode_reports_malformed_backend_config ... ok
test backend_serve_reports_unknown_fields_in_explicit_config_file ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.46s

     Running tests/backend_status_more.rs (target/debug/deps/backend_status_more-aa519b75c3f3154c)

running 6 tests
test backend_status_with_valid_config_but_no_server ... ok
test backend_status_incomplete_config_fails ... ok
test backend_status_json_includes_config_details ... ok
test silent_fallback_grep_warns_on_bad_config ... ok
test silent_fallback_tasks_warns_on_bad_config ... ok
test silent_fallback_event_forwarding_warns_on_bad_config ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.37s

     Running tests/cli_smoke.rs (target/debug/deps/cli_smoke-96370ce6c164066c)

running 1 test
test create_workflow_plan_state_config_smoke ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.54s

     Running tests/cli_snapshots.rs (target/debug/deps/cli_snapshots-7c1730df16091dce)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/config_more.rs (target/debug/deps/config_more-2b07fe477bfa3aa6)

running 5 tests
test config_set_rejects_invalid_audit_mirror_branch_name ... ok
test config_set_rejects_invalid_coordination_branch_name ... ok
test config_unknown_subcommand_errors ... ok
test config_help_path_list_unset_and_schema_smoke ... ok
test config_set_get_supports_coordination_and_audit_mirror_keys ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.52s

     Running tests/coverage_smoke.rs (target/debug/deps/coverage_smoke-7ee31e863e055e38)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/create_more.rs (target/debug/deps/create_more-1eb549e6ee2b19f4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/grep_more.rs (target/debug/deps/grep_more-12d6fa086b067b60)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/help.rs (target/debug/deps/help-47cf4e4502e137a2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/init_coordination.rs (target/debug/deps/init_coordination-a8e1fe050cfe618d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/init_gitignore_session_json.rs (target/debug/deps/init_gitignore_session_json-02d4d8af74b870e1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/init_more.rs (target/debug/deps/init_more-990e9107201c627c)

running 2 tests
test init_writes_config_with_release_tag_schema_reference ... ok
test init_setup_coordination_branch_uses_configured_branch_name ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.24s

     Running tests/init_tmux.rs (target/debug/deps/init_tmux-0cb603e1b3d3489e)

running 1 test
test init_uses_cascading_tmux_preference_from_global_config ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.03s

     Running tests/init_upgrade_more.rs (target/debug/deps/init_upgrade_more-bf0b5ccaa73da775)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/instructions_more.rs (target/debug/deps/instructions_more-5a6197adeb4e71b2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/list_regression.rs (target/debug/deps/list_regression-267bcdf7f3cacc50)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/misc_more.rs (target/debug/deps/misc_more-ebc8cb0d8172f0d7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s

     Running tests/new_more.rs (target/debug/deps/new_more-84efd7f8317ba173)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/parity_help_version.rs (target/debug/deps/parity_help_version-02a624cd49df9249)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/parity_tasks.rs (target/debug/deps/parity_tasks-45e2840c229ce443)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/path_more.rs (target/debug/deps/path_more-1e8cceb044034fbf)

running 1 test
test path_worktrees_root_and_change_worktree_resolve_from_config ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.20s

     Running tests/plan_state_more.rs (target/debug/deps/plan_state_more-5353e3be3e8ea721)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph_smoke.rs (target/debug/deps/ralph_smoke-92469ed76d149f4f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/serve_more.rs (target/debug/deps/serve_more-1f71060fb81d761e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show_specs_bundle.rs (target/debug/deps/show_specs_bundle-70fb160f2eeb6046)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/show_specs_remote_mode.rs (target/debug/deps/show_specs_remote_mode-5fa7552010144487)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/source_file_size.rs (target/debug/deps/source_file_size-a7080b328c132c9d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-0d4b9f631759b0d6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_more.rs (target/debug/deps/tasks_more-b7d7c0d6dad51920)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/tasks_remote_mode.rs (target/debug/deps/tasks_remote_mode-e5b4156ce931f306)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_schemas_export.rs (target/debug/deps/templates_schemas_export-51dce4828d63c6e5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/trace_more.rs (target/debug/deps/trace_more-0a0264026e6b215f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/update_smoke.rs (target/debug/deps/update_smoke-1406c2447b2bab42)

running 2 tests
test update_refreshes_opencode_plugin_and_preserves_user_config ... ok
test update_preserves_project_config_and_project_md ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.08s

     Running tests/user_guidance_injection.rs (target/debug/deps/user_guidance_injection-0d0e4f282997dfba)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/validate_more.rs (target/debug/deps/validate_more-9f3775453f1e4c72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/view_proposal.rs (target/debug/deps/view_proposal-0fca87d4c168555a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

```
