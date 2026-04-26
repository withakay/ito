# Task 3.1: Final Validation Gate

*2026-04-25T23:02:27Z by Showboat 0.6.1*
<!-- showboat-id: ec221f37-bd51-45da-b6cd-6b739568a9ca -->

Captured the final gate runs. Strict change validation and the full workspace test suite pass; make check now fails only on environment and baseline repository issues outside this change.

```bash
ito validate 001-33_enhance-spec-driven-workflow-validation --strict
```

```output
Change '001-33_enhance-spec-driven-workflow-validation' is valid
```

```bash
cargo test --workspace
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.31s
     Running unittests src/lib.rs (target/debug/deps/ito_backend-29787de16f653de3)

running 27 tests
test auth::tests::derive_project_token_is_64_hex_chars ... ok
test auth::tests::derive_project_token_is_deterministic ... ok
test auth::tests::derive_project_token_differs_by_project ... ok
test auth::tests::derive_project_token_differs_by_seed ... ok
test auth::tests::exempt_paths_are_health_and_ready ... ok
test auth::tests::extract_org_repo_non_project_path ... ok
test auth::tests::extract_org_repo_no_trailing ... ok
test auth::tests::extract_org_repo_valid_path ... ok
test auth::tests::token_scope_serializes_admin ... ok
test auth::tests::token_scope_serializes_project ... ok
test auth::tests::validate_token_admin_matches ... ok
test auth::tests::validate_token_invalid_fails ... ok
test auth::tests::validate_token_project_matches ... ok
test auth::tests::validate_token_wrong_project_fails ... ok
test error::tests::api_error_serializes_to_json_with_error_and_code ... ok
test error::tests::bad_request_response_has_400_status ... ok
test error::tests::core_not_found_maps_to_404 ... ok
test error::tests::core_validation_maps_to_400 ... ok
test error::tests::forbidden_response_has_403_status ... ok
test error::tests::internal_response_has_500_status ... ok
test error::tests::into_response_produces_json_content_type ... ok
test error::tests::not_found_response_has_404_status ... ok
test error::tests::service_unavailable_response_has_503_status ... ok
test error::tests::unauthorized_response_has_401_status ... ok
test state::tests::ito_path_for_rejects_path_traversal ... ok
test state::tests::ito_path_for_resolves_to_expected_path ... ok
test state::tests::ensure_project_dir_creates_directories ... ok

test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/archive_sync.rs (target/debug/deps/archive_sync-869eb0677722afd2)

running 3 tests
test sync_pull_returns_artifact_bundle ... ok
test sync_push_updates_backend_artifacts ... ok
test archive_endpoint_promotes_specs_and_moves_change ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/bootstrap_endpoints.rs (target/debug/deps/bootstrap_endpoints-2857bd391f027137)

running 9 tests
test health_endpoint_does_not_require_auth ... ok
test health_endpoint_returns_status_and_version ... ok
test ready_endpoint_returns_ready_when_data_dir_exists ... ok
test ready_endpoint_does_not_require_auth ... ok
test project_route_rejects_non_allowlisted_org ... ok
test project_route_rejects_missing_token ... ok
test project_route_accepts_derived_project_token ... ok
test project_route_rejects_invalid_token ... ok
test project_route_accepts_admin_token ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/event_ingest.rs (target/debug/deps/event_ingest-c500ccaf26eff398)

running 6 tests
test ingest_requires_authentication ... ok
test ingest_empty_batch_accepted ... ok
test ingest_missing_idempotency_key_rejected ... ok
test ingest_accepts_event_batch ... ok
test ingest_idempotent_retry_returns_duplicates ... ok
test list_events_returns_backend_managed_audit_log ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/multi_tenant.rs (target/debug/deps/multi_tenant-e72b663696a4a015)

running 13 tests
test non_allowlisted_repo_in_allowed_org_is_rejected ... ok
test derived_token_for_project_b_cannot_access_project_a ... ok
test get_change_tasks_returns_task_list ... ok
test derived_token_for_project_a_cannot_access_project_b ... ok
test get_nonexistent_change_returns_404 ... ok
test modules_are_isolated_between_projects ... ok
test get_single_module_returns_detail ... ok
test admin_token_lists_changes_for_project_a ... ok
test get_nonexistent_module_returns_404 ... ok
test get_single_change_returns_detail ... ok
test derived_token_for_project_a_accesses_project_a ... ok
test admin_token_lists_changes_for_project_b ... ok
test events_are_isolated_between_projects ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s

     Running tests/specs.rs (target/debug/deps/specs-ad3613eac0c4855f)

running 2 tests
test get_spec_returns_markdown ... ok
test list_specs_returns_promoted_specs ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

     Running tests/task_mutations.rs (target/debug/deps/task_mutations-7c5dc160fccf6660)

running 5 tests
test start_task_endpoint_reports_missing_tasks_as_not_found ... ok
test tasks_markdown_endpoint_returns_none_for_missing_artifact ... ok
test complete_task_endpoint_accepts_note_payload ... ok
test shelve_task_endpoint_accepts_reason_payload ... ok
test start_task_endpoint_updates_remote_tasks ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

     Running unittests src/main.rs (target/debug/deps/ito-ba91895743956f22)

running 65 tests
test app::archive::tests::archive_follow_up_messages_cover_all_modes ... ok
test app::archive::tests::only_filesystem_mode_requires_local_changes_dir ... ok
test app::instructions::tests::collect_tracking_diagnostic_counts_empty_slice ... ok
test app::instructions::tests::collect_context_files_preserves_order ... ok
test app::instructions::tests::collect_tracking_diagnostic_counts_none_input ... ok
test app::instructions::tests::json_get_empty_keys_returns_root ... ok
test app::instructions::tests::collect_tracking_diagnostic_counts_mixed_levels ... ok
test app::instructions::tests::json_get_returns_none_for_missing_key ... ok
test app::instructions::tests::json_get_returns_none_for_non_object_intermediate ... ok
test app::instructions::tests::json_get_traverses_nested_keys ... ok
test app::instructions::tests::worktree_config_checkout_siblings_sets_project_root ... ok
test app::instructions::tests::worktree_config_checkout_subdir_sets_project_root ... ok
test app::instructions::tests::worktree_config_defaults_when_no_worktrees_key ... ok
test app::instructions::tests::backend_instruction_is_cli_first_for_remote_mode ... ok
test app::instructions::tests::worktree_config_ignores_empty_strings ... ok
test app::instructions::tests::worktree_config_no_project_root_when_none_passed ... ok
test app::instructions::tests::worktree_config_parses_all_fields ... ok
test app::instructions::tests::worktree_config_parses_bare_control_siblings_strategy ... ok
test app::list::tests::format_relative_time_covers_major_buckets ... ok
test app::list::tests::format_task_status_handles_various_states ... ok
test app::list::tests::parse_sort_order_supports_separate_and_equals_forms ... ok
test app::run::tests::removed_serve_api_replacement_preserves_flags_and_args ... ok
test cli::ralph::ralph_tests::harness_arg_converts_to_core_harness_name ... ok
test commands::config::config_tests::json_render_value_renders_common_json_types ... ok
test commands::backend::tests::resolve_project_root_rejects_parentless_paths ... ok
test cli::cli_tests::parses_top_level_sync_command ... ok
test commands::backend::tests::resolve_project_root_returns_parent_directory ... ok
test app::worktree_wizard::worktree_wizard_tests::is_worktree_configured_detects_strategy_key ... ok
test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_errors_when_enabled_missing_fields ... ok
test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_on_empty_ip ... ok
test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_enabled_settings ... ok
test cli::cli_tests::parses_top_level_sync_force_flag ... ok
test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_when_command_missing ... ok
test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_success ... ok
test app::worktree_wizard::worktree_wizard_tests::load_worktree_result_from_config_returns_expected_defaults_and_values ... ok
test commands::serve::serve_tests::detect_tailscale_ip_with_cmd_errors_on_non_zero_exit ... ok
test commands::serve_api::serve_api_tests::builds_allowlist_from_allow_org_args ... ok
test commands::serve_api::serve_api_tests::builds_config_with_defaults ... ok
test commands::serve_api::serve_api_tests::merge_allow_orgs_preserves_existing_repo_rules ... ok
test commands::config::config_tests::config_schema_includes_coordination_sync_interval_default ... ok
test diagnostics::tests::blocking_task_error_message_includes_rendered_errors ... ok
test commands::config::config_tests::config_schema_includes_archive_main_integration_mode_default ... ok
test diagnostics::tests::blocking_task_error_message_returns_none_when_no_errors ... ok
test diagnostics::tests::format_path_line_includes_optional_line_number ... ok
test diagnostics::tests::render_validation_issues_renders_level_path_and_message ... ok
test util::tests::command_id_maps_x_templates_to_templates ... ok
test diagnostics::tests::render_task_diagnostics_filters_by_level_and_renders_task_id_when_present ... ok
test util::tests::command_id_maps_gr_to_grep ... ok
test util::tests::command_id_uses_positional_args_and_normalizes_hyphens ... ok
test util::tests::sanitize_args_redacts_equals_form ... ok
test util::tests::sanitize_args_replaces_paths ... ok
test util::tests::sanitize_args_redacts_sensitive_flags ... ok
test util::tests::split_csv_trims_parts ... ok
test commands::serve::serve_tests::ensure_ito_dir_exists_ok_when_present ... ok
test app::worktree_wizard::worktree_wizard_tests::save_worktree_config_writes_config_and_runs_print_paths ... ok
test commands::serve::serve_tests::ensure_ito_dir_exists_errors_when_missing ... ok
test app::worktree_wizard::worktree_wizard_tests::persist_worktree_config_writes_disabled_and_preserves_other_keys ... ok
test commands::serve::serve_tests::ensure_ito_dir_exists_errors_when_path_is_file ... ok
test commands::serve_api::serve_api_tests::load_backend_server_config_file_accepts_full_ito_json_config ... ok
test commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_trailing_json_content ... ok
test commands::serve_api::serve_api_tests::load_backend_server_config_file_rejects_unknown_json_fields ... ok
test commands::serve_api::serve_api_tests::load_backend_server_config_file_reads_toml ... ok
test commands::config::config_tests::handle_config_schema_writes_file_when_output_is_set ... ok
test app::list::tests::progress_filter_flags_are_mutually_exclusive ... ok
test app::instructions::tests::worktree_config_bare_control_siblings_calls_resolve ... ok

test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/agent_instruction_bootstrap.rs (target/debug/deps/agent_instruction_bootstrap-fe91dd32cdfef954)

running 9 tests
test bootstrap_rejects_invalid_tool ... ok
test bootstrap_opencode_success ... ok
test bootstrap_contains_artifact_pointers ... ok
test bootstrap_requires_tool_flag ... ok
test bootstrap_claude_success ... ok
test bootstrap_github_copilot_success ... ok
test bootstrap_output_is_short ... ok
test bootstrap_json_output ... ok
test bootstrap_codex_success ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.65s

     Running tests/agent_instruction_context.rs (target/debug/deps/agent_instruction_context-134ecd776a2cd2fc)

running 2 tests
Switched to a new branch '023-07_harness-context-inference'
test agent_instruction_context_prefers_path_inference_in_text_output ... ok
test agent_instruction_context_supports_json_output ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s

     Running tests/agent_instruction_memory.rs (target/debug/deps/agent_instruction_memory-b9912b209d2130a9)

running 14 tests
test agent_instruction_help_lists_memory_artifacts ... ok
test memory_search_requires_query_flag ... ok
test memory_query_renders_not_configured_when_only_capture_set ... ok
test memory_query_not_configured_branch_renders_setup_guidance ... ok
test memory_capture_skill_branch_emits_structured_inputs ... ok
test memory_capture_not_configured_branch_renders_setup_guidance ... ok
test memory_capture_renders_skill_when_only_capture_configured ... ok
test memory_capture_command_branch_renders_executable_command_line ... ok
test memory_query_skill_branch_emits_structured_inputs ... ok
test memory_search_not_configured_branch_renders_setup_guidance ... ok
test memory_search_command_branch_overrides_limit_when_supplied ... ok
test memory_search_command_branch_substitutes_query_and_default_limit ... ok
test memory_search_skill_branch_emits_structured_inputs ... ok
test memory_query_command_branch_substitutes_query ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.79s

     Running tests/agent_instruction_orchestrate.rs (target/debug/deps/agent_instruction_orchestrate-c9a9935cd0ff6e43)

running 5 tests
test orchestrate_requires_orchestrate_md ... ok
test orchestrate_surfaces_recommended_skills_from_preset ... ok
test orchestrate_tolerates_trailing_whitespace_in_front_matter_delimiter ... ok
test orchestrate_json_output_has_correct_artifact_id ... ok
test orchestrate_succeeds_when_orchestrate_md_exists ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/agent_instruction_repo_sweep.rs (target/debug/deps/agent_instruction_repo_sweep-47f9998af6ec2353)

running 3 tests
test repo_sweep_json_output_has_correct_artifact_id ... ok
test repo_sweep_output_contains_key_phrases ... ok
test repo_sweep_succeeds_without_change_flag ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/agent_instruction_worktrees.rs (target/debug/deps/agent_instruction_worktrees-633166f157723f0b)

running 2 tests
test worktrees_instruction_does_not_require_change ... ok
test worktrees_instruction_json_output ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/aliases.rs (target/debug/deps/aliases-39f60ae5845e310b)

running 4 tests
test subcommand_aliases_work ... ok
test main_command_aliases_work ... ok
test main_command_aliases_execute ... ok
test short_flags_work ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

     Running tests/archive_completed.rs (target/debug/deps/archive_completed-4c7aae7613d7cf5f)

running 7 tests
test archive_completed_conflict_with_positional ... ok
test archive_completed_no_completed_changes ... ok
test archive_completed_decline_confirmation_cancels ... ok
test archive_completed_empty_confirmation_cancels ... ok
test archive_completed_accept_yes_confirmation_archives ... ok
test archive_completed_skip_specs ... ok
test archive_completed_archives_all_completed ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.39s

     Running tests/archive_remote_mode.rs (target/debug/deps/archive_remote_mode-a5ee42fd29c35d09)

running 1 test
test remote_archive_succeeds_without_local_active_change_markdown ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

     Running tests/archive_smoke.rs (target/debug/deps/archive_smoke-54d60c8ef0499b04)

running 1 test
test archive_with_specs_and_validation_smoke ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

     Running tests/audit_more.rs (target/debug/deps/audit_more-f99b3f75c50162df)

running 6 tests
test audit_more_local_audit_writes_warn_and_fallback_without_worktree_log_when_branch_storage_is_unavailable ... ok
test audit_log_stats_and_validate_json_outputs_are_well_formed ... ok
test audit_more_local_audit_writes_use_internal_branch_without_worktree_log_churn ... ok
test audit_subcommands_cover_text_output_limit_reconcile_and_stream ... ok
test audit_stream_all_worktrees_dedupes_shared_routed_storage ... ok
test audit_commands_migrate_legacy_worktree_log_into_routed_storage ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.23s

     Running tests/audit_remote_mode.rs (target/debug/deps/audit_remote_mode-3612884985bb8e8d)

running 1 test
test audit_commands_in_backend_mode_use_server_only_storage ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.31s

     Running tests/backend_import.rs (target/debug/deps/backend_import-08e167b0686fa27d)

running 4 tests
test backend_import_rejects_local_mode ... ok
test backend_import_writes_active_and_archived_changes_to_backend ... ok
test backend_import_dry_run_reports_scope_without_writing_backend ... ok
test backend_import_is_idempotent_and_remote_reads_match_imported_changes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.34s

     Running tests/backend_qa_walkthrough.rs (target/debug/deps/backend_qa_walkthrough-669695876df2e7d0)

running 1 test
test backend_qa_script_verify_runs_end_to_end ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.72s

     Running tests/backend_serve.rs (target/debug/deps/backend_serve-7a34bb541ee1074d)

running 5 tests
test backend_serve_service_mode_reports_malformed_backend_config ... ok
test backend_serve_reports_unknown_fields_in_explicit_config_file ... ok
test backend_serve_init_prints_backend_command_guidance ... ok
test backend_serve_service_mode_reuses_existing_auth_without_printing_init_output ... ok
test backend_serve_service_mode_bootstraps_missing_auth_silently ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

     Running tests/backend_status_more.rs (target/debug/deps/backend_status_more-0e857496d9c79d11)

running 20 tests
test backend_status_disabled_shows_informational_output ... ok
test backend_status_incomplete_config_fails ... ok
test backend_status_unreachable_server_fails ... ok
test backend_status_with_valid_config_but_no_server ... ok
test generate_token_missing_org_fails ... ok
test generate_token_derives_deterministic_token ... ok
test generate_token_seed_from_env_takes_precedence ... ok
test backend_status_json_includes_config_details ... ok
test backend_status_disabled_json_output ... ok
test backend_status_token_security_warning ... ok
test generate_token_with_all_sources_prefers_env ... ok
test backend_status_unreachable_server_json_output ... ok
test generate_token_missing_repo_fails ... ok
test generate_token_no_seed_fails ... ok
test generate_token_flag_overrides_for_org_repo ... ok
test backend_status_with_env_token_no_warning ... ok
test silent_fallback_grep_warns_on_bad_config ... ok
test silent_fallback_with_valid_backend_no_warnings ... ok
test silent_fallback_tasks_warns_on_bad_config ... ok
test silent_fallback_event_forwarding_warns_on_bad_config ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.14s

     Running tests/cli_smoke.rs (target/debug/deps/cli_smoke-b7ce74a5475e86b4)

running 6 tests
test cli_help_hides_top_level_serve_api_entrypoint ... ok
test cli_top_level_serve_api_help_shows_backend_migration_guidance ... ok
test cli_top_level_serve_api_shows_backend_migration_guidance ... ok
test agent_instruction_status_archive_smoke ... ok
test list_show_validate_smoke ... ok
test create_workflow_plan_state_config_smoke ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.76s

     Running tests/cli_snapshots.rs (target/debug/deps/cli_snapshots-ba5c3c13ecc44770)

running 14 tests
test snapshot_list_help ... ok
test snapshot_tasks_help ... ok
test snapshot_version ... ok
test snapshot_ralph_help ... ok
test snapshot_create_help ... ok
test snapshot_validate_help ... ok
test snapshot_help ... ok
test snapshot_help_all_global_flag ... ok
test snapshot_agent_instruction_help ... ok
test snapshot_init_help ... ok
test snapshot_backend_serve_help ... ok
test snapshot_backend_help ... ok
test snapshot_agent_help ... ok
test snapshot_help_all_subcommand ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/config_more.rs (target/debug/deps/config_more-e04f1f54ca5a663a)

running 5 tests
test config_unknown_subcommand_errors ... ok
test config_set_rejects_invalid_audit_mirror_branch_name ... ok
test config_set_rejects_invalid_coordination_branch_name ... ok
test config_help_path_list_unset_and_schema_smoke ... ok
test config_set_get_supports_coordination_and_audit_mirror_keys ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.52s

     Running tests/coverage_smoke.rs (target/debug/deps/coverage_smoke-b4442509aba625f6)

running 3 tests
test serve_errors_when_no_ito_dir_exists ... ok
test completions_command_runs_for_all_shells ... ok
test audit_validate_and_log_work_with_empty_event_log ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/create_more.rs (target/debug/deps/create_more-5be268ce3e70508b)

running 4 tests
test create_change_sub_module_and_module_are_mutually_exclusive ... ok
test create_change_sub_module_rejects_remote_persistence_mode ... ok
test create_change_with_sub_module_flag_creates_composite_id_change ... ok
test create_module_and_change_error_paths_and_outputs ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.51s

     Running tests/grep_more.rs (target/debug/deps/grep_more-25fa575e2b29f0d3)

running 5 tests
test grep_change_scope_rejects_too_many_positional_args ... ok
test grep_change_scope_prints_matches_with_locations ... ok
test grep_all_scope_searches_all_changes ... ok
test grep_limit_caps_output_and_prints_warning ... ok
test grep_module_scope_searches_all_changes_in_module ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

     Running tests/help.rs (target/debug/deps/help-50ad8105f838ccfd)

running 7 tests
test agent_instruction_help_shows_instruction_details ... ok
test help_prints_usage ... ok
test help_all_global_flag_works ... ok
test dash_h_help_matches_dash_dash_help ... ok
test help_shows_navigation_footer ... ok
test help_all_shows_complete_reference ... ok
test help_all_json_outputs_valid_json ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/init_coordination.rs (target/debug/deps/init_coordination-3c6b789b966a83e6)

running 4 tests
test init_no_coordination_worktree_writes_embedded_storage ... ok
test init_without_git_remote_falls_back_gracefully ... ok
test init_upgrade_does_not_touch_coordination_storage ... ok
test init_with_git_remote_creates_coordination_worktree ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.42s

     Running tests/init_gitignore_session_json.rs (target/debug/deps/init_gitignore_session_json-3c2dccfbb03431c2)

running 1 test
test init_writes_gitignore_session_json_and_is_idempotent ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/init_more.rs (target/debug/deps/init_more-cbf21b78c5b065fa)

running 28 tests
test init_interactive_detects_tools_and_installs_adapter_files ... ignored, PTY interactive test hangs in CI; run locally with --include-ignored
test init_help_prints_usage ... ok
test init_requires_tools_when_non_interactive ... ok
test init_refuses_to_overwrite_existing_file_without_markers_when_not_forced ... ok
test init_renders_agents_md_without_raw_jinja2_syntax ... ok
test init_prints_project_setup_nudge_when_marker_incomplete ... ok
test init_opencode_installs_audit_hook_plugin ... ok
test init_github_copilot_installs_audit_preflight_assets ... ok
test init_renders_skill_files_without_raw_jinja2_syntax ... ok
test init_codex_installs_audit_instruction_assets ... ok
test init_tools_csv_ignores_empty_segments ... ok
test init_force_overwrites_existing_user_prompt_stubs ... ok
test init_does_not_print_project_setup_nudge_when_marker_complete ... ok
test init_does_not_print_project_setup_nudge_when_marker_absent ... ok
test init_update_does_not_overwrite_existing_user_prompt_stubs ... ok
test init_update_without_prior_init_creates_all_files ... ok
test init_with_tools_none_installs_ito_skeleton ... ok
test init_with_tools_opencode_installs_orchestrator_agent_template ... ok
test init_tools_parser_covers_all_and_invalid_id ... ok
test init_with_tools_csv_installs_selected_adapters ... ok
test init_writes_config_with_release_tag_schema_reference ... ok
test init_upgrade_refreshes_marker_managed_block_and_preserves_user_content ... ok
test init_update_renders_agents_md_without_raw_jinja2 ... ok
test init_update_preserves_user_files_and_creates_missing ... ok
test init_setup_coordination_branch_fails_without_origin_remote ... ok
test init_setup_coordination_branch_reports_ready_when_already_present ... ok
test init_setup_coordination_branch_creates_branch_on_origin ... ok
test init_setup_coordination_branch_uses_configured_branch_name ... ok

test result: ok. 27 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.80s

     Running tests/init_tmux.rs (target/debug/deps/init_tmux-63bdc980e33c8df8)

running 5 tests
test init_interactive_can_disable_tmux_preference ... ignored, PTY interactive test hangs in CI; run locally with --include-ignored
test init_writes_tmux_enabled_true_by_default ... ok
test init_with_no_tmux_writes_tmux_enabled_false ... ok
test init_update_preserves_existing_tmux_preference ... ok
test init_uses_cascading_tmux_preference_from_global_config ... ok

test result: ok. 4 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running tests/init_upgrade_more.rs (target/debug/deps/init_upgrade_more-8997143db7f5fc26)

running 5 tests
test init_upgrade_skips_and_warns_when_markers_missing ... ok
test init_upgrade_flag_is_accepted ... ok
test init_update_does_not_error_on_existing_agents_md_without_markers ... ok
test init_upgrade_refreshes_marker_managed_block_and_preserves_user_content ... ok
test init_update_preserves_user_owned_files ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

     Running tests/instructions_more.rs (target/debug/deps/instructions_more-cf06fcdbb6bd38ff)

running 14 tests
test agent_instruction_review_requires_change_flag ... ok
test agent_instruction_proposal_without_change_prints_new_proposal_guide ... ok
test agent_instruction_apply_text_is_compact_and_has_trailing_newline ... ok
test agent_instruction_proposal_honors_testing_policy_override ... ok
test agent_instruction_change_flag_reports_ambiguous_target ... ok
test agent_instruction_proposal_without_change_supports_json_output ... ok
test agent_instruction_archive_with_invalid_change_fails ... ok
test agent_instruction_change_flag_supports_shorthand ... ok
test agent_instruction_text_output_renders_artifact_envelope ... ok
test agent_instruction_finish_with_change_prompts_for_archive ... ok
test agent_instruction_archive_with_change_prints_targeted_instruction ... ok
test agent_instruction_change_flag_supports_slug_query ... ok
test agent_instruction_archive_without_change_prints_generic_guidance ... ok
test agent_instruction_review_renders_review_template ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.32s

     Running tests/list_archive.rs (target/debug/deps/list_archive-59f2114a24fdc573)

running 3 tests
test list_archive_reports_empty_archives ... ok
test list_archive_json_lists_archived_changes_only ... ok
test list_archive_lists_archived_changes_only ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/list_regression.rs (target/debug/deps/list_regression-33a0e28dc42535b4)

running 3 tests
test list_default_text_and_json_shape_regression ... ok
test list_sort_regression ... ok
test list_filters_regression ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

     Running tests/misc_more.rs (target/debug/deps/misc_more-914a1b9ea8cda8da)

running 16 tests
test archive_prompts_on_incomplete_tasks_and_proceeds_when_confirmed ... ignored, PTY interactive test — can hang in CI; run with --ignored locally
test list_errors_when_ito_changes_dir_missing ... ok
test plan_status_errors_when_roadmap_missing ... ok
test list_modules_empty_prints_hint ... ok
test status_change_flag_not_found_shows_suggestions ... ok
test status_change_flag_supports_module_scoped_slug_query ... ok
test status_change_flag_reports_ambiguous_target ... ok
test status_schema_not_found_includes_available_schemas ... ok
test show_unknown_item_offers_suggestions ... ok
test status_missing_change_flag_lists_available_changes ... ok
test list_specs_empty_prints_sentence_even_for_json ... ok
test git_env_vars_do_not_override_runtime_root_detection ... ok
test commands_run_from_nested_dir_use_git_worktree_root ... ok
test status_change_flag_supports_shorthand_and_partial_match ... ok
test show_module_errors_and_json_not_implemented ... ok
test show_spec_json_filters_and_requirement_index_errors ... ok

test result: ok. 15 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.20s

     Running tests/new_more.rs (target/debug/deps/new_more-60c2eae34fde18c2)

running 1 test
test new_change_covers_happy_and_error_paths ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s

     Running tests/parity_help_version.rs (target/debug/deps/parity_help_version-475b89bcc564a164)

running 2 tests
test version_prints_workspace_version ... ok
test help_prints_usage ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_tasks.rs (target/debug/deps/parity_tasks-95dfdd4c716f154e)

running 2 tests
test parity_tasks_init_writes_same_file ... ok
test parity_tasks_status_next_start_complete_match_oracle ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

     Running tests/path_more.rs (target/debug/deps/path_more-442409cfff21b84e)

running 8 tests
test path_missing_subcommand_errors ... ok
test path_errors_in_bare_repo ... ok
test path_worktrees_root_requires_worktrees_enabled ... ok
test path_roots_text_renders_worktree_fields_when_available ... ok
test path_worktree_requires_a_selector_flag ... ok
test path_roots_json_includes_worktree_fields_when_enabled ... ok
test path_worktrees_root_and_change_worktree_resolve_from_config ... ok
test path_roots_are_absolute_in_initialized_repo ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.55s

     Running tests/plan_state_more.rs (target/debug/deps/plan_state_more-c254b2b1ab12e61b)

running 3 tests
test plan_status_fails_without_roadmap ... ok
test plan_init_creates_structure ... ok
test plan_status_succeeds_after_init ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

     Running tests/ralph_smoke.rs (target/debug/deps/ralph_smoke-307ed12c38db2681)

running 26 tests
test ralph_change_flag_supports_shorthand_resolution ... ok
test ralph_file_flag_allowed_without_change_or_module ... ok
test ralph_file_flag_runs_without_change_or_module ... ok
test ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains ... ok
test ralph_change_flag_supports_slug_query_resolution ... ok
test ralph_file_flag_requires_readable_file ... ok
test ralph_continue_ready_exits_successfully_when_all_changes_complete ... ok
test ralph_interactive_prompts_and_runs_selected_changes_sequentially ... ok
test ralph_no_interactive_without_target_returns_clear_error ... ok
test ralph_markdown_prd_source_marks_first_pending_task_complete ... ok
test ralph_interactive_status_prompts_for_exactly_one_change ... ok
test ralph_unknown_harness_returns_clear_error ... ok
test ralph_yaml_source_marks_first_pending_task_complete ... ok
[main (root-commit) 83bc6ca] init
 6 files changed, 35 insertions(+)
 create mode 100644 .ito/changes/000-01_test-change/proposal.md
 create mode 100644 .ito/changes/000-01_test-change/tasks.md
 create mode 100644 .ito/modules/000_ungrouped/module.md
 create mode 100644 .ito/specs/alpha/spec.md
 create mode 100644 PRD.md
 create mode 100644 README.md
[main (root-commit) 83bc6ca] init
 6 files changed, 35 insertions(+)
 create mode 100644 .ito/changes/000-01_test-change/proposal.md
 create mode 100644 .ito/changes/000-01_test-change/tasks.md
 create mode 100644 .ito/modules/000_ungrouped/module.md
 create mode 100644 .ito/specs/alpha/spec.md
 create mode 100644 PRD.md
 create mode 100644 README.md
[main (root-commit) f96430a] init
 6 files changed, 38 insertions(+)
 create mode 100644 .ito/changes/000-01_test-change/proposal.md
 create mode 100644 .ito/changes/000-01_test-change/tasks.md
 create mode 100644 .ito/modules/000_ungrouped/module.md
 create mode 100644 .ito/specs/alpha/spec.md
 create mode 100644 README.md
 create mode 100644 tasks.yaml
test ralph_accepts_new_harness_names_for_status_flow ... ok
[main (root-commit) 83bc6ca] init
 6 files changed, 35 insertions(+)
 create mode 100644 .ito/changes/000-01_test-change/proposal.md
 create mode 100644 .ito/changes/000-01_test-change/tasks.md
 create mode 100644 .ito/modules/000_ungrouped/module.md
 create mode 100644 .ito/specs/alpha/spec.md
 create mode 100644 PRD.md
 create mode 100644 README.md
[main (root-commit) 8fb3b7f] init
 6 files changed, 44 insertions(+)
 create mode 100644 .ito/changes/000-01_test-change/proposal.md
 create mode 100644 .ito/changes/000-01_test-change/tasks.md
 create mode 100644 .ito/modules/000_ungrouped/module.md
 create mode 100644 .ito/specs/alpha/spec.md
 create mode 100644 README.md
 create mode 100644 tasks.yaml
test ralph_branch_per_task_requires_clean_worktree ... ok
test ralph_stub_harness_writes_state_and_status_works ... ok
test ralph_github_source_closes_issue_on_success ... ok
test ralph_branch_per_task_creates_task_branch_for_prd_source ... ok
To /var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpXrjyrR
 * [new branch]      main -> main
branch 'main' set up to track 'origin/main'.
test ralph_notify_emits_operator_notification_on_success ... ok
test ralph_parallel_yaml_source_completes_grouped_tasks ... ok
test ralph_sync_issue_updates_prd_back_to_github_issue ... ok
test ralph_interactive_options_wizard_exit_on_error_stops_on_nonzero_harness_exit ... ok
test ralph_browser_flag_injects_agent_browser_guidance_for_opencode ... ok
test ralph_create_pr_uses_base_branch_and_fake_gh ... ok
test ralph_interactive_options_wizard_prompts_for_missing_values_and_applies_them ... ok
test ralph_parallel_preserves_worker_code_changes ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.50s

     Running tests/serve_more.rs (target/debug/deps/serve_more-2ba1c2c039fe0bc3)

running 1 test
test serve_errors_when_not_initialized ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/show_specs_bundle.rs (target/debug/deps/show_specs_bundle-3861ffdbef443f33)

running 2 tests
test show_specs_bundles_truth_specs_as_markdown_with_metadata ... ok
test show_specs_bundles_truth_specs_as_json_with_absolute_paths ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/show_specs_remote_mode.rs (target/debug/deps/show_specs_remote_mode-60aed8656ea96c51)

running 1 test
test show_specs_reads_backend_specs_without_local_markdown ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

     Running tests/source_file_size.rs (target/debug/deps/source_file_size-97cb4269c9543244)

running 1 test
test ito_cli_source_files_are_reasonably_sized ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-702a7d80ac59558b)

running 1 test
test stats_counts_command_end_events ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/tasks_more.rs (target/debug/deps/tasks_more-a608438466da4cc1)

running 11 tests
test tasks_status_rejects_free_form_with_more_than_two_numbers ... ok
test tasks_status_resolves_short_change_id ... ok
test tasks_status_resolves_free_form_two_numbers ... ok
test tasks_json_lists_are_sorted_by_task_id ... ok
test tasks_commands_use_apply_tracks_filename_when_set ... ok
test tasks_complete_supports_checkbox_compat_mode ... ok
test tasks_error_paths_cover_more_branches ... ok
test tasks_start_supports_checkbox_compat_mode_and_enforces_single_in_progress ... ok
test tasks_next_supports_checkbox_compat_mode_and_shows_current_or_next ... ok
test tasks_add_shelve_unshelve_show_cover_more_paths ... ok
test tasks_commands_support_json_output ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.06s

     Running tests/tasks_remote_mode.rs (target/debug/deps/tasks_remote_mode-c2bcb0e75d4c5a2b)

running 2 tests
test remote_missing_tasks_commands_do_not_hard_fail ... ok
test remote_task_start_updates_backend_without_local_tasks_file ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.37s

     Running tests/templates_schemas_export.rs (target/debug/deps/templates_schemas_export-01e57391d9a306ab)

running 3 tests
test templates_help_includes_schemas_export ... ok
test templates_schemas_export_writes_embedded_files ... ok
test templates_schemas_export_skips_without_force_then_overwrites_with_force ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/trace_more.rs (target/debug/deps/trace_more-928ea741ccabc759)

running 8 tests
test trace_missing_change_exits_nonzero ... ok
test trace_legacy_checkbox_change_shows_unavailable ... ok
test trace_fully_covered_exits_zero ... ok
test trace_uncovered_requirement_json_shows_uncovered_list ... ok
test trace_unresolved_reference_shows_unresolved_in_output ... ok
test trace_partial_ids_json_shows_invalid_status ... ok
test trace_uncovered_requirement_shows_uncovered_in_output ... ok
test trace_fully_covered_json_has_ready_status ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running tests/update_marker_scoped.rs (target/debug/deps/update_marker_scoped-326e5c7ebff32a93)

running 5 tests
test update_refuses_to_overwrite_partial_marker_pair ... ok
test update_preserves_user_edits_after_end_marker_in_harness_command ... ok
test update_still_refreshes_non_markdown_manifest_assets ... ok
test update_preserves_user_edits_after_end_marker_in_harness_skill ... ok
test second_update_is_a_noop_for_harness_skills ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s

     Running tests/update_smoke.rs (target/debug/deps/update_smoke-764b27dd75e306a7)

running 8 tests
test update_preserves_project_config_and_project_md ... ok
test update_installs_adapter_files_from_local_ito_skills ... ok
test update_renders_agents_md_without_jinja2_syntax ... ok
test update_refreshes_codex_audit_instruction_assets ... ok
test update_refreshes_opencode_plugin_and_preserves_user_config ... ok
test update_preserves_user_guidance_and_user_prompt_files ... ok
test update_refreshes_github_copilot_audit_assets ... ok
test update_merges_claude_settings_without_clobbering_user_keys ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s

     Running tests/user_guidance_injection.rs (target/debug/deps/user_guidance_injection-a57b1bc612abc621)

running 3 tests
test agent_instruction_includes_user_guidance_when_present ... ok
test agent_instruction_includes_scoped_user_prompt_for_artifact ... ok
test agent_instruction_prefers_user_prompts_shared_guidance_file ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

     Running tests/validate_more.rs (target/debug/deps/validate_more-1983422fab4ce86c)

running 7 tests
test validate_type_module_special_cases_to_spec_by_id ... ok
test validate_unknown_spec_offers_suggestions ... ok
test validate_ambiguous_item_is_an_error ... ok
test validate_all_json_success_has_summary_and_by_type ... ok
test validate_module_routes_and_error_paths ... ok
test validate_all_prints_failure_report_in_text_mode ... ok
test validate_change_reports_audit_drift_against_routed_storage ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.38s

     Running tests/view_proposal.rs (target/debug/deps/view_proposal-e119097c215598ab)

running 8 tests
test view_proposal_help_shows_viewer_flag ... ok
test view_proposal_html_viewer_errors_when_pandoc_missing ... ok
test view_proposal_unknown_change_fails ... ok
test view_proposal_unknown_viewer_is_rejected ... ok
test view_proposal_disabled_tmux_is_rejected ... ok
test view_proposal_json_outputs_bundle ... ok
test view_proposal_html_viewer_is_recognized ... ok
test view_proposal_html_viewer_succeeds_with_stub_pandoc ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.44s

     Running unittests src/lib.rs (target/debug/deps/ito_common-f042c81ff62a74de)

running 59 tests
test git_url::tests::parses_gitlab_style_subgroup_takes_last_two_segments ... ok
test git_url::tests::parses_https_url_without_git_suffix ... ok
test git_url::tests::returns_none_for_bare_string_without_separator ... ok
test git_url::tests::returns_none_for_empty_string ... ok
test git_url::tests::returns_none_for_no_path_after_host ... ok
test git_url::tests::returns_none_for_scp_url_with_single_component ... ok
test git_url::tests::returns_none_for_single_path_component ... ok
test git_url::tests::returns_none_for_whitespace_only ... ok
test git_url::tests::handles_trailing_slash_in_https_url ... ok
test git_url::tests::parses_git_protocol_url ... ok
test git_url::tests::parses_scp_ssh_url ... ok
test git_url::tests::parses_https_url_with_git_suffix ... ok
test git_url::tests::parses_ssh_with_explicit_port ... ok
test id::change_id::tests::parse_change_id_allows_large_change_numbers ... ok
test git_url::tests::handles_ssh_url_without_user ... ok
test git_url::tests::parses_http_scheme ... ok
test git_url::tests::strips_git_suffix_only_once ... ok
test id::change_id::tests::parse_change_id_allows_three_digit_change_numbers ... ok
test id::change_id::tests::parse_change_id_missing_name_has_specific_error ... ok
test id::change_id::tests::parse_change_id_normalizes_excessive_padding_for_large_change_numbers ... ok
test id::change_id::tests::parse_change_id_pads_both_parts ... ok
test id::change_id::tests::parse_change_id_rejects_overlong_input ... ok
test id::change_id::tests::parse_change_id_sub_module_format_canonical ... ok
test id::change_id::tests::parse_change_id_sub_module_format_lowercases_name ... ok
test id::change_id::tests::parse_change_id_sub_module_format_pads_all_parts ... ok
test id::change_id::tests::parse_change_id_sub_module_missing_name_is_error ... ok
test id::change_id::tests::parse_change_id_sub_module_rejects_module_overflow ... ok
test id::change_id::tests::parse_change_id_sub_module_rejects_sub_overflow ... ok
test id::change_id::tests::parse_change_id_supports_extra_leading_zeros_for_change_num ... ok
test id::change_id::tests::parse_change_id_uses_specific_hint_for_wrong_separator ... ok
test id::module_id::tests::parse_module_id_pads_and_lowercases_name ... ok
test id::module_id::tests::parse_module_id_rejects_overflow ... ok
test id::module_id::tests::parse_module_id_rejects_overlong_input ... ok
test id::spec_id::tests::parse_spec_id_preserves_value ... ok
test id::spec_id::tests::parse_spec_id_rejects_path_traversal_sequences ... ok
test id::sub_module_id::tests::parse_sub_module_id_canonical_form ... ok
test id::sub_module_id::tests::parse_sub_module_id_lowercases_name ... ok
test id::sub_module_id::tests::parse_sub_module_id_pads_both_parts ... ok
test id::sub_module_id::tests::parse_sub_module_id_rejects_empty ... ok
test id::sub_module_id::tests::parse_sub_module_id_rejects_missing_dot ... ok
test id::sub_module_id::tests::parse_sub_module_id_rejects_module_overflow ... ok
test id::sub_module_id::tests::parse_sub_module_id_rejects_non_digit_module ... ok
test id::sub_module_id::tests::parse_sub_module_id_rejects_overlong_input ... ok
test id::sub_module_id::tests::parse_sub_module_id_rejects_sub_overflow ... ok
test id::sub_module_id::tests::parse_sub_module_id_strips_extra_leading_zeros ... ok
test id::sub_module_id::tests::parse_sub_module_id_with_name_suffix ... ok
test id::sub_module_id::tests::sub_module_id_display ... ok
test id::tests::classify_id_hyphen_without_underscore_is_module_change_id ... ok
test id::tests::classify_id_module_change_id ... ok
test id::tests::classify_id_module_id ... ok
test id::tests::classify_id_sub_module_change_id ... ok
test id::tests::classify_id_sub_module_id ... ok
test id::tests::looks_like_change_id_recognizes_sub_module_format ... ok
test id::tests::looks_like_change_id_requires_digits_hyphen_and_underscore ... ok
test id::tests::looks_like_module_id_is_digit_prefixed ... ok
test match_::tests::levenshtein_matches_ts_examples ... ok
test match_::tests::nearest_matches_is_stable_on_ties ... ok
test paths::tests::builders_join_expected_paths ... ok
test paths::tests::default_ito_root_is_dot_ito ... ok

test result: ok. 59 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/ito_config-dd7eee201c3f266d)

running 68 tests
test config::tests::global_config_path_prefers_xdg ... ok
test config::tests::ito_config_dir_prefers_xdg ... ok
test config::tests::load_global_ito_config_returns_defaults_when_no_file ... ok
test config::schema::tests::schema_contains_expected_sections ... ok
test config::tests::audit_mirror_defaults_exist_in_cascading_config ... ok
test config::types::coordination_storage_tests::coordination_branch_config_missing_storage_defaults_to_worktree ... ok
test config::types::coordination_storage_tests::coordination_branch_config_missing_worktree_path_is_none ... ok
test config::types::coordination_storage_tests::coordination_branch_config_worktree_path_absent_not_serialized ... ok
test config::types::coordination_storage_tests::coordination_branch_config_worktree_path_round_trips ... ok
test config::types::coordination_storage_tests::coordination_storage_default_is_worktree ... ok
test config::types::coordination_storage_tests::coordination_storage_round_trips_embedded ... ok
test config::types::coordination_storage_tests::coordination_storage_round_trips_worktree ... ok
test config::types::coordination_storage_tests::coordination_storage_serializes_embedded_as_lowercase ... ok
test config::types::coordination_storage_tests::coordination_storage_serializes_worktree_as_lowercase ... ok
test config::types::memory_tests::memory_default_is_absent_on_ito_config ... ok
test config::types::memory_tests::memory_op_config_command_variant_requires_command_field ... ok
test config::types::memory_tests::memory_op_config_skill_variant_requires_skill_field ... ok
test config::types::memory_tests::memory_op_config_unknown_kind_is_rejected ... ok
test config::types::memory_tests::memory_section_accepts_capture_only ... ok
test config::types::memory_tests::memory_section_accepts_skill_with_options ... ok
test config::types::memory_tests::memory_section_omits_absent_ops_when_serialized ... ok
test config::types::memory_tests::memory_section_round_trips_full_config ... ok
test config::types::memory_tests::memory_section_round_trips_when_absent ... ok
test config::types::memory_tests::memory_section_skill_options_are_optional ... ok
test config::tests::coordination_branch_defaults_exist_in_cascading_config ... ok
test config::types::memory_tests::memory_section_supports_mixed_per_op_shapes ... ok
test config::types::memory_tests::memory_section_unknown_op_key_is_rejected ... ok
test config::tests::tools_tmux_enabled_defaults_to_true_in_cascading_config ... ok
test config::tests::logging_invalid_commands_defaults_exist_in_cascading_config ... ok
test config::types::worktree_init_tests::worktree_init_config_default_has_empty_include_and_no_setup ... ok
test config::types::worktree_init_tests::worktree_init_config_deserializes_with_include_only ... ok
test config::types::worktree_init_tests::worktree_init_config_with_multiple_setup_deserializes ... ok
test config::types::worktree_init_tests::worktree_init_config_absent_deserializes_to_default ... ok
test config::types::worktree_init_tests::full_ito_config_with_worktree_init_round_trips ... ok
test config::types::worktree_init_tests::worktree_init_config_with_single_setup_deserializes ... ok
test config::types::worktree_init_tests::worktree_setup_config_array_deserializes ... ok
test config::types::worktree_init_tests::worktree_setup_config_is_empty_multiple_empty_vec ... ok
test config::types::worktree_init_tests::worktree_setup_config_is_empty_single_empty_string ... ok
test config::tests::legacy_worktree_default_branch_key_migrates ... ok
test config::types::worktree_init_tests::worktree_setup_config_is_not_empty_with_command ... ok
test config::types::worktree_init_tests::worktree_setup_config_multiple_round_trips ... ok
test config::types::worktree_init_tests::worktree_setup_config_single_round_trips ... ok
test config::types::worktree_init_tests::worktree_setup_config_single_string_deserializes ... ok
test config::types::worktree_init_tests::worktrees_config_init_does_not_break_existing_fields ... ok
test config::types::worktree_init_tests::worktrees_config_with_init_section_deserializes ... ok
test config::types::worktree_init_tests::worktrees_config_without_init_section_uses_defaults ... ok
test ito_dir::tests::get_ito_dir_name_defaults_to_dot_ito ... ok
test config::tests::audit_mirror_defaults_can_be_overridden ... ok
test config::tests::logging_invalid_commands_can_be_enabled ... ok
test config::tests::new_worktree_keys_take_precedence_over_legacy ... ok
test config::tests::coordination_branch_defaults_can_be_overridden ... ok
test ito_dir::tests::sanitize_rejects_path_separators_and_overlong_values ... ok
test config::tests::worktrees_config_has_defaults_in_cascading_config ... ok
test output::tests::no_color_env_set_matches_ts_values ... ok
test output::tests::resolve_ui_options_combines_sources ... ok
test output::tests::resolve_interactive_respects_cli_and_env ... ok
test config::tests::cascading_project_config_ignores_schema_ref_key ... ok
test config::tests::legacy_worktree_local_files_key_migrates ... ok
test context::tests::resolve_with_ctx_uses_explicit_config_context_paths ... ok
test config::tests::cascading_project_config_ignores_invalid_json_sources ... ok
test context::tests::resolve_with_ctx_sets_none_when_ito_dir_is_missing ... ok
test config::tests::load_global_ito_config_reads_backend_server_auth ... ok
test context::tests::resolve_with_ctx_sets_ito_path_when_directory_exists ... ok
test ito_dir::tests::invalid_repo_project_path_falls_back_to_default ... ok
test ito_dir::tests::get_ito_path_normalizes_dotdot_segments ... ok
test ito_dir::tests::dot_repo_config_overrides_repo_config ... ok
test config::tests::cascading_project_config_merges_sources_in_order_with_scalar_override ... ok
test ito_dir::tests::repo_config_overrides_global_config ... ok

test result: ok. 68 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/lib.rs (target/debug/deps/ito_core-e5d1b8dd8c5c34f3)

running 583 tests
test audit::reader::reader_tests::reads_events_from_injected_store ... ok
test audit::mirror::tests::merge_jsonl_ignores_blank_lines ... ok
test audit::mirror::tests::merge_jsonl_dedupes_and_appends_local_lines ... ok
test audit::store::tests::internal_branch_location_keys_include_branch_identity ... ok
test audit::reconcile::tests::build_file_state_from_default_tasks_md ... ok
test audit::reconcile::tests::build_file_state_uses_apply_tracks_when_set ... ok
test audit::stream::tests::default_config_has_sensible_values ... ok
test audit::reader::reader_tests::read_from_missing_file_returns_empty ... ok
test audit::reconcile::tests::reconcile_empty_log ... ok
test audit::reader::reader_tests::filter_by_entity_type ... ok
test audit::reader::reader_tests::filter_by_operation ... ok
test audit::reader::reader_tests::read_parses_valid_events ... ok
test audit::reader::reader_tests::skips_empty_lines ... ok
test audit::validate::tests::detect_timestamp_ordering_violation ... ok
test audit::validate::tests::detect_duplicate_create ... ok
test audit::validate::tests::detect_status_transition_mismatch ... ok
test audit::validate::tests::different_scopes_are_independent ... ok
test audit::validate::tests::empty_events_no_issues ... ok
test audit::validate::tests::no_issues_for_valid_sequence ... ok
test audit::worktree::tests::aggregate_empty_worktrees ... ok
test audit::worktree::tests::find_worktree_bare_excluded ... ok
test audit::worktree::tests::find_worktree_matching_branch ... ok
test audit::worktree::tests::find_worktree_multiple_returns_first_match ... ok
test audit::worktree::tests::find_worktree_no_match ... ok
test audit::worktree::tests::parse_bare_worktree_excluded ... ok
test audit::worktree::tests::parse_detached_head ... ok
test audit::worktree::tests::parse_multiple_worktrees ... ok
test audit::worktree::tests::parse_single_worktree ... ok
test audit::worktree::tests::worktree_audit_log_path_resolves ... ok
test audit::writer::tests::audit_log_path_resolves_correctly ... ok
test audit::reader::reader_tests::skips_malformed_lines ... ok
test audit::reader::reader_tests::filter_by_scope ... ok
test audit::reader::reader_tests::combined_filters ... ok
test audit::writer::tests::best_effort_returns_ok_even_on_failure ... ok
test audit::writer::tests::appends_events_to_existing_file ... ok
test backend_change_repository::tests::get_delegates_to_reader ... ok
test backend_change_repository::tests::list_complete_filters_correctly ... ok
test backend_change_repository::tests::list_incomplete_filters_correctly ... ok
test backend_change_repository::tests::list_returns_all_changes ... ok
test backend_change_repository::tests::resolve_target_ambiguous ... ok
test backend_change_repository::tests::resolve_target_exact_match ... ok
test backend_change_repository::tests::resolve_target_not_found ... ok
test audit::writer::tests::creates_directory_and_file_on_first_write ... ok
test backend_change_repository::tests::resolve_target_prefix_match ... ok
test backend_client::tests::custom_backup_dir_is_used ... ok
test backend_client::tests::default_backup_dir_uses_home ... ok
test backend_client::tests::disabled_backend_returns_none ... ok
test backend_client::tests::enabled_backend_empty_token_fails ... ok
test backend_client::tests::enabled_backend_missing_token_fails ... ok
test backend_client::tests::enabled_backend_with_env_var_token_resolves ... ok
test backend_client::tests::enabled_backend_with_explicit_token_resolves ... ok
test backend_client::tests::env_var_token_takes_precedence_over_config_token ... ok
test backend_client::tests::idempotency_key_includes_operation ... ok
test backend_client::tests::is_retriable_status_checks ... ok
test backend_client::tests::project_api_prefix_formats_correctly ... ok
test backend_client::tests::project_namespace_empty_string_falls_through_to_env ... ok
test backend_client::tests::project_namespace_env_takes_precedence_over_config ... ok
test backend_client::tests::project_namespace_from_config ... ok
test audit::writer::tests::events_deserialize_back_correctly ... ok
test backend_client::tests::project_namespace_from_env_vars ... ok
test backend_client::tests::project_namespace_missing_org_fails ... ok
test backend_client::tests::project_namespace_missing_repo_fails ... ok
test audit::writer::tests::each_line_is_valid_json ... ok
test audit::writer::tests::preserves_existing_content ... ok
test backend_coordination::tests::allocate_with_work ... ok
test backend_coordination::tests::allocate_no_work ... ok
test backend_coordination::tests::claim_success ... ok
test backend_coordination::tests::claim_conflict ... ok
test backend_coordination::tests::release_success ... ok
test backend_health::tests::backend_health_status_default_is_all_false ... ok
test backend_health::tests::backend_health_status_serializes_error_state ... ok
test backend_health::tests::backend_health_status_serializes_to_json ... ok
test backend_http::backend_http_tests::archived_task_fallback_only_treats_not_found_as_missing ... ok
test backend_coordination::tests::is_backend_unavailable_detects_process_error ... ok
test backend_http::backend_http_tests::audit_ingest_posts_can_opt_into_retries ... ok
test backend_http::backend_http_tests::get_requests_are_retried_by_default ... ok
test backend_http::backend_http_tests::optional_task_text_body_uses_empty_object_when_absent ... ok
test backend_http::backend_http_tests::optional_task_text_body_serializes_payload_when_present ... ok
test backend_http::backend_http_tests::parse_timestamp_returns_error_for_invalid_rfc3339 ... ok
test backend_http::backend_http_tests::post_requests_are_not_retried_by_default ... ok
test backend_sync::tests::backend_error_mapping_produces_correct_error_types ... ok
test backend_sync::tests::path_traversal_in_change_id_rejected ... ok
test backend_sync::tests::path_traversal_in_capability_rejected ... ok
test backend_sync::tests::pull_creates_backup ... ok
test backend_sync::tests::pull_writes_artifacts_locally ... ok
test backend_coordination::tests::archive_with_backend_skip_specs ... ok
test backend_sync::tests::push_missing_change_dir_fails ... ok
test backend_coordination::tests::archive_with_backend_happy_path ... ok
test backend_coordination::tests::archive_with_backend_backend_unavailable ... ok
test backend_task_repository::tests::checkbox_tasks_parsed_correctly ... ok
test backend_task_repository::tests::get_task_counts_from_backend ... ok
test backend_task_repository::tests::has_tasks_empty_content ... ok
test backend_task_repository::tests::has_tasks_detects_content ... ok
test backend_sync::tests::push_conflict_returns_actionable_error ... ok
test backend_task_repository::tests::missing_tasks_returns_empty ... ok
test change_repository::tests::resolve_target_includes_archive_when_requested ... ok
test backend_sync::tests::read_local_bundle_sorts_specs ... ok
test change_repository::tests::exists_and_get_work ... ok
test change_repository::tests::list_skips_archive_dir ... ok
test config::tests::is_valid_integration_mode_checks_correctly ... ok
test config::tests::is_valid_repository_mode_checks_correctly ... ok
test config::tests::is_valid_worktree_strategy_checks_correctly ... ok
test backend_sync::tests::push_sends_local_bundle ... ok
test config::tests::resolve_worktree_template_defaults_uses_defaults_when_missing ... ok
test config::tests::skill_id_resolves_returns_false_when_no_paths_exist ... ok
test config::tests::validate_config_value_accepts_archive_main_integration_mode ... ok
test config::tests::validate_config_value_accepts_positive_sync_interval ... ok
test config::tests::validate_config_value_accepts_unknown_keys ... ok
test config::tests::validate_config_value_accepts_valid_audit_mirror_branch_name ... ok
test config::tests::validate_config_value_accepts_valid_coordination_branch_name ... ok
test config::tests::validate_config_value_accepts_valid_integration_mode ... ok
test config::tests::resolve_worktree_template_defaults_reads_overrides ... ok
test config::tests::validate_config_value_accepts_valid_memory_kind ... ok
test config::tests::validate_config_value_accepts_valid_repository_mode ... ok
test config::tests::validate_config_value_accepts_valid_strategy ... ok
test config::tests::validate_config_value_rejects_empty_memory_command_template ... ok
test config::tests::validate_config_value_rejects_empty_memory_skill_id ... ok
test config::tests::validate_config_value_rejects_invalid_archive_main_integration_mode ... ok
test config::tests::validate_config_value_rejects_invalid_audit_mirror_branch_name ... ok
test config::tests::validate_config_value_rejects_invalid_coordination_branch_name ... ok
test config::tests::validate_config_value_rejects_invalid_integration_mode ... ok
test config::tests::validate_config_value_rejects_invalid_repository_mode ... ok
test config::tests::validate_config_value_rejects_invalid_strategy ... ok
test config::tests::validate_config_value_rejects_lock_suffix_in_path_segment ... ok
test config::tests::validate_config_value_rejects_memory_op_missing_required_field ... ok
test config::tests::validate_config_value_rejects_memory_op_unknown_kind ... ok
test config::tests::validate_config_value_rejects_non_string_strategy ... ok
test config::tests::validate_config_value_rejects_unknown_memory_kind ... ok
test config::tests::validate_config_value_rejects_unknown_memory_op_key ... ok
test config::tests::validate_config_value_rejects_zero_sync_interval ... ok
test config::tests::validate_memory_config_passes_when_no_skill_provider ... ok
test change_repository::tests::resolve_target_reports_ambiguity ... ok
test config::tests::validate_memory_config_passes_when_skill_resolves_in_flat_layout ... ok
test change_repository::tests::resolve_target_module_scoped_query ... ok
test config::tests::validate_memory_config_rejects_missing_skill ... ok
test coordination::tests::format_message_broken_symlinks_contains_paths_and_hint ... ok
test coordination::tests::format_message_embedded_is_none ... ok
test coordination::tests::format_message_healthy_is_none ... ok
test coordination::tests::format_message_not_wired_contains_dir_and_hint ... ok
test config::tests::validate_memory_config_passes_when_skill_resolves_in_grouped_layout ... ok
test coordination::tests::format_message_wrong_target_contains_paths_and_hint ... ok
test coordination::tests::format_message_worktree_missing_contains_path_and_hint ... ok
test coordination::tests::create_dir_link_creates_symlink ... ok
test coordination::tests::gitignore_entries_added_when_missing ... ok
test coordination::tests::create_dir_link_fails_when_dst_exists ... ok
test coordination::tests::gitignore_created_when_absent ... ok
test coordination::tests::gitignore_no_duplicates_on_second_call ... ok
test coordination::tests::health_embedded_returns_embedded ... ok
test coordination::tests::gitignore_preserves_existing_content ... ok
test coordination::tests::gitignore_skips_already_present_entries ... ok
test change_repository::tests::suggest_targets_prioritizes_slug_matches ... ok
test coordination::tests::health_missing_link_is_not_wired ... ok
test coordination::tests::health_worktree_missing_when_dir_absent ... ok
test coordination::tests::health_not_wired_when_real_dirs_present ... ok
test coordination::tests::health_broken_symlinks_when_target_missing ... ok
test coordination::tests::health_healthy_when_all_symlinks_correct ... ok
test coordination::tests::remove_is_noop_when_dirs_absent ... ok
test coordination::tests::remove_is_noop_for_real_dirs ... ok
test coordination::tests::health_wrong_target_when_symlink_points_elsewhere ... ok
test coordination::tests::wire_creates_symlinks_for_all_dirs ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_is_noop_when_nothing_staged ... ok
test coordination::tests::wire_is_idempotent ... ok
test coordination::tests::remove_restores_real_dirs_with_content ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_commit_fails ... ok
test coordination::tests::wire_handles_empty_real_dir ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_git_add_fails ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_stages_and_commits_when_changes_exist ... ok
test coordination::tests::wire_migrates_real_dir_content ... ok
test coordination_worktree::coordination_worktree_tests::create_fetches_branch_from_origin_when_not_local ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback ... ok
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_fetch_fails_unexpectedly ... ok
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_worktree_add_fails ... ok
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_orphan_commit_fails ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_when_not_on_remote ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback_in_sha256_repo ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_when_origin_not_configured ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_storage_is_embedded ... ok
test coordination_worktree::coordination_worktree_tests::create_uses_existing_local_branch ... ok
test coordination_worktree::coordination_worktree_tests::remove_falls_back_to_force_when_clean_remove_fails ... ok
test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_force_remove_also_fails ... ok
test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_prune_fails ... ok
test coordination_worktree::coordination_worktree_tests::remove_runs_worktree_remove_then_prune ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_fetches_commits_and_pushes_when_healthy ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_force_bypasses_rate_limit ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_is_noop_when_storage_is_embedded ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_rate_limits_when_recent_and_clean ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_returns_error_when_links_point_to_wrong_target ... ok
test create::create_sub_module_tests::create_sub_module_accepts_full_module_folder_name ... ok
test create::create_sub_module_tests::create_sub_module_allocates_sequential_numbers ... ok
test create::create_sub_module_tests::create_sub_module_creates_directory_and_module_md ... ok
test create::create_sub_module_tests::create_sub_module_errors_on_duplicate_name ... ok
test create::create_sub_module_tests::create_sub_module_errors_on_unknown_parent_module ... ok
test create::create_sub_module_tests::create_sub_module_rejects_invalid_name ... ok
test create::create_sub_module_tests::create_sub_module_with_description_writes_purpose ... ok
test distribution::tests::ensure_manifest_script_is_executable_only_adds_execute_bits ... ok
test distribution::tests::pi_adapter_asset_exists_in_embedded_templates ... ok
test distribution::tests::pi_agent_templates_discoverable ... ok
test distribution::tests::pi_manifests_commands_match_opencode_commands ... ok
test distribution::tests::pi_manifests_includes_adapter_skills_and_commands ... ok
test distribution::tests::pi_manifests_skills_match_opencode_skills ... ok
test errors::tests::core_error_helpers_construct_expected_variants ... ok
test event_forwarder::tests::checkpoint_missing_returns_zero ... ok
test event_forwarder::tests::checkpoint_roundtrip ... ok
test audit::reconcile::tests::reconcile_missing_tasks_file ... ok
test audit::reconcile::tests::reconcile_no_drift ... ok
test audit::reconcile::tests::reconcile_detects_drift ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_worktree_dir_does_not_exist ... ok
test event_forwarder::tests::forward_no_events_returns_zero ... ok
test audit::worktree::tests::aggregate_worktree_with_events ... ok
test event_forwarder::tests::forward_result_equality ... ok
test audit::stream::tests::poll_returns_empty_when_no_new_events ... ok
test audit::stream::tests::poll_detects_new_events ... ok
test event_forwarder::tests::forward_reports_duplicates ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_calls_auto_commit_when_worktree_mode_and_dir_exists ... ok
test event_forwarder::tests::is_retriable_backend_error_checks ... ok
test front_matter::tests::body_sha256_is_deterministic ... ok
test front_matter::tests::created_at_dt_returns_none_for_invalid_timestamp ... ok
test front_matter::tests::created_at_dt_returns_none_when_absent ... ok
test front_matter::tests::format_timestamp_produces_rfc3339 ... ok
test front_matter::tests::parse_delimiter_with_extra_text_on_first_line ... ok
test front_matter::tests::parse_empty_front_matter ... ok
test front_matter::tests::parse_invalid_yaml ... ok
test front_matter::tests::parse_no_closing_delimiter ... ok
test front_matter::tests::parse_no_front_matter ... ok
test front_matter::tests::parse_preserves_extra_fields ... ok
test front_matter::tests::parse_valid_front_matter ... ok
test front_matter::tests::parse_with_integrity ... ok
test front_matter::tests::roundtrip_write_parse ... ok
test front_matter::tests::touch_creates_new_front_matter ... ok
test front_matter::tests::touch_updates_existing ... ok
test front_matter::tests::update_integrity_sets_checksum ... ok
test front_matter::tests::validate_id_fails_on_mismatch ... ok
test front_matter::tests::validate_id_passes_when_absent ... ok
test front_matter::tests::validate_id_passes_when_matching ... ok
test front_matter::tests::validate_integrity_fails_on_mismatch ... ok
test front_matter::tests::validate_integrity_passes_when_matching ... ok
test front_matter::tests::validate_integrity_passes_when_no_checksum ... ok
test front_matter::tests::write_no_front_matter_returns_body ... ok
test fs_project_store::tests::change_repository_returns_box_trait ... ok
test fs_project_store::tests::ensure_project_creates_directory ... ok
test fs_project_store::tests::ito_path_rejects_path_traversal ... ok
test fs_project_store::tests::ito_path_resolves_correctly ... ok
test fs_project_store::tests::module_repository_returns_box_trait ... ok
test fs_project_store::tests::project_exists_returns_false_for_missing ... ok
test fs_project_store::tests::store_is_send_sync ... ok
test fs_project_store::tests::task_repository_returns_box_trait ... ok
test git::tests::fetch_coordination_branch_classifies_missing_remote_branch ... ok
test git::tests::fetch_coordination_branch_classifies_missing_remote_configuration ... ok
test git::tests::fetch_coordination_branch_succeeds_on_clean_fetch ... ok
test git::tests::push_coordination_branch_classifies_missing_remote_configuration ... ok
test git::tests::push_coordination_branch_classifies_non_fast_forward_rejection ... ok
test git::tests::push_coordination_branch_classifies_protection_rejection ... ok
test event_forwarder::tests::forward_persists_checkpoint_per_batch ... ok
test git::tests::setup_coordination_branch_creates_branch_when_remote_missing ... ok
test git::tests::setup_coordination_branch_fails_when_not_git_worktree ... ok
test git::tests::setup_coordination_branch_reports_missing_origin_when_create_push_fails ... ok
test git::tests::setup_coordination_branch_returns_ready_when_remote_branch_exists ... ok
test git_remote::tests::falls_back_to_remote_when_config_empty ... ok
test git_remote::tests::falls_back_to_remote_when_config_org_missing ... ok
test git_remote::tests::falls_back_to_remote_when_config_repo_missing ... ok
test git_remote::tests::ignores_empty_config_strings_and_falls_back_to_remote ... ok
test git_remote::tests::reexport_delegates_to_common_parser ... ok
test git_remote::tests::returns_config_values_when_both_set ... ok
test git_remote::tests::returns_none_when_remote_command_fails ... ok
test git_remote::tests::returns_none_when_remote_output_is_empty ... ok
test git_remote::tests::returns_none_when_remote_url_unrecognised ... ok
test grep::tests::collect_change_artifact_files_finds_all_md_files ... ok
test grep::tests::search_files_finds_matching_lines ... ok
test grep::tests::search_files_includes_correct_line_numbers ... ok
test grep::tests::search_files_rejects_invalid_regex ... ok
test grep::tests::search_files_respects_limit ... ok
test grep::tests::search_files_returns_empty_for_no_matches ... ok
test harness::claude_code::tests::binary_is_claude ... ok
test harness::claude_code::tests::build_args_with_allow_all ... ok
test harness::claude_code::tests::build_args_without_allow_all ... ok
test harness::claude_code::tests::build_args_without_model ... ok
test harness::claude_code::tests::harness_name_is_claude ... ok
test harness::codex::tests::binary_is_codex ... ok
test harness::codex::tests::build_args_with_allow_all ... ok
test harness::codex::tests::build_args_without_allow_all ... ok
test harness::codex::tests::harness_name_is_codex ... ok
test harness::github_copilot::tests::binary_is_copilot ... ok
test harness::github_copilot::tests::build_args_with_allow_all ... ok
test harness::github_copilot::tests::build_args_without_allow_all ... ok
test harness::github_copilot::tests::harness_name_is_github_copilot ... ok
test harness::opencode::tests::binary_is_opencode ... ok
test harness::opencode::tests::build_args_with_model ... ok
test harness::opencode::tests::build_args_without_model ... ok
test harness::opencode::tests::harness_name_is_opencode ... ok
test harness::stub::tests::from_env_or_default_with_explicit_path ... ok
test harness::stub::tests::name_returns_stub ... ok
test harness::stub::tests::run_sets_nonzero_duration ... ok
test harness::stub::tests::run_sets_timed_out_false ... ok
test harness::stub::tests::streams_output_returns_false ... ok
test harness::types::tests::as_str_all_variants ... ok
test harness::types::tests::display_matches_as_str ... ok
test harness::types::tests::from_str_invalid_returns_error ... ok
test harness::types::tests::from_str_valid_variants ... ok
test harness::types::tests::harness_help_matches_user_facing ... ok
test harness::types::tests::is_not_retriable_for_normal_codes ... ok
test harness::types::tests::is_retriable_for_all_retriable_codes ... ok
test harness::types::tests::parse_error_display ... ok
test installers::json_tests::classify_project_file_ownership_handles_user_owned_paths ... ok
test installers::json_tests::merge_json_objects_appends_and_deduplicates_array_entries ... ok
test installers::json_tests::merge_json_objects_keeps_existing_and_adds_template_keys ... ok
test installers::json_tests::write_claude_settings_merges_existing_file_on_update ... ok
test installers::json_tests::write_claude_settings_preserves_invalid_json_on_update ... ok
test installers::markers::tests::errors_when_only_one_marker_found ... ok
test installers::markers::tests::idempotent_when_applying_same_content_twice ... ok
test installers::markers::tests::inserts_block_when_missing ... ok
test installers::markers::tests::marker_must_be_on_own_line ... ok
test installers::markers::tests::replaces_existing_block_preserving_unmanaged_content ... ok
test installers::markers::tests::updates_file_on_disk ... ok
test installers::tests::gitignore_audit_session_added ... ok
test installers::tests::gitignore_both_session_entries ... ok
test installers::tests::gitignore_created_when_missing ... ok
test installers::tests::gitignore_does_not_duplicate_on_repeated_calls ... ok
test installers::tests::gitignore_exact_line_matching_trims_whitespace ... ok
test installers::tests::gitignore_full_audit_setup ... ok
test installers::tests::gitignore_ignores_local_configs ... ok
test installers::tests::gitignore_legacy_audit_events_unignore_noop_when_absent ... ok
test installers::tests::gitignore_legacy_audit_events_unignore_removed ... ok
test installers::tests::gitignore_noop_when_already_present ... ok
test installers::tests::gitignore_preserves_existing_content_and_adds_newline_if_missing ... ok
test installers::tests::release_tag_is_prefixed_with_v ... ok
test installers::tests::should_install_project_rel_filters_by_tool_id ... ok
test installers::tests::should_install_project_rel_filters_pi ... ok
test installers::tests::update_agent_model_field_updates_frontmatter_when_present ... ok
test installers::tests::update_model_in_yaml_replaces_or_inserts ... ok
test installers::tests::write_one_marker_managed_files_error_when_markers_missing_in_update_mode ... ok
test installers::tests::write_one_marker_managed_files_refuse_overwrite_without_markers ... ok
test installers::tests::write_one_marker_managed_files_update_existing_markers ... ok
test installers::tests::write_one_non_marker_files_skip_on_init_update_mode ... ok
test installers::tests::write_one_non_marker_ito_managed_files_overwrite_on_init_update_mode ... ok
test installers::tests::write_one_non_marker_user_owned_files_preserve_on_update_mode ... ok
test list::tests::counts_requirements_from_headings ... ok
test list::tests::iso_millis_matches_expected_shape ... ok
test event_forwarder::tests::forward_retries_transient_failure ... ok
test list::tests::list_changes_filters_by_progress_status ... ok
test list::tests::parse_modular_change_module_id_allows_overflow_change_numbers ... ok
test memory::rendering_tests::capture_command_empty_lists_render_as_empty_strings ... ok
test memory::rendering_tests::capture_command_expands_files_as_repeated_flags ... ok
test memory::rendering_tests::capture_command_expands_folders_with_explicit_flag_name ... ok
test list::tests::list_changes_sorts_by_name_and_recent ... ok
test memory::rendering_tests::capture_command_preserves_unknown_placeholders_literally ... ok
test memory::rendering_tests::capture_command_quotes_shell_metacharacters ... ok
test memory::rendering_tests::capture_command_substitutes_context_with_quoting ... ok
test memory::rendering_tests::capture_command_substitutes_missing_context_with_empty_quoted_string ... ok
test memory::rendering_tests::capture_not_configured_when_memory_section_absent ... ok
test memory::rendering_tests::capture_not_configured_when_only_search_is_set ... ok
test memory::rendering_tests::capture_skill_emits_structured_inputs_and_options ... ok
test memory::rendering_tests::mixed_shapes_render_independently ... ok
test memory::rendering_tests::query_command_substitutes_query ... ok
test memory::rendering_tests::search_command_renders_scope_as_empty_quoted_token_when_absent ... ok
test memory::rendering_tests::search_command_renders_scope_as_quoted_value ... ok
test memory::rendering_tests::search_command_substitutes_query_and_default_limit ... ok
test memory::rendering_tests::search_command_uses_supplied_limit_when_present ... ok
test memory::rendering_tests::search_not_configured_when_only_capture_is_set ... ok
test memory::rendering_tests::search_skill_includes_default_limit_in_structured_inputs ... ok
test memory::rendering_tests::shell_quote_escapes_embedded_single_quotes ... ok
test memory::rendering_tests::shell_quote_handles_empty_string ... ok
test memory::rendering_tests::shell_quote_preserves_unicode_bytes ... ok
test memory::rendering_tests::shell_quote_wraps_simple_strings_in_single_quotes ... ok
test module_repository::tests::regression_change_repository_populates_sub_module_id ... ok
test module_repository::tests::regression_parent_module_retains_direct_changes_while_sub_module_owns_sub_changes ... ok
test module_repository::tests::test_exists ... ok
test module_repository::tests::test_get ... ok
test module_repository::tests::test_get_not_found ... ok
test module_repository::tests::test_get_uses_full_name_input ... ok
test module_repository::tests::test_list ... ok
test orchestrate::gates::tests::remediation_includes_failed_gate_and_downstream_run_gates ... ok
test orchestrate::gates::tests::remediation_includes_failed_gate_even_when_policy_is_skip ... ok
test orchestrate::gates::tests::remediation_returns_empty_when_failed_gate_not_found ... ok
test orchestrate::gates::tests::remediation_skips_downstream_skip_gates ... ok
test module_repository::tests::test_list_with_change_counts ... ok
test git::tests::setup_coordination_branch_core_wraps_process_error ... ok
test process::tests::missing_executable_is_spawn_failure ... ok
test process::tests::rejects_current_dir_with_parent_component ... ok
test process::tests::rejects_empty_program ... ok
test process::tests::rejects_excessive_argument_bytes ... ok
test process::tests::rejects_nul_in_argument ... ok
test process::tests::rejects_nul_in_program ... ok
test process::tests::rejects_relative_program_with_components ... ok
test process::tests::run_returns_invalid_request_before_spawn ... ok
test ralph::duration::tests::test_format_duration ... ok
test ralph::duration::tests::test_parse_bare_number ... ok
test ralph::duration::tests::test_parse_case_insensitive ... ok
test ralph::duration::tests::test_parse_combined ... ok
test ralph::duration::tests::test_parse_errors ... ok
test ralph::duration::tests::test_parse_hours ... ok
test ralph::duration::tests::test_parse_minutes ... ok
test ralph::duration::tests::test_parse_seconds ... ok
test ralph::duration::tests::test_parse_with_whitespace ... ok
test ralph::prompt::tests::build_prompt_preamble_includes_completion_promise ... ok
test ralph::prompt::tests::build_prompt_preamble_includes_context ... ok
test ralph::prompt::tests::build_prompt_preamble_includes_iteration ... ok
test ralph::prompt::tests::build_prompt_preamble_includes_validation_failure ... ok
test ralph::prompt::tests::build_prompt_preamble_omits_context_when_none ... ok
test ralph::prompt::tests::build_prompt_preamble_omits_validation_when_none ... ok
test ralph::runner::runner_tests::commit_iteration_errors_on_git_add_failure ... ok
test ralph::runner::runner_tests::commit_iteration_errors_when_failed_commit_still_has_staged_changes ... ok
test ralph::runner::runner_tests::commit_iteration_noops_when_no_changes ... ok
test ralph::runner::runner_tests::commit_iteration_succeeds_when_git_add_and_commit_succeed ... ok
test ralph::runner::runner_tests::commit_iteration_treats_no_staged_changes_after_failed_commit_as_success ... ok
test ralph::runner::runner_tests::count_git_changes_counts_non_empty_lines ... ok
test ralph::runner::runner_tests::count_git_changes_returns_zero_on_git_failure ... ok
test ralph::runner::runner_tests::filter_eligible ... ok
test ralph::runner::runner_tests::filter_incomplete ... ok
test ralph::runner::runner_tests::filter_module_incomplete ... ok
test ralph::runner::runner_tests::filter_ready ... ok
test ralph::runner::runner_tests::filter_unprocessed_changes ... ok
test ralph::runner::runner_tests::finalize_queue_results_errors_with_failed_change_ids ... ok
test ralph::runner::runner_tests::infer_module_no_hyphen ... ok
test ralph::runner::runner_tests::infer_module_ok ... ok
test ralph::runner::runner_tests::now_ms_returns_positive_value ... ok
test ralph::runner::runner_tests::print_helpers ... ok
test ralph::runner::runner_tests::promise_empty_stdout ... ok
test ralph::runner::runner_tests::promise_empty_token ... ok
test ralph::runner::runner_tests::promise_incomplete ... ok
test ralph::runner::runner_tests::promise_nested ... ok
test ralph::runner::runner_tests::promise_no_tags ... ok
test ralph::runner::runner_tests::promise_second_match ... ok
test ralph::runner::runner_tests::promise_single_match ... ok
test ralph::runner::runner_tests::promise_whitespace_trimmed ... ok
test ralph::runner::runner_tests::render_failure_both ... ok
test ralph::runner::runner_tests::render_failure_empty ... ok
test ralph::runner::runner_tests::render_validation_fail_with_output ... ok
test ralph::runner::runner_tests::render_validation_pass ... ok
test ralph::runner::runner_tests::render_validation_whitespace_output ... ok
test ralph::runner::runner_tests::resolve_cwd_no_change_targeted_fallback ... ok
test ralph::runner::runner_tests::resolve_cwd_no_worktree_found_fallback ... ok
test ralph::runner::runner_tests::resolve_cwd_worktree_found ... ok
test ralph::runner::runner_tests::resolve_cwd_worktrees_not_enabled_fallback ... ok
test ralph::runner::runner_tests::worktree_task_validation_repo_selection ... ok
test ralph::state::tests::append_context_no_op_on_whitespace ... ok
test ralph::state::tests::is_safe_change_id_segment_accepts_valid ... ok
test ralph::state::tests::is_safe_change_id_segment_rejects_backslash ... ok
test ralph::state::tests::is_safe_change_id_segment_rejects_empty ... ok
test ralph::state::tests::is_safe_change_id_segment_rejects_too_long ... ok
test ralph::state::tests::load_context_returns_empty_when_missing ... ok
test ralph::state::tests::load_state_backfills_missing_new_fields ... ok
test ralph::state::tests::load_state_returns_none_when_missing ... ok
test ralph::state::tests::ralph_context_path_correct ... ok
test ralph::state::tests::ralph_state_dir_uses_safe_fallback_for_invalid_change_ids ... ok
test ralph::state::tests::ralph_state_json_path_correct ... ok
test ralph::state::tests::save_and_load_state_round_trip ... ok
test ralph::validation::tests::discover_commands_falls_back_to_agents_md ... ok
test ralph::validation::tests::discover_commands_falls_back_to_claude_md ... ok
test ralph::validation::tests::discover_commands_ito_config_json ... ok
test ralph::validation::tests::discover_commands_priority_ito_json_first ... ok
test ralph::validation::tests::discover_commands_returns_empty_when_nothing_configured ... ok
test ralph::validation::tests::extract_commands_from_json_multiple_paths ... ok
test ralph::validation::tests::extract_commands_from_markdown_finds_make_check ... ok
test ralph::validation::tests::extract_commands_from_markdown_finds_make_test ... ok
test ralph::validation::tests::extract_commands_from_markdown_ignores_other_lines ... ok
test ralph::validation::tests::normalize_commands_value_array ... ok
test ralph::validation::tests::normalize_commands_value_non_string ... ok
test ralph::validation::tests::normalize_commands_value_null ... ok
test ralph::validation::tests::normalize_commands_value_string ... ok
test ralph::validation::tests::project_validation_discovers_commands_from_repo_json ... ok
test process::tests::captures_stdout_and_stderr ... ok
test process::tests::captures_non_zero_exit ... ok
test ralph::validation::tests::run_extra_validation_failure ... ok
test ralph::validation::tests::task_completion_fails_when_remaining ... ok
test ralph::validation::tests::task_completion_passes_when_no_tasks ... ok
test ralph::validation::tests::truncate_for_context_long_truncated ... ok
test ralph::validation::tests::truncate_for_context_multibyte_utf8 ... ok
test ralph::validation::tests::truncate_for_context_short_unchanged ... ok
test sqlite_project_store::repositories::tests::archive_change_rolls_back_when_spec_promotion_fails ... ok
test sqlite_project_store::repositories::tests::ensure_project_creates_row ... ok
test sqlite_project_store::repositories::tests::ensure_project_is_idempotent ... ok
test sqlite_project_store::repositories::tests::get_change_returns_full_data ... ok
test sqlite_project_store::repositories::tests::get_missing_change_returns_not_found ... ok
test sqlite_project_store::repositories::tests::get_module_by_id ... ok
test sqlite_project_store::repositories::tests::on_disk_database_persists ... ok
test sqlite_project_store::repositories::tests::open_in_memory_creates_schema ... ok
test sqlite_project_store::repositories::tests::push_artifact_bundle_rolls_back_partial_writes_on_failure ... ok
test sqlite_project_store::repositories::tests::store_is_send_sync ... ok
test sqlite_project_store::repositories::tests::task_mutation_service_reports_poisoned_connection_without_panicking ... ok
test sqlite_project_store::repositories::tests::task_repository_loads_tasks ... ok
test sqlite_project_store::repositories::tests::task_repository_missing_change_returns_empty ... ok
test sqlite_project_store::repositories::tests::two_projects_are_isolated ... ok
test sqlite_project_store::repositories::tests::upsert_and_list_changes ... ok
test sqlite_project_store::repositories::tests::upsert_and_list_modules ... ok
test task_repository::tests::load_tasks_uses_schema_apply_tracks_when_set ... ok
test ralph::validation::tests::shell_timeout_is_failure ... ok
test task_repository::tests::test_get_task_counts_checkbox_format ... ok
test task_repository::tests::test_get_task_counts_enhanced_format ... ok
test task_repository::tests::test_missing_tasks_file_returns_zero ... ok
test task_repository::tests::test_has_tasks ... ok
test tasks::tests::read_tasks_markdown_rejects_traversal_like_change_id ... ok
test tasks::tests::read_tasks_markdown_returns_error_for_missing_file ... ok
test tasks::tests::read_tasks_markdown_returns_contents_for_existing_file ... ok
test tasks::tests::returns_empty_when_no_ready_tasks_exist ... ok
test templates::guidance::tests::strip_ito_internal_comment_blocks_removes_internal_template_guidance ... ok
test templates::schema_assets::tests::safe_relative_path_validation_blocks_traversal_and_absolute_paths ... ok
test templates::schema_assets::tests::safe_schema_name_rejects_dot_segments_and_periods ... ok
test templates::task_parsing::tests::parse_enhanced_tasks_extracts_ids_status_and_done ... ok
test templates::types::tests::schema_source_as_str_returns_expected_labels ... ok
test templates::types::tests::validation_yaml_parses_minimal_config ... ok
test templates::types::tests::validation_yaml_parses_proposal_entry_with_rules ... ok
test templates::types::tests::validation_yaml_parses_rules_extension_without_breaking_existing_shape ... ok
test token::tests::generated_token_has_expected_length ... ok
test token::tests::generated_token_is_url_safe ... ok
test token::tests::two_tokens_are_distinct ... ok
test token::tests::url_safe_base64_encode_known_vector ... ok
test token::tests::url_safe_base64_roundtrip_known_value ... ok
test validate::issue::tests::constructors_set_expected_fields ... ok
test validate::issue::tests::format_spec_is_idempotent_for_message_suffix ... ok
test validate::issue::tests::format_spec_preserves_non_object_metadata ... ok
test validate::issue::tests::location_helpers_set_line_and_column ... ok
test validate::issue::tests::metadata_helper_attaches_json_context ... ok
test validate::issue::tests::rule_id_helper_marks_issue_and_is_reflected_in_metadata ... ok
test validate::report::tests::extend_collects_multiple_issues ... ok
test validate::report::tests::finish_non_strict_only_fails_on_errors ... ok
test validate::report::tests::finish_strict_fails_on_warnings ... ok
test viewer::collector::tests::collect_proposal_artifacts_errors_for_unknown_change ... ok
test tasks::tests::returns_ready_tasks_for_ready_changes ... ok
test viewer::collector::tests::collect_proposal_artifacts_skips_missing_optional_files ... ok
test viewer::html::tests::html_viewer_availability_depends_on_pandoc ... ok
test viewer::html::tests::html_viewer_open_errors_when_pandoc_missing ... ok
test viewer::html::tests::html_viewer_reports_expected_description ... ok
test viewer::html::tests::html_viewer_reports_expected_name ... ok
test viewer::tests::concrete_viewers_report_expected_names ... ok
test viewer::tests::default_registry_includes_html_viewer ... ok
test viewer::collector::tests::collect_proposal_artifacts_orders_sections_and_preserves_content ... ok
test viewer::tests::viewer_backend_trait_exposes_required_methods ... ok
test viewer::tests::viewer_registry_filters_and_finds_available_viewers ... ok
test viewer::tests::viewer_registry_hides_tmux_when_disabled ... ok
test worktree_ensure::worktree_ensure_tests::ensure_creates_worktree_when_absent ... ok
test worktree_ensure::worktree_ensure_tests::ensure_existing_worktree_returns_path_without_creation ... ok
test worktree_ensure::worktree_ensure_tests::ensure_git_failure_returns_error ... ok
test worktree_ensure::worktree_ensure_tests::ensure_with_include_files_copies_them ... ok
test worktree_ensure::worktree_ensure_tests::ensure_worktrees_disabled_returns_cwd ... ok
test worktree_ensure::worktree_ensure_tests::validate_change_id_accepts_normal_ids ... ok
test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_empty ... ok
test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_leading_dash ... ok
test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_nul ... ok
test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_path_separators ... ok
test worktree_ensure::worktree_ensure_tests::validate_change_id_rejects_path_traversal ... ok
test worktree_init::worktree_init_tests::copy_include_files_copies_to_dest ... ok
test worktree_init::worktree_init_tests::copy_include_files_empty_config_and_no_file ... ok
test worktree_init::worktree_init_tests::copy_include_files_skips_existing_destination ... ok
test worktree_init::worktree_init_tests::copy_include_files_skips_missing_source ... ok
test worktree_init::worktree_init_tests::init_worktree_copies_files_and_runs_setup ... ok
test worktree_init::worktree_init_tests::init_worktree_no_setup_copies_files_only ... ok
test worktree_init::worktree_init_tests::init_worktree_preserves_existing_destination_file ... ok
test worktree_init::worktree_init_tests::init_worktree_setup_failure_returns_error ... ok
test worktree_init::worktree_init_tests::parse_worktree_include_file_comments_only ... ok
test worktree_init::worktree_init_tests::parse_worktree_include_file_empty_content ... ok
test worktree_init::worktree_init_tests::parse_worktree_include_file_strips_comments_and_blanks ... ok
test worktree_init::worktree_init_tests::parse_worktree_include_file_trims_whitespace ... ok
test worktree_init::worktree_init_tests::resolve_include_files_config_only ... ok
test worktree_init::worktree_init_tests::resolve_include_files_deduplicates ... ok
test worktree_init::worktree_init_tests::resolve_include_files_file_only ... ok
test worktree_init::worktree_init_tests::resolve_include_files_glob_expansion ... ok
test worktree_init::worktree_init_tests::resolve_include_files_ignores_directories ... ok
test worktree_init::worktree_init_tests::resolve_include_files_missing_include_file_ok ... ok
test worktree_init::worktree_init_tests::resolve_include_files_no_match_returns_empty ... ok
test worktree_init::worktree_init_tests::resolve_include_files_rejects_absolute_path_in_pattern ... ok
test worktree_init::worktree_init_tests::resolve_include_files_rejects_path_traversal ... ok
test worktree_init::worktree_init_tests::resolve_include_files_union_of_config_and_file ... ok
test worktree_init::worktree_init_tests::run_setup_empty_multiple_commands_is_noop ... ok
test worktree_init::worktree_init_tests::run_setup_empty_single_command_is_noop ... ok
test worktree_init::worktree_init_tests::run_setup_first_command_fails_stops_sequence ... ok
test worktree_init::worktree_init_tests::run_setup_multiple_commands_run_in_order ... ok
test worktree_init::worktree_init_tests::run_setup_no_config_is_noop ... ok
test worktree_init::worktree_init_tests::run_setup_single_command_invoked ... ok
test event_forwarder::tests::forward_skips_when_fully_forwarded ... ok
test audit::reconcile::tests::reconcile_fix_writes_compensating_events ... ok
test ralph::validation::tests::run_extra_validation_success ... ok
test event_forwarder::tests::forward_batches_correctly ... ok
test event_forwarder::tests::forward_respects_checkpoint ... ok
test viewer::tests::run_with_stdin_closes_pipe_after_write ... ok
test event_forwarder::tests::forward_sends_all_new_events ... ok
test event_forwarder::tests::forward_stops_on_permanent_failure ... ok
test coordination_worktree::coordination_worktree_tests::integration_create_and_remove_coordination_worktree ... ok
test audit::stream::tests::read_initial_events_returns_last_n ... ok
test audit::store::tests::legacy_worktree_log_is_removed_after_successful_migration ... ok
test event_forwarder::tests::forward_reads_events_from_routed_local_store ... ok
test coordination_worktree::coordination_worktree_tests::integration_auto_commit_coordination ... ok
test audit::store::tests::read_all_merges_and_replays_fallback_events_when_branch_recovers ... ok
test audit::stream::tests::poll_detects_new_events_from_routed_store ... ok

test result: ok. 583 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.26s

     Running tests/archive.rs (target/debug/deps/archive-c79e72efc7458f71)

running 3 tests
test check_task_completion_handles_checkbox_and_enhanced_formats ... ok
test generate_archive_name_prefixes_with_date ... ok
test discover_and_copy_specs_and_archive_change ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target/debug/deps/audit_mirror-48a966d634f28671)

running 6 tests
test audit_mirror_default_local_store_falls_back_without_creating_worktree_log ... ok
test audit_mirror_disabled_does_not_create_remote_branch ... ok
test audit_mirror_failures_do_not_break_local_append ... ok
test local_store_does_not_fall_back_when_internal_branch_exists_without_log_file ... ok
test audit_mirror_default_local_store_writes_to_internal_branch_without_worktree_log ... ok
test audit_mirror_enabled_pushes_to_configured_branch ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.67s

     Running tests/audit_storage.rs (target/debug/deps/audit_storage-e7a629a8e2f55903)

running 3 tests
test reads_events_from_injected_store_without_filesystem_path ... ok
test filters_events_from_injected_store ... ok
test memory_store_append_persists_events ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target/debug/deps/backend_archive-533e5837d2acd3aa)

running 6 tests
test backend_archive_fails_when_pull_unavailable ... ok
test backend_archive_with_skip_specs_does_not_copy_specs ... ok
test backend_archive_fails_when_backend_unavailable_for_mark_archived ... ok
test backend_archive_happy_path_produces_committable_state ... ok
test backend_archive_creates_backup_before_overwriting ... ok
test backend_archive_does_not_mutate_local_module_markdown ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/backend_auth.rs (target/debug/deps/backend_auth-b6789cd2da6a7c75)

running 13 tests
test resolve_token_seed_returns_none_when_all_empty ... ok
test resolve_token_seed_falls_back_to_config ... ok
test resolve_token_seed_cli_takes_precedence ... ok
test resolve_admin_tokens_deduplicates ... ok
test resolve_admin_tokens_merges_all_sources ... ok
test resolve_admin_tokens_skips_empty_config_entries ... ok
test init_generates_tokens_when_none_exist ... ok
test write_auth_creates_config_file ... ok
test write_auth_sets_restrictive_permissions ... ok
test write_auth_rejects_non_object_backend_server ... ok
test write_auth_rejects_non_object_root ... ok
test write_auth_preserves_existing_config ... ok
test init_skips_when_tokens_exist ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target/debug/deps/backend_auth_service-9df8cef3b672ffd8)

running 1 test
test init_rejects_non_object_backend_server ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target/debug/deps/backend_client_mode-c0a2e5b5098e243f)

running 15 tests
test allocate_returns_claimed_change ... ok
test allocate_no_work_returns_none ... ok
test config_disabled_returns_none ... ok
test backend_unavailable_detection ... ok
test config_enabled_with_token_resolves ... ok
test config_enabled_missing_token_fails_with_clear_message ... ok
test claim_conflict_returns_holder_error ... ok
test retriable_status_codes ... ok
test backend_task_repo_missing_returns_zero ... ok
test claim_success_returns_holder_info ... ok
test backend_change_repo_lists_and_filters ... ok
test pull_writes_artifacts_and_revision ... ok
test push_stale_revision_gives_actionable_error ... ok
test push_success_updates_local_revision ... ok
test backend_task_repo_parses_from_content ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target/debug/deps/backend_module_repository-6fa909b0e00d76c5)

running 5 tests
test backend_module_repository_list_sorts_by_id ... ok
test backend_module_repository_accepts_name_inputs ... ok
test backend_module_repository_normalizes_full_name_inputs ... ok
test backend_module_repository_list_sorts_deterministically ... ok
test read_module_markdown_falls_back_without_local_file ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target/debug/deps/backend_sub_module_support-9f7e7e2c3c195dc7)

running 9 tests
test backend_module_repository_list_includes_sub_module_summaries ... ok
test backend_module_repository_list_sub_modules_for_unknown_module_returns_error ... ok
test backend_module_repository_get_sub_module_not_found_returns_error ... ok
test backend_module_repository_list_sub_modules_returns_sorted_summaries ... ok
test backend_module_repository_get_sub_module_by_composite_id ... ok
test sqlite_store_persists_sub_module_id_on_change ... ok
test sqlite_store_legacy_change_has_no_sub_module_id ... ok
test sqlite_store_sub_module_change_roundtrips_through_artifact_bundle ... ok
test sqlite_store_list_changes_filters_by_sub_module_id ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/change_repository_lifecycle.rs (target/debug/deps/change_repository_lifecycle-e372d703538d7db4)

running 2 tests
test remote_runtime_ignores_local_change_dirs ... ok
test filesystem_change_repository_filters_archived ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/change_repository_orchestrate_metadata.rs (target/debug/deps/change_repository_orchestrate_metadata-067010cb844218ec)

running 1 test
test change_repository_exposes_orchestrate_metadata_from_ito_yaml ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/change_repository_parity.rs (target/debug/deps/change_repository_parity-f659b05d00236593)

running 18 tests
test backend_list_by_module_normalizes_module_id ... ok
test backend_resolve_lifecycle_filter_respected ... ok
test backend_resolve_numeric_short_form_matches_canonical_id ... ok
test backend_resolve_empty_input_returns_not_found ... ok
test backend_resolve_numeric_short_form_ambiguous ... ok
test backend_resolve_module_scoped_slug_not_found ... ok
test backend_resolve_module_scoped_slug_query ... ok
test sqlite_resolve_prefix_match ... ok
test sqlite_list_archived_filter_returns_empty ... ok
test sqlite_resolve_all_filter_finds_active_changes ... ok
test sqlite_resolve_archived_filter_returns_not_found ... ok
test sqlite_resolve_empty_input_returns_not_found ... ok
test sqlite_resolve_numeric_short_form_matches_canonical_id ... ok
test sqlite_list_all_filter_returns_active_changes ... ok
test sqlite_list_by_module_normalizes_module_id ... ok
test sqlite_get_with_all_filter_finds_change ... ok
test sqlite_resolve_numeric_short_form_ambiguous ... ok
test sqlite_get_with_archived_filter_returns_not_found ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/change_target_resolution_parity.rs (target/debug/deps/change_target_resolution_parity-d2e3698ece536198)

running 2 tests
test sqlite_resolver_honors_archived_lifecycle_like_filesystem ... ok
test change_target_resolution_matches_across_repository_modes ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/coordination_worktree.rs (target/debug/deps/coordination_worktree-6b768ba8ed344698)

running 15 tests
test symlink_tests::task_repo_missing_tasks_file_returns_zero_through_symlink ... ok
test symlink_tests::module_repo_exists_through_symlink ... ok
test symlink_tests::task_repo_has_tasks_through_symlink ... ok
test symlink_tests::module_repo_list_multiple_through_symlink ... ok
test symlink_tests::module_repo_get_through_symlink ... ok
test symlink_tests::change_repo_exists_through_symlink ... ok
test symlink_tests::task_written_through_symlink_lands_in_worktree ... ok
test symlink_tests::module_repo_list_through_symlink ... ok
test symlink_tests::change_repo_get_through_symlink ... ok
test symlink_tests::change_written_through_symlink_lands_in_worktree ... ok
test symlink_tests::task_repo_load_tasks_through_symlink ... ok
test symlink_tests::change_repo_list_through_symlink ... ok
test symlink_tests::all_repos_consistent_through_symlinks ... ok
test symlink_tests::module_repo_change_counts_through_symlink ... ok
test symlink_tests::change_repo_list_multiple_through_symlink ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/create.rs (target/debug/deps/create-2dc3126a4cdd511c)

running 15 tests
test create_change_rejects_uppercase_names ... ok
test create_change_in_sub_module_rejects_missing_parent_module ... ok
test create_module_returns_existing_module_when_name_matches ... ok
test create_change_in_sub_module_rejects_missing_sub_module_dir ... ok
test create_module_writes_description_to_purpose_section ... ok
test create_module_creates_directory_and_module_md ... ok
test create_change_in_sub_module_checklist_is_sorted_ascending ... ok
test create_change_rewrites_module_changes_in_ascending_change_id_order ... ok
test allocation_state_sub_module_keys_sort_after_parent ... ok
test create_change_allocates_next_number_from_existing_change_dirs ... ok
test create_change_creates_change_dir_and_updates_module_md ... ok
test create_change_in_sub_module_uses_composite_id_format ... ok
test create_change_in_sub_module_writes_checklist_to_sub_module_md ... ok
test create_change_in_sub_module_allocates_independent_sequence ... ok
test create_change_writes_allocation_modules_in_ascending_id_order ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/distribution.rs (target/debug/deps/distribution-482441b4c2a6fc82)

running 11 tests
test opencode_manifests_includes_plugin_and_skills ... ok
test codex_manifests_includes_bootstrap_and_skills ... ok
test claude_manifests_includes_hooks_and_skills ... ok
test github_manifests_includes_skills_and_commands ... ok
test install_manifests_keeps_non_worktree_placeholders_verbatim ... ok
test install_manifests_renders_worktree_skill_with_context ... ok
test install_manifests_make_tmux_skill_scripts_executable ... ok
test install_manifests_renders_worktree_skill_enabled ... ok
test install_manifests_creates_parent_directories ... ok
test install_manifests_writes_files_to_disk ... ok
test all_manifests_use_embedded_assets ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running tests/event_forwarding.rs (target/debug/deps/event_forwarding-2374a5666005a6bd)

running 6 tests
test forward_result_reports_diagnostics ... ok
test permanent_failure_stops_forwarding ... ok
test batch_boundaries_preserved ... ok
test full_forwarding_workflow ... ok
test transient_failure_retried_then_succeeds ... ok
test incremental_forwarding ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s

     Running tests/grep_scopes.rs (target/debug/deps/grep_scopes-9ec1f085c85e208b)

running 4 tests
test grep_scope_change_only_searches_one_change ... ok
test grep_respects_limit_across_scopes ... ok
test grep_scope_module_searches_all_changes_in_module ... ok
test grep_scope_all_searches_all_changes ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/harness_context.rs (target/debug/deps/harness_context-56d4b0db062c9238)

running 6 tests
test infer_context_from_cwd_infers_change_from_path ... ok
test infer_context_from_cwd_infers_module_from_ito_modules_path ... ok
test infer_context_from_cwd_returns_no_target_when_inconclusive ... ok
test infer_context_from_cwd_prefers_path_over_git_branch ... ok
test infer_context_from_cwd_infers_change_from_git_branch ... ok
test infer_context_from_cwd_infers_module_from_git_branch ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/harness_opencode.rs (target/debug/deps/harness_opencode-c5cffd50e1677ff2)

running 8 tests
test claude_harness_errors_when_claude_missing ... ok
test codex_harness_errors_when_codex_missing ... ok
test copilot_harness_errors_when_copilot_missing ... ok
test opencode_harness_errors_when_opencode_missing ... ok
test claude_harness_passes_model_and_allow_all_flags ... ok
test github_copilot_harness_passes_model_and_allow_all_flags ... ok
test opencode_harness_runs_opencode_binary_and_returns_outputs ... ok
test codex_harness_passes_model_and_allow_all_flags ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.04s

     Running tests/harness_streaming.rs (target/debug/deps/harness_streaming-5d3c1d442a1cfa4b)

running 2 tests
test no_timeout_when_process_exits_normally ... ok
test inactivity_timeout_kills_stalled_process ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.19s

     Running tests/harness_stub.rs (target/debug/deps/harness_stub-ad39800fc00d20fe)

running 6 tests
test stub_harness_default_returns_complete_promise ... ok
test stub_harness_errors_on_empty_steps ... ok
test stub_harness_from_env_prefers_env_over_default ... ok
test stub_step_defaults_match_json_schema ... ok
test stub_harness_from_json_path_runs_steps_and_repeats_last ... ok
test stub_harness_errors_on_missing_and_invalid_json ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/import.rs (target/debug/deps/import-d0e585bd2005ee38)

running 10 tests
test skips_already_imported_active_change_when_remote_bundle_matches ... ok
test rerun_archives_existing_remote_active_change_without_repush_when_bundle_matches ... ok
test dry_run_uses_preview_logic_without_mutating_backend ... ok
test dry_run_previews_without_importing ... ok
test pushes_when_remote_active_bundle_differs ... ok
test active_local_change_fails_when_backend_only_has_archived_copy ... ok
test archived_directory_with_empty_canonical_change_id_is_ignored ... ok
test imports_active_and_archived_changes_with_lifecycle_fidelity ... ok
test import_summary_records_failures_without_aborting_remaining_changes ... ok
test ignores_unrecognized_archive_directories_during_discovery ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/io.rs (target/debug/deps/io-d48f43efaf270075)

running 3 tests
test read_to_string_optional_returns_none_for_missing_file ... ok
test read_to_string_or_default_returns_empty_for_missing_file ... ok
test write_atomic_std_creates_parent_and_replaces_contents ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target/debug/deps/orchestrate_run_state-1adfc868ec2e7b7d)

running 7 tests
test orchestrate_max_parallel_aliases_resolve ... ok
test orchestrate_dependency_cycle_is_rejected ... ok
test orchestrate_resume_skips_terminal_gates ... ok
test orchestrate_run_id_generation_matches_expected_format ... ok
test orchestrate_run_state_creates_expected_layout ... ok
test orchestrate_change_state_is_written_and_readable ... ok
test orchestrate_event_log_appends_without_truncation ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target/debug/deps/planning_init-dd915d5d54cd7c9d)

running 3 tests
test read_planning_status_returns_error_for_missing_roadmap ... ok
test read_planning_status_returns_contents_for_existing_roadmap ... ok
test init_planning_structure_writes_files ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/ralph.rs (target/debug/deps/ralph-fa3bc8176b694532)

running 30 tests
test run_ralph_errors_when_max_iterations_is_zero ... ok
test run_ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains ... ok
test run_ralph_continue_ready_errors_when_targeting_change_or_module ... ok
test run_ralph_continue_ready_errors_when_repo_shifts_to_no_eligible_changes ... ok
test run_ralph_gives_up_after_max_retriable_retries ... ok
test run_ralph_opencode_counts_git_changes_when_in_repo ... ignored, Flaky in pre-commit: counts real uncommitted changes instead of test fixture
test run_ralph_add_and_clear_context_paths ... ok
test run_ralph_continues_after_harness_failure_by_default ... ok
test run_ralph_continue_ready_exits_when_repo_becomes_complete_before_preflight ... ok
test run_ralph_fails_after_error_threshold ... ok
test run_ralph_module_resolves_single_change ... ok
test run_ralph_non_retriable_exit_still_counts_against_threshold ... ok
test run_ralph_retries_retriable_exit_code_with_exit_on_error ... ok
test run_ralph_prompt_includes_task_context_and_guidance ... ok
test run_ralph_returns_error_on_harness_failure ... ok
test run_ralph_resets_retriable_counter_on_success ... ok
test run_ralph_status_path_works_with_no_state ... ok
test run_ralph_retries_retriable_exit_code_without_counting_against_threshold ... ok
test run_ralph_continue_ready_reorients_when_repo_state_shifts ... ok
test run_ralph_skip_validation_exits_immediately ... ok
test state_helpers_append_and_clear_context ... ok
test run_ralph_module_multiple_changes_errors_when_non_interactive ... ok
test run_ralph_continue_ready_processes_all_eligible_changes_across_repo ... ok
test run_ralph_continue_module_processes_all_ready_changes ... ok
test run_ralph_continue_ready_accumulates_failures_after_processing_remaining_changes ... ok
test run_ralph_completion_promise_trims_whitespace ... ok
test run_ralph_loop_writes_state_and_honors_min_iterations ... ok
test run_ralph_continues_when_completion_validation_fails ... ok
test run_ralph_worktree_disabled_uses_fallback_cwd ... ok
test run_ralph_worktree_enabled_state_written_to_effective_ito ... ok

test result: ok. 29 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.14s

     Running tests/repo_index.rs (target/debug/deps/repo_index-d4892230468ae1ee)

running 1 test
test repo_index_loads_and_excludes_archive_change_dir ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target/debug/deps/repo_integrity-e5e9905bed75a3e2)

running 3 tests
test invalid_change_dir_names_are_reported ... ok
test change_referring_to_missing_module_is_an_error ... ok
test duplicate_numeric_change_id_is_reported_for_all_conflicting_dirs ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target/debug/deps/repo_paths-38fc8e991f99cac4)

running 11 tests
test coordination_worktree_path_uses_explicit_worktree_path_when_set ... ok
test coordination_worktree_path_correct_structure_with_home_fallback ... ok
test coordination_worktree_path_falls_back_to_local_share_when_xdg_unset ... ok
test coordination_worktree_path_correct_structure_with_xdg ... ok
test coordination_worktree_path_ignores_xdg_when_explicit_path_set ... ok
test coordination_worktree_path_last_resort_uses_ito_path ... ok
test coordination_worktree_path_uses_xdg_data_home_when_set ... ok
test resolve_worktree_paths_respects_bare_control_siblings_strategy ... ok
Initialized empty Git repository in /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpM2vqqF/
test resolve_env_from_cwd_errors_in_bare_repo_without_ito_dir ... ok
test resolve_env_from_cwd_uses_nearest_ito_root_when_git_is_unavailable ... ok
Initialized empty Git repository in /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpIvndFO/.git/
test resolve_env_from_cwd_prefers_git_toplevel ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

     Running tests/repository_runtime.rs (target/debug/deps/repository_runtime-2b76e5971bf8859d)

running 6 tests
test remote_runtime_uses_remote_factory ... ok
test sqlite_mode_requires_db_path ... ok
test filesystem_runtime_builds_repository_set ... ok
test sqlite_runtime_builds_repository_set ... ok
test repository_modes_return_consistent_change_names ... ok
test resolve_target_parity_between_filesystem_and_sqlite ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/repository_runtime_config_validation.rs (target/debug/deps/repository_runtime_config_validation-272c7887ec5b6039)

running 1 test
test invalid_repository_mode_fails_fast ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/show.rs (target/debug/deps/show-eee2ebee907cd3ae)

running 15 tests
test parse_requirement_block_requirement_id_absent_gives_none ... ok
test parse_requirement_block_extracts_requirement_id ... ok
test parse_spec_show_json_extracts_overview_requirements_and_scenarios ... ok
test parse_requirement_block_multiple_requirements_with_ids ... ok
test parse_change_show_json_emits_deltas_with_operations ... ok
test parse_delta_spec_requirement_id_is_extracted ... ok
test read_module_markdown_returns_error_for_nonexistent_module ... ok
test bundle_main_specs_show_json_returns_not_found_when_no_specs_exist ... ok
test load_delta_spec_file_uses_parent_dir_name_as_spec ... ok
test bundle_main_specs_show_json_returns_io_error_when_spec_md_is_missing ... ok
test read_module_markdown_returns_empty_for_missing_module_md ... ok
test read_module_markdown_returns_contents_for_existing_module ... ok
test bundle_main_specs_show_json_is_id_sorted_and_contains_absolute_paths ... ok
test bundle_main_specs_markdown_includes_metadata_comments_and_excludes_deltas ... ok
test read_change_delta_spec_files_lists_specs_sorted ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/spec_repository_backends.rs (target/debug/deps/spec_repository_backends-4eb47ab8fd5a0468)

running 2 tests
test remote_runtime_exposes_spec_repository_without_local_specs ... ok
test filesystem_runtime_exposes_promoted_specs ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target/debug/deps/spec_show_repository-dcd281fb1a869620)

running 3 tests
test read_spec_markdown_from_repository_reads_remote_spec ... ok
test bundle_specs_show_json_from_repository_sorts_ids ... ok
test bundle_specs_markdown_from_repository_adds_metadata_comments ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target/debug/deps/sqlite_archive_mirror-34cac3b347cc1a37)

running 1 test
test sqlite_archive_promotes_specs_and_marks_change_archived ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/sqlite_task_mutations.rs (target/debug/deps/sqlite_task_mutations-c5baee2db61acd19)

running 3 tests
test sqlite_task_mutation_service_returns_not_found_for_missing_tasks ... ok
test sqlite_task_mutation_service_initializes_missing_tasks ... ok
test sqlite_task_mutation_service_updates_existing_markdown ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/stats.rs (target/debug/deps/stats-efc68f4a79b455dd)

running 2 tests
test compute_command_stats_counts_command_end_events ... ok
test collect_jsonl_files_finds_nested_jsonl_files ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target/debug/deps/task_repository_summary-e9c26eb34eb64124)

running 1 test
test repository_status_builds_summary_and_next_task ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target/debug/deps/tasks_api-db44ccbf9bb6153d)

running 15 tests
test list_ready_tasks_across_changes_handles_empty_repo ... ok
test init_tasks_creates_file_when_missing ... ok
test init_tasks_returns_true_when_file_already_exists ... ok
test tasks_api_rejects_non_tasks_tracking_validator_for_schema_tracking ... ok
test shelve_task_rejects_shelving_complete_task ... ok
test shelve_task_accepts_reason_parameter ... ok
test start_task_rejects_starting_shelved_task_directly ... ok
test get_next_task_returns_none_when_all_tasks_complete ... ok
test add_task_creates_wave_if_not_exists ... ok
test add_task_appends_new_task_with_next_id ... ok
test complete_task_accepts_note_parameter ... ok
test get_next_task_returns_first_ready_task_for_enhanced_format ... ok
test shelve_and_unshelve_task_round_trip_for_enhanced_format ... ok
test tasks_api_operates_on_schema_apply_tracks_file ... ok
test start_and_complete_task_enforced_by_dependencies_for_enhanced_format ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/tasks_checkbox_format.rs (target/debug/deps/tasks_checkbox_format-9e36ce1ac56f0dad)

running 3 tests
test checkbox_tasks_do_not_support_shelving ... ok
test checkbox_tasks_enforce_single_in_progress_and_next_task_logic_index_fallback ... ok
test checkbox_tasks_enforce_single_in_progress_and_next_task_logic_explicit_ids ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/tasks_orchestration.rs (target/debug/deps/tasks_orchestration-3b156920f5808258)

running 26 tests
test init_tasks_rejects_invalid_change_id ... ok
test get_task_status_returns_error_when_file_missing ... ok
test init_tasks_does_not_overwrite_existing_file ... ok
test init_tasks_creates_file_when_missing ... ok
test add_task_rejects_checkbox_format ... ok
test get_next_task_returns_current_in_progress_for_checkbox ... ok
test shelve_task_rejects_checkbox_format ... ok
test complete_task_handles_checkbox_format ... ok
test get_next_task_returns_none_when_all_complete ... ok
test start_task_rejects_already_complete ... ok
test shelve_task_rejects_complete_task ... ok
test start_task_errors_with_parse_errors ... ok
test unshelve_task_errors_with_parse_errors ... ok
test add_task_assigns_next_id_in_wave ... ok
test add_task_errors_with_parse_errors ... ok
test complete_task_errors_with_parse_errors ... ok
test start_task_rejects_shelved_task ... ok
test unshelve_task_rejects_not_shelved ... ok
test complete_task_handles_enhanced_format ... ok
test get_task_status_returns_diagnostics_for_malformed_file ... ok
test add_task_creates_wave_when_missing ... ok
test get_next_task_returns_first_ready_for_enhanced ... ok
test start_task_validates_task_is_ready ... ok
test shelve_task_errors_with_parse_errors ... ok
test add_task_defaults_to_wave_1 ... ok
test unshelve_task_transitions_to_pending ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-953d59564e34e391)

running 1 test
test compute_apply_instructions_reports_blocked_states_and_progress ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-31a1c1203be8b60f)

running 2 tests
test compute_change_status_rejects_invalid_change_name ... ok
test compute_change_status_marks_ready_and_blocked_based_on_generated_files ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target/debug/deps/templates_review_context-0b742d16dc218ae3)

running 1 test
test compute_review_context_collects_artifacts_validation_tasks_and_specs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-b807ffd193843121)

running 9 tests
test resolve_schema_rejects_path_traversal_name ... ok
test resolve_schema_rejects_absolute_and_backslash_names ... ok
test resolve_schema_uses_embedded_when_no_overrides_exist ... ok
test resolve_instructions_reads_embedded_templates ... ok
test resolve_instructions_exposes_enhanced_spec_driven_templates ... ok
test resolve_templates_rejects_traversal_template_path ... ok
test resolve_instructions_rejects_traversal_template_path ... ok
test resolve_schema_prefers_project_over_user_override ... ok
test export_embedded_schemas_writes_then_skips_without_force ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-66cd4e07749d4bd1)

running 9 tests
test list_schemas_detail_recommended_default_is_spec_driven ... ok
test list_schemas_detail_returns_all_embedded_schemas ... ok
test list_schemas_detail_json_round_trips ... ok
test built_in_minimalist_and_event_driven_spec_templates_use_delta_shape ... ok
test list_schemas_detail_entries_have_descriptions ... ok
test list_schemas_detail_is_sorted ... ok
test list_schemas_detail_all_sources_are_embedded ... ok
test list_schemas_detail_entries_have_artifacts ... ok
test list_schemas_detail_spec_driven_has_expected_artifacts ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target/debug/deps/templates_user_guidance-96447346c50cfeea)

running 7 tests
test load_user_guidance_for_artifact_rejects_path_traversal_ids ... ok
test load_user_guidance_strips_managed_header_block ... ok
test load_user_guidance_strips_ito_internal_comment_block ... ok
test load_user_guidance_for_artifact_reads_scoped_file ... ok
test load_user_guidance_for_artifact_strips_managed_header_block ... ok
test load_user_guidance_prefers_user_prompts_guidance_file ... ok
test load_composed_user_guidance_combines_scoped_and_shared ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target/debug/deps/traceability_e2e-c6124957efb5ebbe)

running 15 tests
test legacy_checkbox_change_validate_passes_without_traceability_checks ... ok
test legacy_checkbox_change_trace_output_is_unavailable ... ok
test traced_change_unresolved_ref_trace_output_shows_unresolved ... ok
test traced_change_all_covered_trace_output_is_ready ... ok
test traced_change_uncovered_req_trace_output_shows_uncovered ... ok
test duplicate_requirement_ids_trace_output_has_diagnostics ... ok
test shelved_task_leaves_requirement_uncovered ... ok
test partial_ids_trace_output_is_invalid ... ok
test partial_ids_validate_reports_error ... ok
test traced_change_unresolved_ref_is_error_in_validate ... ok
test traced_change_uncovered_req_is_warning_in_non_strict ... ok
test shelved_task_uncovered_req_is_warning_in_validate ... ok
test duplicate_requirement_ids_produce_error_in_validate ... ok
test traced_change_all_covered_validate_passes ... ok
test traced_change_uncovered_req_is_error_in_strict ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/validate.rs (target/debug/deps/validate-b649902940afd9dc)

running 23 tests
test validate_change_requires_at_least_one_delta ... ok
test validate_spec_markdown_reports_missing_purpose_and_requirements ... ok
test validate_spec_markdown_strict_treats_warnings_as_invalid ... ok
test validate_module_errors_when_sub_module_has_invalid_naming ... ok
test validate_module_warns_when_sub_module_purpose_too_short ... ok
test validate_module_errors_when_sub_module_missing_module_md ... ok
test validate_module_passes_when_sub_modules_have_valid_module_md ... ok
test validate_module_reports_missing_scope_and_short_purpose ... ok
test validate_tasks_file_issues_cite_tasks_tracking_validator_id ... ok
test validate_tasks_file_returns_error_for_missing_file ... ok
test validate_tasks_file_uses_apply_tracks_when_set ... ok
test validate_tasks_file_returns_diagnostics_for_malformed_content ... ok
test validate_change_with_unknown_schema_and_no_validation_yaml_does_not_require_deltas ... ok
test validate_tasks_file_returns_empty_for_valid_tasks ... ok
test validate_change_requires_shall_or_must_in_requirement_text ... ok
test validate_change_with_validation_yaml_and_no_delta_validator_does_not_require_deltas ... ok
test validate_change_validates_apply_tracks_file_when_configured ... ok
test validate_change_rejects_unsafe_apply_tracks_for_schema_validation_tracking ... ok
test validate_change_skips_optional_validator_when_artifact_is_missing ... ok
test validate_change_uses_apply_tracks_for_legacy_delta_schemas ... ok
test validate_change_rejects_unsafe_apply_tracks_for_legacy_delta_schemas ... ok
test validate_change_uses_validation_yaml_delta_specs_validator_when_configured ... ok
test empty_tracking_file_is_warning_in_non_strict_and_error_in_strict ... ok

test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-89df2223d1762e0e)

running 10 tests
test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
test contract_refs_rule_rejects_unknown_schemes ... ok
test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok
test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
test scenario_grammar_rule_warns_on_ui_mechanics_but_respects_ui_tags ... ok
test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/validate_rules_extension.rs (target/debug/deps/validate_rules_extension-e64c80ffaec810ba)

running 2 tests
test validation_yaml_proposal_entry_dispatches_rule_configuration ... ok
test validation_yaml_rules_extension_warns_for_unknown_rule_names ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (target/debug/deps/validate_tracking_rules-260a25c4b8016ed4)

running 4 tests
test task_quality_rule_enforces_done_when_and_verify_for_impl_tasks ... ok
test task_quality_rule_errors_on_unknown_requirement_ids ... ok
test task_quality_rule_errors_on_missing_status ... ok
test task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/worktree_ensure_e2e.rs (target/debug/deps/worktree_ensure_e2e-6f1ea015111cdb1f)

running 3 tests
test ensure_worktree_disabled_returns_cwd ... ok
test ensure_worktree_creates_and_initializes_with_include_files ... ok
test ensure_worktree_with_setup_script ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

     Running unittests src/lib.rs (target/debug/deps/ito_domain-5fc5bb94290af129)

running 119 tests
test audit::context::tests::resolve_harness_session_id_returns_none_without_env ... ok
test audit::event::tests::actor_serializes_to_lowercase ... ok
test audit::event::tests::entity_type_display ... ok
test audit::event::tests::builder_returns_none_without_required_fields ... ok
test audit::event::tests::builder_with_meta ... ok
test audit::event::tests::builder_produces_valid_event ... ok
test audit::event::tests::schema_version_is_one ... ok
test audit::event::tests::actor_round_trip ... ok
test audit::materialize::tests::empty_events_produce_empty_state ... ok
test audit::event::tests::entity_type_serializes_to_lowercase ... ok
test audit::event::tests::entity_type_round_trip ... ok
test audit::event::tests::optional_fields_omitted_when_none ... ok
test audit::event::tests::audit_event_serializes_to_single_line ... ok
test audit::event::tests::event_context_round_trip ... ok
test audit::event::tests::entity_type_as_str_matches_serde ... ok
test audit::event::tests::audit_event_round_trip_serialization ... ok
test audit::materialize::tests::archive_event_without_to_uses_sentinel ... ok
test audit::materialize::tests::reconciled_events_update_state ... ok
test audit::materialize::tests::single_create_event ... ok
test audit::materialize::tests::multiple_entities_tracked_independently ... ok
test audit::materialize::tests::last_event_wins ... ok
test audit::reconcile::tests::compensating_events_use_scope_from_drift_key ... ok
test audit::materialize::tests::status_change_updates_state ... ok
test audit::materialize::tests::global_entities_have_no_scope ... ok
test audit::reconcile::tests::detect_extra_in_log ... ok
test audit::reconcile::tests::detect_diverged_status ... ok
test audit::reconcile::tests::detect_missing_entity_in_log ... ok
test audit::reconcile::tests::display_drift_items ... ok
test audit::reconcile::tests::generate_compensating_events_for_diverged ... ok
test audit::reconcile::tests::generate_compensating_events_for_extra ... ok
test audit::reconcile::tests::generate_compensating_events_for_missing ... ok
test audit::reconcile::tests::multiple_drift_types_detected ... ok
test audit::reconcile::tests::no_drift_when_states_match ... ok
test audit::writer::tests::noop_writer_is_object_safe ... ok
test audit::writer::tests::noop_writer_is_send_sync ... ok
test audit::writer::tests::noop_writer_returns_ok ... ok
test audit::context::tests::resolve_session_id_generates_uuid ... ok
test audit::writer::tests::trait_is_object_safe_for_dyn_dispatch ... ok
test backend::tests::archive_result_roundtrip ... ok
test backend::tests::artifact_bundle_roundtrip ... ok
test backend::tests::backend_error_display_lease_conflict ... ok
test audit::context::tests::resolve_session_id_is_stable_across_calls ... ok
test backend::tests::backend_error_display_not_found ... ok
test backend::tests::backend_error_display_other ... ok
test backend::tests::backend_error_display_revision_conflict ... ok
test backend::tests::backend_error_display_unauthorized ... ok
test backend::tests::backend_error_display_unavailable ... ok
test backend::tests::event_batch_roundtrip ... ok
test backend::tests::event_ingest_result_roundtrip ... ok
test changes::tests::test_change_status_display ... ok
test changes::tests::test_change_sub_module_id_field ... ok
test changes::tests::test_change_summary_status ... ok
test changes::tests::test_change_work_status ... ok
test changes::tests::test_extract_module_id ... ok
test changes::tests::test_extract_sub_module_id ... ok
test changes::tests::test_normalize_id ... ok
test changes::tests::test_parse_change_id ... ok
test changes::tests::test_parse_change_id_sub_module_format ... ok
test changes::tests::test_parse_module_id ... ok
test errors::tests::ambiguous_target_joins_candidates_in_display_message ... ok
test errors::tests::io_constructor_preserves_context_and_source ... ok
test errors::tests::not_found_constructor_formats_display_message ... ok
test modules::tests::test_module_creation ... ok
test modules::tests::test_module_summary ... ok
test modules::tests::test_module_summary_with_sub_modules ... ok
test modules::tests::test_module_with_sub_modules ... ok
test modules::tests::test_sub_module_creation ... ok
test modules::tests::test_sub_module_summary_creation ... ok
test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_accepts_valid_formats ... ok
test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_handles_large_numbers ... ok
test tasks::checkbox::checkbox_tests::is_checkbox_task_id_token_rejects_invalid_formats ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_edge_case_single_digit_with_many_dots ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_extracts_id_and_rest ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_colon_suffix ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_dot_suffix ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_leading_whitespace ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_multiple_spaces ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_tab_separator ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_handles_unicode_in_task_name ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_preserves_trailing_whitespace_in_rest ... ok
test tasks::checkbox::checkbox_tests::split_checkbox_task_label_returns_none_for_invalid_inputs ... ok
test tasks::compute::tests::checkbox_mode_returns_pending_sorted_and_no_blocked ... ok
test tasks::compute::tests::enhanced_backcompat_blocks_later_waves_and_checkpoints_until_first_incomplete_wave_done ... ok
test tasks::compute::tests::enhanced_ready_and_blocked_lists_are_sorted_by_task_id ... ok
test tasks::compute::tests::enhanced_task_dependencies_produce_missing_crosswave_and_not_complete_blockers ... ok
test tasks::compute::tests::enhanced_wave_dependency_blocks_by_wave_and_unblocks_when_complete ... ok
test tasks::cycle::cycle_tests::find_cycle_path_returns_none_for_empty_graph ... ok
test discovery::tests::list_changes_skips_archive_dir ... ok
test discovery::tests::list_module_ids_extracts_numeric_prefixes ... ok
test discovery::tests::list_modules_only_returns_directories ... ok
test tasks::cycle::cycle_tests::find_cycle_path_handles_multiple_cycles_returns_one ... ok
test tasks::cycle::cycle_tests::find_cycle_path_handles_long_cycle ... ok
test tasks::cycle::cycle_tests::find_cycle_path_with_numeric_node_names ... ok
test tasks::cycle::cycle_tests::find_cycle_path_returns_none_for_acyclic_graph ... ok
test tasks::cycle::cycle_tests::find_cycle_path_handles_diamond_pattern_without_cycle ... ok
test tasks::cycle::cycle_tests::find_cycle_path_handles_special_characters_in_node_names ... ok
test tasks::cycle::cycle_tests::find_cycle_path_detects_self_loop ... ok
test tasks::cycle::cycle_tests::find_cycle_path_detects_simple_two_node_cycle ... ok
test tasks::cycle::cycle_tests::find_cycle_path_detects_cycle_in_complex_graph ... ok
test tasks::cycle::cycle_tests::find_cycle_path_detects_three_node_cycle ... ok
test tasks::relational::relational_tests::validate_relational_detects_duplicate_task_ids ... ok
test tasks::relational::relational_tests::validate_relational_detects_missing_task_dependencies ... ok
test tasks::relational::relational_tests::validate_relational_detects_self_referencing_task ... ok
test tasks::relational::relational_tests::validate_relational_accepts_valid_dependency_graph ... ok
test tasks::relational::relational_tests::validate_relational_detects_cross_wave_task_dependencies ... ok
test tasks::relational::relational_tests::validate_relational_marks_errors_as_error_level ... ok
test tasks::relational::relational_tests::validate_relational_ignores_empty_and_checkpoint_dependencies ... ok
test tasks::relational::relational_tests::validate_relational_detects_task_dependency_cycle ... ok
test tasks::relational::relational_tests::validate_relational_allows_shelved_task_depending_on_shelved_task ... ok
test tasks::relational::relational_tests::validate_relational_handles_tasks_without_wave ... ok
test tasks::relational::relational_tests::validate_relational_detects_dependency_on_shelved_task ... ok
test tasks::relational::relational_tests::validate_relational_detects_wave_dependency_cycle ... ok
test tasks::relational::relational_tests::validate_relational_detects_three_node_task_cycle ... ok
test tasks::relational::relational_tests::validate_relational_multiple_errors_for_same_task ... ok
test tasks::relational::relational_tests::validate_relational_reports_line_numbers ... ok
test tasks::relational::relational_tests::validate_relational_with_complex_valid_graph ... ok
test audit::context::tests::resolve_user_identity_returns_at_prefixed_string ... ok
test audit::context::tests::resolve_git_context_does_not_panic ... ok
test audit::context::tests::resolve_context_populates_session_id ... ok

test result: ok. 119 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

     Running tests/planning.rs (target/debug/deps/planning-bb4aa9b7f3dc37fa)

running 1 test
test roadmap_parsing_extracts_current_progress_and_phases ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/schema_roundtrip.rs (target/debug/deps/schema_roundtrip-55ee3dbf021be65a)

running 3 tests
test workflow_plan_json_roundtrip ... ok
test workflow_execution_json_roundtrip ... ok
test workflow_yaml_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/schema_validation.rs (target/debug/deps/schema_validation-4cf9060cf974ced3)

running 12 tests
test task_definition_validate_accepts_optional_fields ... ok
test task_execution_validate_rejects_empty_optional_strings ... ok
test execution_validate_rejects_invalid_fields_and_accepts_valid ... ok
test plan_validate_rejects_empty_prompt_content ... ok
test workflow_definition_validate_rejects_duplicate_wave_ids ... ok
test task_definition_validate_rejects_invalid_fields ... ok
test wave_definition_validate_rejects_invalid_shapes ... ok
test workflow_definition_validate_accepts_minimal_valid ... ok
test execution_validate_rejects_out_of_bounds_wave_index ... ok
test workflow_definition_validate_rejects_requires_and_context_files_empty_entries ... ok
test plan_validate_rejects_other_invalid_fields ... ok
test workflow_definition_validate_rejects_empty_fields ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tasks.rs (target/debug/deps/tasks-c2da7e8bbd45c045)

running 2 tests
test update_enhanced_task_status_inserts_or_replaces_status_line ... ok
test enhanced_template_parses_and_has_checkpoint_warning ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/tasks_parsing.rs (target/debug/deps/tasks_parsing-24bdbadbc41fa91a)

running 32 tests
test detect_tasks_format_enhanced_vs_checkbox ... ok
test parse_checkbox_tasks_uppercase_x_marks_complete ... ok
test parse_checkbox_tasks_handles_empty_lines_and_non_checkbox_content ... ok
test parse_checkbox_tasks_preserves_explicit_ids ... ok
test parse_checkbox_tasks_accepts_right_arrow_in_progress_marker ... ok
test parse_checkbox_tasks_handles_mixed_explicit_and_implicit_ids ... ok
test parse_checkbox_tasks_assigns_sequential_ids_when_not_explicit ... ok
test parse_checkbox_tasks_supports_dash_and_star ... ok
test parse_enhanced_tasks_handles_empty_dependencies_field ... ok
test tasks_path_checked_rejects_traversal_like_change_ids ... ok
test tasks_path_uses_safe_fallback_for_invalid_change_id ... ok
test update_checkbox_task_status_by_explicit_id ... ok
test update_checkbox_task_status_preserves_bullet_style ... ok
test update_checkbox_task_status_sets_marker_and_preserves_text ... ok
test update_enhanced_task_status_inserts_missing_fields ... ok
test parse_enhanced_tasks_handles_multiline_action ... ok
test update_enhanced_task_status_preserves_existing_fields ... ok
test parse_enhanced_tasks_handles_multiple_files ... ok
test update_enhanced_task_status_preserves_requirements_line ... ok
test parse_enhanced_tasks_accepts_all_prior_tasks_dependency_shorthand ... ok
test parse_enhanced_tasks_handles_task_without_optional_prefix ... ok
test enhanced_tasks_diagnostics_cover_common_errors ... ok
test parse_enhanced_tasks_requirements_not_carried_across_tasks ... ok
test parse_enhanced_tasks_parses_fields_and_action_block ... ok
test parse_enhanced_tasks_requirements_absent_gives_empty_vec ... ok
test parse_enhanced_tasks_extracts_requirements_field ... ok
test parse_enhanced_tasks_requirements_single_entry ... ok
test parse_enhanced_tasks_progress_counts_all_statuses ... ok
test parse_enhanced_tasks_handles_wave_with_comma_in_title ... ok
test enhanced_tasks_cycles_and_shelved_deps_are_reported ... ok
test enhanced_tasks_wave_gating_blocks_later_waves ... ok
test parse_enhanced_tasks_accepts_wave_heading_titles ... ok

test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/tasks_parsing_additional.rs (target/debug/deps/tasks_parsing_additional-70392865577173e8)

running 28 tests
test checkbox_format_handles_newlines_in_adjacent_lines ... ok
test checkbox_format_handles_empty_task_text ... ok
test checkbox_format_handles_special_characters_in_task_names ... ok
test checkbox_format_handles_very_long_task_names ... ok
test checkbox_format_ignores_incomplete_checkbox_patterns ... ok
test checkbox_format_progress_info_counts_correctly ... ok
test parse_empty_file_returns_empty_result ... ok
test parse_file_with_only_non_task_content ... ok
test parse_file_with_only_whitespace ... ok
test tasks_path_checked_accepts_valid_change_ids ... ok
test tasks_path_checked_rejects_empty_change_id ... ok
test tasks_path_checked_rejects_very_long_change_ids ... ok
test enhanced_format_handles_task_without_wave ... ok
test enhanced_format_handles_empty_action_block ... ok
test enhanced_format_handles_very_large_wave_numbers ... ok
test enhanced_format_handles_duplicate_wave_numbers ... ok
test progress_info_calculates_remaining_correctly ... ok
test enhanced_format_handles_very_long_file_paths ... ok
test wave_dependencies_detect_forward_references ... ok
test enhanced_format_handles_uppercase_x_in_complete_marker ... ok
test enhanced_format_handles_multiple_files_with_spaces ... ok
test enhanced_format_handles_status_marker_mismatch ... ok
test enhanced_format_validates_date_format_strictly ... ok
test enhanced_format_handles_checkpoints ... ok
test enhanced_format_validates_missing_required_fields ... ok
test enhanced_format_handles_multiline_action_with_code ... ok
test enhanced_format_handles_complex_dependency_chains ... ok
test wave_dependencies_handle_various_formats ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/tasks_quality_fields.rs (target/debug/deps/tasks_quality_fields-0eb0f0236fc88aa7)

running 2 tests
test quality_fields_allow_missing_optional_metadata ... ok
test quality_fields_round_trip_when_present ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/tasks_update.rs (target/debug/deps/tasks_update-ac55e02b3278861e)

running 19 tests
test update_checkbox_task_status_errors_for_invalid_or_missing_task_id ... ok
test update_checkbox_task_status_rejects_shelving ... ok
test update_checkbox_task_status_handles_mixed_explicit_and_implicit_ids ... ok
test update_checkbox_task_status_updates_by_1_based_index_and_preserves_formatting ... ok
test update_checkbox_task_status_handles_various_markers ... ok
test update_checkbox_task_status_with_id_suffix_colon ... ok
test update_checkbox_task_status_handles_unicode_in_task_text ... ok
test update_checkbox_task_status_with_id_suffix_dot ... ok
test update_checkbox_task_status_matches_explicit_ids_over_index ... ok
test update_checkbox_task_status_preserves_bullet_style ... ok
test update_enhanced_task_status_preserves_trailing_newline ... ok
test update_enhanced_task_status_handles_task_prefix_optional ... ok
test update_enhanced_task_status_preserves_other_fields ... ok
test update_enhanced_task_status_handles_in_progress ... ok
test update_enhanced_task_status_updates_status_and_date ... ok
test update_enhanced_task_status_handles_shelved ... ok
test update_enhanced_task_status_handles_complex_task_ids ... ok
test update_enhanced_task_status_only_updates_specified_task ... ok
test update_enhanced_task_status_inserts_missing_fields ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/traceability.rs (target/debug/deps/traceability-7d1e7a97dc8e05ad)

running 13 tests
test checkbox_format_gives_unavailable ... ok
test shelved_task_does_not_count_as_coverage ... ok
test duplicate_requirement_ids_flagged_in_diagnostics ... ok
test declared_requirements_are_sorted_and_deduplicated ... ok
test multiple_tasks_can_cover_same_requirement ... ok
test uncovered_requirement_appears_in_uncovered_list ... ok
test unresolved_task_reference_is_reported ... ok
test complete_task_counts_as_coverage ... ok
test in_progress_task_counts_as_coverage ... ok
test partial_ids_gives_invalid_with_missing_titles ... ok
test empty_requirements_list_gives_unavailable ... ok
test all_requirements_covered_by_tasks ... ok
test no_requirement_ids_gives_unavailable ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/lib.rs (target/debug/deps/ito_logging-56272adf71e46481)

running 2 tests
test tests::unsafe_session_ids_are_rejected ... ok
test tests::invalid_command_logger_writes_jsonl_entry ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/ito_templates-03541c9656a1fcd1)

running 83 tests
test agents::tests::render_template_replaces_variant ... ok
test agents::tests::default_configs_has_all_combinations ... ok
test agents::tests::render_template_removes_variant_line_if_not_set ... ok
test agents::tests::render_template_replaces_model ... ok
test instructions::tests::list_instruction_templates_is_sorted_and_non_empty ... ok
test instructions::tests::render_instruction_template_returns_not_found_for_missing_template ... ok
test instructions::tests::render_template_str_preserves_trailing_newline ... ok
test instructions::tests::render_template_str_is_strict_on_undefined ... ok
test instructions::tests::render_instruction_template_str_trims_block_whitespace ... ok
test instructions::tests::orchestrate_template_renders ... ok
test instructions::tests::render_template_str_renders_from_serialize_ctx ... ok
test instructions::tests::repo_sweep_template_renders ... ok
test instructions::tests::template_fetchers_work_for_known_and_unknown_paths ... ok
test instructions::tests::archive_template_renders_generic_guidance_without_change ... ok
test instructions::tests::worktree_init_template_includes_fresh_worktree_rules ... ok
test instructions::tests::archive_template_lists_available_changes_in_generic_mode ... ok
test instructions::tests::new_proposal_template_moves_to_worktree_after_create ... ok
test instructions::tests::schemas_template_includes_fix_and_platform_guidance ... ok
test instructions::tests::apply_template_bare_control_siblings_branches_from_default_branch ... ok
test instructions::tests::worktrees_template_bare_control_siblings_branches_from_default_branch ... ok
test instructions::tests::artifact_template_renders_when_instruction_is_empty ... ok
test instructions::tests::apply_template_renders_capture_reminder_when_configured ... ok
test instructions::tests::finish_template_includes_capture_reminder_when_memory_capture_configured ... ok
test project_templates::tests::default_context_is_disabled ... ok
test instructions::tests::archive_template_renders_targeted_instruction_with_change ... ok
test instructions::tests::apply_template_checkout_subdir_branches_from_default_branch ... ok
test instructions::tests::finish_template_prompts_for_archive ... ok
test instructions::tests::finish_template_includes_archive_check_when_prompt_suppressed ... ok
test instructions::tests::apply_template_omits_capture_reminder_when_search_only_configured ... ok
test instructions::tests::review_template_renders_conditional_sections ... ok
test project_templates::tests::render_project_template_passes_non_utf8_through ... ok
test project_templates::tests::render_project_template_passes_plain_text_through ... ok
test project_templates::tests::render_project_template_renders_simple_variable ... ok
test project_templates::tests::render_project_template_renders_conditional ... ok
test project_templates::tests::render_project_template_strict_on_undefined ... ok
test tests::default_home_files_returns_a_vec ... ok
test tests::default_project_includes_orchestrate_user_prompt ... ok
test tests::default_project_files_contains_expected_files ... ok
test tests::default_project_agents_mentions_fix_and_feature_entrypoints ... ok
test tests::every_shipped_agent_has_ito_prefix ... ok
test project_templates::tests::render_agents_md_with_worktrees_disabled ... ok
test tests::every_shipped_command_has_ito_prefix ... ok
test project_templates::tests::render_agents_md_with_checkout_siblings ... ok
test project_templates::tests::render_agents_md_with_checkout_subdir ... ok
test project_templates::tests::render_agents_md_with_bare_control_siblings ... ok
test tests::extract_managed_block_preserves_trailing_newline_from_content ... ok
test tests::extract_managed_block_rejects_inline_markers ... ok
test tests::extract_managed_block_returns_empty_for_empty_inner ... ok
test tests::extract_managed_block_returns_inner_content ... ok
test tests::fix_and_feature_commands_are_embedded ... ok
test tests::loop_command_template_uses_ito_loop_command_name ... ok
test tests::loop_skill_template_includes_yaml_frontmatter ... ok
test tests::memory_skill_is_embedded ... ok
test tests::normalize_ito_dir_empty_defaults_to_dot_ito ... ok
test tests::normalize_ito_dir_prefixes_dot ... ok
test tests::normalize_ito_dir_rejects_traversal_and_path_separators ... ok
test tests::orchestrate_skills_and_command_are_embedded ... ok
test tests::orchestrator_agent_templates_are_embedded_for_all_harnesses ... ok
test tests::proposal_intake_and_routing_skills_are_embedded ... ok
test tests::render_bytes_preserves_non_utf8 ... ok
test tests::render_bytes_returns_borrowed_when_no_rewrite_needed ... ok
test tests::render_bytes_rewrites_dot_ito_paths ... ok
test tests::render_rel_path_rewrites_ito_prefix ... ok
test tests::stamp_version_canonical_with_leading_whitespace_is_rewritten ... ok
test tests::stamp_version_handles_crlf_line_endings ... ok
test tests::get_preset_file_returns_contents ... ok
test tests::presets_files_contains_orchestrate_builtins ... ok
test tests::stamp_version_handles_prerelease_semver ... ok
test tests::get_schema_file_returns_contents ... ok
test tests::schema_files_contains_builtins ... ok
test tests::stamp_version_idempotent_on_canonical_match ... ok
test tests::stamp_version_idempotent_on_canonical_with_trailing_whitespace ... ok
test tests::every_shipped_skill_has_ito_prefix ... ok
test tests::stamp_version_inserts_when_missing ... ok
test tests::stamp_version_noop_without_marker ... ok
test tests::stamp_version_preserves_frontmatter ... ok
test tests::stamp_version_preserves_trailing_content ... ok
test tests::stamp_version_rewrites_older_version ... ok
test tests::stamp_version_rewrites_spaced_form_to_canonical ... ok
test tests::every_shipped_markdown_has_managed_markers ... ok
test tests::stamp_version_round_trip_on_real_skill ... ok
test tests::tmux_skill_and_scripts_are_embedded ... ok
test tests::every_shipped_markdown_has_exactly_one_marker_pair ... ok

test result: ok. 83 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-236160128d8f1c26)

running 5 tests
test agents_have_managed_markers ... ok
test commands_have_managed_markers ... ok
test schema_files_have_managed_markers ... ok
test skills_have_managed_markers ... ok
test default_project_files_have_managed_markers ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-f20a69c28970c25d)

running 3 tests
test commands_satisfy_ito_prefix_rule ... ok
test agents_satisfy_ito_prefix_rule ... ok
test skills_satisfy_ito_prefix_rule ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-a57612d95b38aa11)

running 8 tests
test stamp_no_op_when_no_managed_block ... ok
test stamp_inserts_when_no_existing_stamp ... ok
test stamp_preserves_rest_of_file ... ok
test stamp_idempotent_when_same_version ... ok
test stamp_rewrites_older_version_stamp ... ok
test stamp_rewrites_spaced_stamp_to_canonical ... ok
test stamp_works_with_frontmatter_before_marker ... ok
test stamp_round_trip_on_real_skill ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-6301bd5f8f1040f2)

running 1 test
test template_markdown_is_well_formed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-1e6ee8a887df41aa)

running 2 tests
test user_guidance_template_exists_and_has_markers ... ok
test user_prompt_stub_templates_exist ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-f7dd0662204f7435)

running 8 tests
test skill_checkout_subdir ... ok
test skill_checkout_siblings ... ok
test skill_disabled ... ok
test skill_bare_control_siblings ... ok
test agents_md_disabled ... ok
test agents_md_checkout_siblings ... ok
test agents_md_checkout_subdir ... ok
test agents_md_bare_control_siblings ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/ito_test_support-c1d17527cc675215)

running 4 tests
test tests::normalize_replaces_home_path ... ok
test tests::normalize_strips_ansi_and_crlf ... ok
test tests::copy_dir_all_copies_nested_files ... ok
test pty::tests::pty_can_echo_input_via_cat ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/mock_repos_smoke.rs (target/debug/deps/mock_repos_smoke-29a8ecfcf016817e)

running 3 tests
test mock_task_repo_returns_configured_tasks ... ok
test mock_module_repo_resolves_by_id_or_name ... ok
test mock_repos_basic_roundtrip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/ito_web-f2e3980d01f8f982)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/ito_web-de8b68d6af6d2ed6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_backend

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_common

running 1 test
test ito-rs/crates/ito-common/src/git_url.rs - git_url::parse_remote_url_org_repo (line 25) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.53s; merged doctests compilation took 0.26s
   Doc-tests ito_config

running 4 tests
test ito-rs/crates/ito-config/src/ito_dir/mod.rs - ito_dir::absolutize_and_normalize_lossy (line 112) ... ignored
test ito-rs/crates/ito-config/src/ito_dir/mod.rs - ito_dir::get_ito_path_fs (line 59) - compile ... ok
test ito-rs/crates/ito-config/src/ito_dir/mod.rs - ito_dir::lexical_normalize (line 129) ... ok
test ito-rs/crates/ito-config/src/ito_dir/mod.rs - ito_dir::absolutize_and_normalize (line 89) ... ok

test result: ok. 3 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.52s; merged doctests compilation took 0.26s
   Doc-tests ito_core

running 52 tests
test ito-rs/crates/ito-core/src/backend_http.rs - backend_http::task_list_to_parse_result (line 695) ... ignored
test ito-rs/crates/ito-core/src/backend_http.rs - backend_http::task_mutation_from_api (line 832) ... ignored
test ito-rs/crates/ito-core/src/git.rs - git::CoordinationGitError::new (line 53) ... ignored
test ito-rs/crates/ito-core/src/git.rs - git::ensure_coordination_branch_on_origin_with_runner (line 255) ... ignored
test ito-rs/crates/ito-core/src/git.rs - git::fetch_coordination_branch_with_runner (line 294) ... ignored
test ito-rs/crates/ito-core/src/git.rs - git::ensure_coordination_branch_on_origin (line 134) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::ensure_coordination_branch_on_origin_core (line 226) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::fetch_coordination_branch_core (line 155) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::reserve_change_on_coordination_branch (line 107) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::push_coordination_branch_with_runner (line 361) - compile ... ok
test ito-rs/crates/ito-core/src/harness/streaming_cli.rs - harness::streaming_cli::monitor_timeout (line 296) ... ignored
test ito-rs/crates/ito-core/src/harness/types.rs - harness::types::Harness::streams_output (line 202) ... ignored
test ito-rs/crates/ito-core/src/ralph/runner.rs - ralph::runner::run_ralph (line 188) - compile ... ok
test ito-rs/crates/ito-core/src/ralph/validation.rs - ralph::validation::run_shell_with_timeout (line 328) ... ignored
test ito-rs/crates/ito-core/src/show/mod.rs - show::extract_section_text (line 592) ... ignored
test ito-rs/crates/ito-core/src/show/mod.rs - show::parse_requirement_block (line 466) ... ignored
test ito-rs/crates/ito-core/src/tasks.rs - tasks::apply_add_task (line 686) ... ignored
test ito-rs/crates/ito-core/src/tasks.rs - tasks::checked_tasks_path (line 34) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::build_order (line 463) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::compute_change_status (line 369) ... ignored
test ito-rs/crates/ito-core/src/harness/streaming_cli.rs - harness::streaming_cli::CliHarness (line 22) ... ok
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::list_available_schemas (line 178) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::list_schemas_detail (line 220) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::read_change_schema (line 113) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::resolve_instructions (line 625) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::resolve_schema (line 284) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::resolve_templates (line 565) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::validate_change_name_input (line 80) ... ignored
test ito-rs/crates/ito-core/src/templates/review.rs - templates::review::compute_review_context (line 32) ... ignored
test ito-rs/crates/ito-core/src/templates/schema_assets.rs - templates::schema_assets::embedded_schema_names (line 110) ... ignored
test ito-rs/crates/ito-core/src/templates/schema_assets.rs - templates::schema_assets::load_embedded_schema_yaml (line 142) ... ignored
test ito-rs/crates/ito-core/src/templates/schema_assets.rs - templates::schema_assets::package_schemas_dir (line 18) ... ignored
test ito-rs/crates/ito-core/src/templates/schema_assets.rs - templates::schema_assets::project_schemas_dir (line 54) ... ignored
test ito-rs/crates/ito-core/src/templates/schema_assets.rs - templates::schema_assets::read_schema_template (line 197) ... ignored
test ito-rs/crates/ito-core/src/templates/schema_assets.rs - templates::schema_assets::user_schemas_dir (line 82) ... ignored
test ito-rs/crates/ito-core/src/trace.rs - trace::compute_trace_output (line 66) ... ignored
test ito-rs/crates/ito-core/src/validate/issue.rs - validate::issue (line 8) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::push_coordination_branch (line 84) ... ok
test ito-rs/crates/ito-core/src/harness/opencode.rs - harness::opencode::OpencodeHarness (line 10) ... ok
test ito-rs/crates/ito-core/src/harness/codex.rs - harness::codex::CodexHarness (line 10) ... ok
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::list_available_changes (line 157) ... ok
test ito-rs/crates/ito-core/src/harness/github_copilot.rs - harness::github_copilot::GitHubCopilotHarness (line 10) ... ok
test ito-rs/crates/ito-core/src/harness/claude_code.rs - harness::claude_code::ClaudeCodeHarness (line 10) ... ok
test ito-rs/crates/ito-core/src/harness/types.rs - harness::types::HarnessRunResult::is_retriable (line 160) ... ok
test ito-rs/crates/ito-core/src/ralph/duration.rs - ralph::duration::parse_duration (line 16) ... ok
test ito-rs/crates/ito-core/src/git.rs - git::push_coordination_branch_core (line 172) ... ok
test ito-rs/crates/ito-core/src/errors.rs - errors::CoreError::sqlite (line 125) ... ok
test ito-rs/crates/ito-core/src/errors.rs - errors::CoreError::serde (line 100) ... ok
test ito-rs/crates/ito-core/src/tasks.rs - tasks::complete_task (line 829) ... ok
test ito-rs/crates/ito-core/src/tasks.rs - tasks::start_task (line 792) ... ok
test ito-rs/crates/ito-core/src/git.rs - git::reserve_change_on_coordination_branch_core (line 195) ... ok
test ito-rs/crates/ito-core/src/process.rs - process::SystemProcessRunner::run_with_timeout (line 189) ... ok

test result: ok. 23 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.05s

all doctests ran in 0.81s; merged doctests compilation took 0.45s
   Doc-tests ito_domain

running 9 tests
test ito-rs/crates/ito-domain/src/tasks/parse.rs - tasks::parse::parse_dependencies_with_checkpoint (line 937) ... ignored
test ito-rs/crates/ito-domain/src/tasks/parse.rs - tasks::parse::parse_enhanced_tasks::flush_current (line 479) ... ignored
test ito-rs/crates/ito-domain/src/tasks/update.rs - tasks::update::update_checkbox_task_status (line 35) ... ok
test ito-rs/crates/ito-domain/src/traceability.rs - traceability::compute_traceability (line 82) ... ok
test ito-rs/crates/ito-domain/src/tasks/update.rs - tasks::update::update_enhanced_task_status (line 128) ... ok
test ito-rs/crates/ito-domain/src/tasks/parse.rs - tasks::parse::detect_tasks_format (line 292) ... ok
test ito-rs/crates/ito-domain/src/tasks/parse.rs - tasks::parse::enhanced_tasks_template (line 271) ... ok
test ito-rs/crates/ito-domain/src/tasks/parse.rs - tasks::parse::parse_checkbox_tasks (line 330) ... ok
test ito-rs/crates/ito-domain/src/tasks/parse.rs - tasks::parse::parse_enhanced_tasks (line 413) ... ok

test result: ok. 7 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.01s

all doctests ran in 0.55s; merged doctests compilation took 0.24s
   Doc-tests ito_logging

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_templates

running 7 tests
test ito-rs/crates/ito-templates/src/project_templates.rs - project_templates::WorktreeTemplateContext::default (line 47) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - commands_files (line 107) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_skill_file (line 74) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_command_file (line 173) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - schema_files (line 123) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_schema_file (line 156) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_adapter_file (line 91) ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

all doctests ran in 0.48s; merged doctests compilation took 0.21s
   Doc-tests ito_test_support

running 4 tests
test ito-rs/crates/ito-test-support/src/lib.rs - is_executable_candidate (line 151) ... ignored
test ito-rs/crates/ito-test-support/src/lib.rs - resolve_candidate_program (line 88) ... ignored
test ito-rs/crates/ito-test-support/src/lib.rs - run_rust_candidate (line 59) ... ignored
test ito-rs/crates/ito-test-support/src/lib.rs - run_with_env (line 184) ... ignored

test result: ok. 0 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.44s; merged doctests compilation took 0.17s
   Doc-tests ito_web

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

```bash
make check || true
```

```output
check for added large files..............................................Passed
check for merge conflicts................................................Passed
check toml...............................................................Passed
check yaml...............................................................Passed
check json...............................................................Passed
fix end of files.........................................................Passed
mixed line ending........................................................Passed
trim trailing whitespace.................................................Passed
pretty format json.......................................................Passed
yamllint.................................................................Passed
markdownlint-cli2........................................................Passed
cargo fmt (ito-rs).......................................................Passed
forbid local version metadata in Cargo.toml..............................Passed
cargo clippy (ito-rs)....................................................Passed
cargo doc warnings as errors (ito-rs)....................................Passed
cargo test with coverage (ito-rs)........................................Failed
- hook id: cargo-test-coverage
- exit code: 2

  Coverage enforcement: hard min=80%, target=90%
    Below 80%: build FAILS (hard floor)
    Below 90%: WARNING (target)
    Excluded crates: ito-web (no tests yet)

  info: cargo-llvm-cov currently setting cfg(coverage); you can opt-out it by passing --no-cfg-coverage
  error: failed to find llvm-tools-preview, please install llvm-tools-preview, or set LLVM_COV and LLVM_PROFDATA environment variables
  make[1]: *** [test-coverage] Error 1
cargo test affected (ito-rs).............................................Passed
check max lines (ito-rs).................................................Failed
- hook id: check-max-lines
- exit code: 2

  python3 "ito-rs/tools/check_max_lines.py" --max-lines "1000" --root "ito-rs"
  Error: 7 Rust files over hard limit (1200 lines):
    - ito-rs/crates/ito-core/src/ralph/runner.rs: 1426
    - ito-rs/crates/ito-cli/tests/ralph_smoke.rs: 1408
    - ito-rs/crates/ito-core/src/installers/mod.rs: 1380
    - ito-rs/crates/ito-config/src/config/types.rs: 1371
    - ito-rs/crates/ito-core/src/coordination_worktree.rs: 1283
    - ito-rs/crates/ito-core/tests/ralph.rs: 1279
    - ito-rs/crates/ito-templates/src/instructions_tests.rs: 1267
  Warning: 14 Rust files over soft limit (1000 lines):
    - ito-rs/crates/ito-cli/src/cli.rs: 1200 (consider splitting)
    - ito-rs/crates/ito-cli/src/app/instructions.rs: 1199 (consider splitting)
    - ito-rs/crates/ito-cli/tests/init_more.rs: 1143 (consider splitting)
    - ito-rs/crates/ito-core/src/create/mod.rs: 1131 (consider splitting)
    - ito-rs/crates/ito-core/src/validate/mod.rs: 1129 (consider splitting)
    - ito-rs/crates/ito-domain/src/tasks/parse.rs: 1097 (consider splitting)
    - ito-rs/crates/ito-templates/src/lib.rs: 1084 (consider splitting)
    - ito-rs/crates/ito-core/src/config.rs: 1077 (consider splitting)
    - ito-rs/crates/ito-core/src/tasks.rs: 1075 (consider splitting)
    - ito-rs/crates/ito-cli/src/commands/tasks.rs: 1061 (consider splitting)
    - ito-rs/crates/ito-core/src/coordination_worktree_tests.rs: 1039 (consider splitting)
    - ito-rs/crates/ito-core/src/backend_http.rs: 1025 (consider splitting)
    - ito-rs/crates/ito-core/src/templates/mod.rs: 1015 (consider splitting)
    - ito-rs/crates/ito-core/tests/validate.rs: 1010 (consider splitting)
  make[1]: *** [check-max-lines] Error 1
architecture guardrails..................................................Passed
cargo deny (license/advisory checks).....................................Failed
- hook id: cargo-deny
- exit code: 2

  advisories ok, bans FAILED, licenses ok, sources ok
  error[duplicate]: found 2 duplicate entries for crate 'wit-bindgen'
      ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/Cargo.lock:333:1
      │  
  333 │ ╭ wit-bindgen 0.51.0 registry+https://github.com/rust-lang/crates.io-index
  334 │ │ wit-bindgen 0.57.1 registry+https://github.com/rust-lang/crates.io-index
      │ ╰────────────────────────────────────────────────────────────────────────┘ lock entries
      │  
      ├ wit-bindgen v0.51.0
        └── wasip3 v0.4.0+wasi-0.3.0-rc-2026-01-06
            └── getrandom v0.4.2
                ├── tempfile v3.27.0
                │   ├── dialoguer v0.12.0
                │   │   └── ito-cli v0.1.28
                │   ├── insta v1.47.2
                │   │   └── (dev) ito-cli v0.1.28 (*)
                │   ├── (dev) ito-backend v0.1.28
                │   │   └── ito-cli v0.1.28 (*)
                │   ├── (dev) ito-cli v0.1.28 (*)
                │   ├── (dev) ito-config v0.1.28
                │   │   ├── (dev) ito-backend v0.1.28 (*)
                │   │   ├── ito-cli v0.1.28 (*)
                │   │   └── ito-core v0.1.28
                │   │       ├── ito-backend v0.1.28 (*)
                │   │       ├── ito-cli v0.1.28 (*)
                │   │       └── ito-web v0.1.28
                │   │           └── ito-cli v0.1.28 (*)
                │   ├── ito-core v0.1.28 (*)
                │   ├── (dev) ito-domain v0.1.28
                │   │   ├── ito-core v0.1.28 (*)
                │   │   └── ito-test-support v0.1.28
                │   │       └── (dev) ito-cli v0.1.28 (*)
                │   ├── (dev) ito-logging v0.1.28
                │   │   └── ito-cli v0.1.28 (*)
                │   ├── ito-test-support v0.1.28 (*)
                │   └── native-tls v0.2.18
                │       └── ureq v3.3.0
                │           ├── ito-cli v0.1.28 (*)
                │           └── ito-core v0.1.28 (*)
                └── uuid v1.23.1
                    ├── ito-core v0.1.28 (*)
                    ├── ito-domain v0.1.28 (*)
                    └── ito-logging v0.1.28 (*)
      ├ wit-bindgen v0.57.1
        └── wasip2 v1.0.3+wasi-0.2.9
            ├── getrandom v0.3.4
            │   └── rand_core v0.9.5
            │       ├── rand v0.9.4
            │       │   ├── ito-core v0.1.28
            │       │   │   ├── ito-backend v0.1.28
            │       │   │   │   └── ito-cli v0.1.28
            │       │   │   ├── ito-cli v0.1.28 (*)
            │       │   │   └── ito-web v0.1.28
            │       │   │       └── ito-cli v0.1.28 (*)
            │       │   ├── ito-logging v0.1.28
            │       │   │   └── ito-cli v0.1.28 (*)
            │       │   └── tungstenite v0.29.0
            │       │       └── tokio-tungstenite v0.29.0
            │       │           └── axum v0.8.9
            │       │               ├── axum-extra v0.10.3
            │       │               │   ├── ito-backend v0.1.28 (*)
            │       │               │   └── ito-web v0.1.28 (*)
            │       │               ├── ito-backend v0.1.28 (*)
            │       │               └── ito-web v0.1.28 (*)
            │       └── rand_chacha v0.9.0
            │           └── rand v0.9.4 (*)
            └── getrandom v0.4.2
                ├── tempfile v3.27.0
                │   ├── dialoguer v0.12.0
                │   │   └── ito-cli v0.1.28 (*)
                │   ├── insta v1.47.2
                │   │   └── (dev) ito-cli v0.1.28 (*)
                │   ├── (dev) ito-backend v0.1.28 (*)
                │   ├── (dev) ito-cli v0.1.28 (*)
                │   ├── (dev) ito-config v0.1.28
                │   │   ├── (dev) ito-backend v0.1.28 (*)
                │   │   ├── ito-cli v0.1.28 (*)
                │   │   └── ito-core v0.1.28 (*)
                │   ├── ito-core v0.1.28 (*)
                │   ├── (dev) ito-domain v0.1.28
                │   │   ├── ito-core v0.1.28 (*)
                │   │   └── ito-test-support v0.1.28
                │   │       └── (dev) ito-cli v0.1.28 (*)
                │   ├── (dev) ito-logging v0.1.28 (*)
                │   ├── ito-test-support v0.1.28 (*)
                │   └── native-tls v0.2.18
                │       └── ureq v3.3.0
                │           ├── ito-cli v0.1.28 (*)
                │           └── ito-core v0.1.28 (*)
                └── uuid v1.23.1
                    ├── ito-core v0.1.28 (*)
                    ├── ito-domain v0.1.28 (*)
                    └── ito-logging v0.1.28 (*)

  warning[unmatched-skip]: skipped crate 'windows-sys = =0.60' was not encountered
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:80:16
     │
  80 │     { crate = "windows-sys@0.60", reason = "tokio/clap transitive" },
     │                ━━━━━━━━━━━━━━━━             ───────────────────── reason
     │                │                             
     │                unmatched skip configuration

  warning[unnecessary-skip]: skip 'windows-targets = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:81:16
     │
  81 │     { crate = "windows-targets@0.52", reason = "follows windows-sys 0.59" },
     │                ━━━━━━━━━━━━━━━━━━━━             ──────────────────────── reason
     │                │                                 
     │                unnecessary skip configuration

  warning[unnecessary-skip]: skip 'windows_aarch64_gnullvm = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:82:16
     │
  82 │     { crate = "windows_aarch64_gnullvm@0.52", reason = "follows windows-targets 0.52" },
     │                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━             ──────────────────────────── reason
     │                │                                         
     │                unnecessary skip configuration

  warning[unnecessary-skip]: skip 'windows_aarch64_msvc = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:83:16
     │
  83 │     { crate = "windows_aarch64_msvc@0.52", reason = "follows windows-targets 0.52" },
     │                ━━━━━━━━━━━━━━━━━━━━━━━━━             ──────────────────────────── reason
     │                │                                      
     │                unnecessary skip configuration

  warning[unnecessary-skip]: skip 'windows_i686_gnu = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:84:16
     │
  84 │     { crate = "windows_i686_gnu@0.52", reason = "follows windows-targets 0.52" },
     │                ━━━━━━━━━━━━━━━━━━━━━             ──────────────────────────── reason
     │                │                                  
     │                unnecessary skip configuration

  warning[unnecessary-skip]: skip 'windows_i686_gnullvm = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:85:16
     │
  85 │     { crate = "windows_i686_gnullvm@0.52", reason = "follows windows-targets 0.52" },
     │                ━━━━━━━━━━━━━━━━━━━━━━━━━             ──────────────────────────── reason
     │                │                                      
     │                unnecessary skip configuration

  warning[unnecessary-skip]: skip 'windows_i686_msvc = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:86:16
     │
  86 │     { crate = "windows_i686_msvc@0.52", reason = "follows windows-targets 0.52" },
     │                ━━━━━━━━━━━━━━━━━━━━━━             ──────────────────────────── reason
     │                │                                   
     │                unnecessary skip configuration

  warning[unnecessary-skip]: skip 'windows_x86_64_gnu = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:87:16
     │
  87 │     { crate = "windows_x86_64_gnu@0.52", reason = "follows windows-targets 0.52" },
     │                ━━━━━━━━━━━━━━━━━━━━━━━             ──────────────────────────── reason
     │                │                                    
     │                unnecessary skip configuration

  warning[unnecessary-skip]: skip 'windows_x86_64_gnullvm = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:88:16
     │
  88 │     { crate = "windows_x86_64_gnullvm@0.52", reason = "follows windows-targets 0.52" },
     │                ━━━━━━━━━━━━━━━━━━━━━━━━━━━             ──────────────────────────── reason
     │                │                                        
     │                unnecessary skip configuration

  warning[unnecessary-skip]: skip 'windows_x86_64_msvc = =0.52' applied to a crate with only one version
     ┌─ /Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/deny.toml:89:16
     │
  89 │     { crate = "windows_x86_64_msvc@0.52", reason = "follows windows-targets 0.52" },
     │                ━━━━━━━━━━━━━━━━━━━━━━━━             ──────────────────────────── reason
     │                │                                     
     │                unnecessary skip configuration

  make[1]: *** [cargo-deny] Error 2
make: *** [check-prek] Error 1
```
