# Task 1.2: Wiki Preservation Semantics

*2026-04-26T20:24:25Z by Showboat 0.6.1*
<!-- showboat-id: 31b2cb2a-a884-4082-b3d4-056bd6f08df9 -->

Classified .ito/wiki/** as user-owned during template installation so update and init --upgrade preserve existing wiki content while still installing missing scaffold files.

```bash
cargo test -p ito-core --test wiki_install
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/wiki_install.rs (target/debug/deps/wiki_install-f1869aceda4b9d20)

running 2 tests
test update_preserves_existing_wiki_content_and_installs_missing_scaffold ... ok
test init_upgrade_preserves_existing_wiki_content ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```

```bash
cargo test -p ito-templates && cargo test -p ito-core
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-43511d335e81e446)

running 84 tests
test agents::tests::render_template_removes_variant_line_if_not_set ... ok
test agents::tests::default_configs_has_all_combinations ... ok
test agents::tests::render_template_replaces_variant ... ok
test agents::tests::render_template_replaces_model ... ok
test instructions::tests::list_instruction_templates_is_sorted_and_non_empty ... ok
test instructions::tests::new_proposal_template_moves_to_worktree_after_create ... ok
test instructions::tests::finish_template_includes_archive_check_when_prompt_suppressed ... ok
test instructions::tests::archive_template_lists_available_changes_in_generic_mode ... ok
test instructions::tests::archive_template_renders_targeted_instruction_with_change ... ok
test instructions::tests::artifact_template_renders_when_instruction_is_empty ... ok
test instructions::tests::render_instruction_template_returns_not_found_for_missing_template ... ok
test instructions::tests::finish_template_includes_capture_reminder_when_memory_capture_configured ... ok
test instructions::tests::finish_template_prompts_for_archive ... ok
test instructions::tests::archive_template_renders_generic_guidance_without_change ... ok
test instructions::tests::orchestrate_template_renders ... ok
test instructions::tests::apply_template_requires_change_worktree_when_apply_setup_disabled ... ok
test instructions::tests::apply_template_checkout_subdir_branches_from_default_branch ... ok
test instructions::tests::apply_template_omits_capture_reminder_when_search_only_configured ... ok
test instructions::tests::apply_template_renders_capture_reminder_when_configured ... ok
test instructions::tests::apply_template_bare_control_siblings_branches_from_default_branch ... ok
test instructions::tests::render_template_str_is_strict_on_undefined ... ok
test instructions::tests::render_instruction_template_str_trims_block_whitespace ... ok
test instructions::tests::render_template_str_preserves_trailing_newline ... ok
test instructions::tests::render_template_str_renders_from_serialize_ctx ... ok
test instructions::tests::repo_sweep_template_renders ... ok
test instructions::tests::template_fetchers_work_for_known_and_unknown_paths ... ok
test instructions::tests::schemas_template_includes_fix_and_platform_guidance ... ok
test project_templates::tests::default_context_is_disabled ... ok
test instructions::tests::worktree_init_template_includes_fresh_worktree_rules ... ok
test instructions::tests::worktrees_template_bare_control_siblings_branches_from_default_branch ... ok
test instructions::tests::review_template_renders_conditional_sections ... ok
test project_templates::tests::render_project_template_passes_non_utf8_through ... ok
test project_templates::tests::render_agents_md_with_checkout_siblings ... ok
test project_templates::tests::render_agents_md_with_bare_control_siblings ... ok
test project_templates::tests::render_project_template_passes_plain_text_through ... ok
test project_templates::tests::render_agents_md_with_worktrees_disabled ... ok
test project_templates::tests::render_agents_md_with_checkout_subdir ... ok
test project_templates::tests::render_project_template_renders_simple_variable ... ok
test project_templates::tests::render_project_template_renders_conditional ... ok
test tests::default_home_files_returns_a_vec ... ok
test project_templates::tests::render_project_template_strict_on_undefined ... ok
test tests::default_project_agents_mentions_fix_and_feature_entrypoints ... ok
test tests::default_project_files_contains_expected_files ... ok
test tests::default_project_includes_orchestrate_user_prompt ... ok
test tests::every_shipped_agent_has_ito_prefix ... ok
test tests::every_shipped_command_has_ito_prefix ... ok
test tests::every_shipped_skill_has_ito_prefix ... ok
test tests::extract_managed_block_preserves_trailing_newline_from_content ... ok
test tests::extract_managed_block_rejects_inline_markers ... ok
test tests::extract_managed_block_returns_empty_for_empty_inner ... ok
test tests::extract_managed_block_returns_inner_content ... ok
test tests::fix_and_feature_commands_are_embedded ... ok
test tests::get_preset_file_returns_contents ... ok
test tests::every_shipped_markdown_has_managed_markers ... ok
test tests::get_schema_file_returns_contents ... ok
test tests::loop_command_template_uses_ito_loop_command_name ... ok
test tests::loop_skill_template_includes_yaml_frontmatter ... ok
test tests::memory_skill_is_embedded ... ok
test tests::every_shipped_markdown_has_exactly_one_marker_pair ... ok
test tests::normalize_ito_dir_empty_defaults_to_dot_ito ... ok
test tests::normalize_ito_dir_prefixes_dot ... ok
test tests::normalize_ito_dir_rejects_traversal_and_path_separators ... ok
test tests::orchestrate_skills_and_command_are_embedded ... ok
test tests::orchestrator_agent_templates_are_embedded_for_all_harnesses ... ok
test tests::presets_files_contains_orchestrate_builtins ... ok
test tests::proposal_intake_and_routing_skills_are_embedded ... ok
test tests::render_bytes_preserves_non_utf8 ... ok
test tests::render_bytes_returns_borrowed_when_no_rewrite_needed ... ok
test tests::render_bytes_rewrites_dot_ito_paths ... ok
test tests::render_rel_path_rewrites_ito_prefix ... ok
test tests::schema_files_contains_builtins ... ok
test tests::stamp_version_canonical_with_leading_whitespace_is_rewritten ... ok
test tests::stamp_version_handles_crlf_line_endings ... ok
test tests::stamp_version_handles_prerelease_semver ... ok
test tests::stamp_version_idempotent_on_canonical_match ... ok
test tests::stamp_version_idempotent_on_canonical_with_trailing_whitespace ... ok
test tests::stamp_version_inserts_when_missing ... ok
test tests::stamp_version_noop_without_marker ... ok
test tests::stamp_version_preserves_frontmatter ... ok
test tests::stamp_version_preserves_trailing_content ... ok
test tests::stamp_version_rewrites_older_version ... ok
test tests::stamp_version_rewrites_spaced_form_to_canonical ... ok
test tests::stamp_version_round_trip_on_real_skill ... ok
test tests::tmux_skill_and_scripts_are_embedded ... ok

test result: ok. 84 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-4be66a48dfefacf5)

running 5 tests
test commands_have_managed_markers ... ok
test default_project_files_have_managed_markers ... ok
test schema_files_have_managed_markers ... ok
test agents_have_managed_markers ... ok
test skills_have_managed_markers ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-89f6f29b2c677eb1)

running 3 tests
test commands_satisfy_ito_prefix_rule ... ok
test agents_satisfy_ito_prefix_rule ... ok
test skills_satisfy_ito_prefix_rule ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-c542d94a0d9bbd52)

running 8 tests
test stamp_idempotent_when_same_version ... ok
test stamp_inserts_when_no_existing_stamp ... ok
test stamp_no_op_when_no_managed_block ... ok
test stamp_rewrites_spaced_stamp_to_canonical ... ok
test stamp_round_trip_on_real_skill ... ok
test stamp_works_with_frontmatter_before_marker ... ok
test stamp_preserves_rest_of_file ... ok
test stamp_rewrites_older_version_stamp ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-354bb8adddb77ade)

running 1 test
test template_markdown_is_well_formed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-d45bf1384b899f95)

running 2 tests
test user_guidance_template_exists_and_has_markers ... ok
test user_prompt_stub_templates_exist ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/wiki_scaffold.rs (target/debug/deps/wiki_scaffold-78e3336b2f711952)

running 1 test
test default_project_embeds_ito_wiki_scaffold ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-ea6b170a0185265d)

running 8 tests
test skill_disabled ... ok
test skill_checkout_siblings ... ok
test skill_checkout_subdir ... ok
test agents_md_disabled ... ok
test skill_bare_control_siblings ... ok
test agents_md_bare_control_siblings ... ok
test agents_md_checkout_siblings ... ok
test agents_md_checkout_subdir ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_templates

running 7 tests
test ito-rs/crates/ito-templates/src/lib.rs - commands_files (line 107) ... ok
test ito-rs/crates/ito-templates/src/project_templates.rs - project_templates::WorktreeTemplateContext::default (line 47) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_adapter_file (line 91) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_command_file (line 173) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - schema_files (line 123) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_skill_file (line 74) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_schema_file (line 156) ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.42s; merged doctests compilation took 0.17s
    Finished `test` profile [optimized + debuginfo] target(s) in 0.19s
     Running unittests src/lib.rs (target/debug/deps/ito_core-97a60e89e32a20a6)

running 589 tests
test audit::mirror::tests::merge_jsonl_ignores_blank_lines ... ok
test audit::mirror::tests::merge_jsonl_dedupes_and_appends_local_lines ... ok
test audit::mirror::tests::merge_jsonl_drops_events_older_than_one_month_from_newest_event ... ok
test audit::reader::reader_tests::reads_events_from_injected_store ... ok
test audit::mirror::tests::merge_jsonl_keeps_reconciled_events_after_different_event ... ok
test audit::mirror::tests::merge_jsonl_aggregates_adjacent_equivalent_reconciled_events ... ok
test audit::mirror::tests::merge_jsonl_count_cap_uses_timestamp_not_input_position ... ok
test audit::mirror::tests::merge_jsonl_caps_git_log_to_newest_1000_events ... ok
test audit::reconcile::tests::build_file_state_from_default_tasks_md ... ok
test audit::reconcile::tests::build_file_state_uses_apply_tracks_when_set ... ok
test audit::store::tests::internal_branch_location_keys_include_branch_identity ... ok
test audit::reader::reader_tests::read_from_missing_file_returns_empty ... ok
test audit::stream::tests::default_config_has_sensible_values ... ok
test audit::reconcile::tests::reconcile_empty_log ... ok
test audit::reader::reader_tests::skips_empty_lines ... ok
test audit::reader::reader_tests::read_parses_valid_events ... ok
test audit::reader::reader_tests::filter_by_scope ... ok
test audit::validate::tests::detect_duplicate_create ... ok
test audit::validate::tests::detect_status_transition_mismatch ... ok
test audit::validate::tests::detect_timestamp_ordering_violation ... ok
test audit::reader::reader_tests::skips_malformed_lines ... ok
test audit::validate::tests::different_scopes_are_independent ... ok
test audit::validate::tests::no_issues_for_valid_sequence ... ok
test audit::validate::tests::empty_events_no_issues ... ok
test audit::worktree::tests::aggregate_empty_worktrees ... ok
test audit::worktree::tests::find_worktree_bare_excluded ... ok
test audit::worktree::tests::find_worktree_matching_branch ... ok
test audit::worktree::tests::find_worktree_multiple_returns_first_match ... ok
test audit::worktree::tests::find_worktree_no_match ... ok
test audit::worktree::tests::parse_bare_worktree_excluded ... ok
test audit::reader::reader_tests::combined_filters ... ok
test audit::worktree::tests::parse_detached_head ... ok
test audit::reader::reader_tests::filter_by_entity_type ... ok
test audit::reader::reader_tests::filter_by_operation ... ok
test audit::worktree::tests::worktree_audit_log_path_resolves ... ok
test audit::worktree::tests::parse_multiple_worktrees ... ok
test audit::worktree::tests::parse_single_worktree ... ok
test audit::writer::tests::audit_log_path_resolves_correctly ... ok
test audit::writer::tests::best_effort_returns_ok_even_on_failure ... ok
test audit::writer::tests::appends_events_to_existing_file ... ok
test audit::writer::tests::creates_directory_and_file_on_first_write ... ok
test backend_change_repository::tests::get_delegates_to_reader ... ok
test backend_change_repository::tests::list_complete_filters_correctly ... ok
test backend_change_repository::tests::list_incomplete_filters_correctly ... ok
test backend_change_repository::tests::list_returns_all_changes ... ok
test backend_change_repository::tests::resolve_target_ambiguous ... ok
test backend_change_repository::tests::resolve_target_exact_match ... ok
test backend_change_repository::tests::resolve_target_not_found ... ok
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
test audit::writer::tests::each_line_is_valid_json ... ok
test backend_client::tests::project_namespace_env_takes_precedence_over_config ... ok
test backend_client::tests::project_namespace_from_config ... ok
test backend_client::tests::project_namespace_from_env_vars ... ok
test backend_client::tests::project_namespace_missing_org_fails ... ok
test backend_client::tests::project_namespace_missing_repo_fails ... ok
test backend_coordination::tests::allocate_no_work ... ok
test backend_coordination::tests::allocate_with_work ... ok
test audit::writer::tests::events_deserialize_back_correctly ... ok
test audit::writer::tests::preserves_existing_content ... ok
test backend_coordination::tests::claim_conflict ... ok
test backend_coordination::tests::claim_success ... ok
test backend_coordination::tests::is_backend_unavailable_detects_process_error ... ok
test backend_coordination::tests::release_success ... ok
test backend_health::tests::backend_health_status_default_is_all_false ... ok
test backend_health::tests::backend_health_status_serializes_error_state ... ok
test backend_health::tests::backend_health_status_serializes_to_json ... ok
test backend_http::backend_http_tests::archived_task_fallback_only_treats_not_found_as_missing ... ok
test backend_http::backend_http_tests::audit_ingest_posts_can_opt_into_retries ... ok
test backend_http::backend_http_tests::get_requests_are_retried_by_default ... ok
test backend_http::backend_http_tests::optional_task_text_body_serializes_payload_when_present ... ok
test backend_http::backend_http_tests::optional_task_text_body_uses_empty_object_when_absent ... ok
test backend_http::backend_http_tests::parse_timestamp_returns_error_for_invalid_rfc3339 ... ok
test backend_http::backend_http_tests::post_requests_are_not_retried_by_default ... ok
test backend_sync::tests::backend_error_mapping_produces_correct_error_types ... ok
test backend_sync::tests::path_traversal_in_capability_rejected ... ok
test backend_sync::tests::path_traversal_in_change_id_rejected ... ok
test backend_coordination::tests::archive_with_backend_skip_specs ... ok
test backend_sync::tests::pull_creates_backup ... ok
test backend_coordination::tests::archive_with_backend_happy_path ... ok
test backend_coordination::tests::archive_with_backend_backend_unavailable ... ok
test backend_sync::tests::push_missing_change_dir_fails ... ok
test backend_sync::tests::push_conflict_returns_actionable_error ... ok
test backend_task_repository::tests::checkbox_tasks_parsed_correctly ... ok
test backend_task_repository::tests::get_task_counts_from_backend ... ok
test backend_task_repository::tests::has_tasks_detects_content ... ok
test backend_task_repository::tests::has_tasks_empty_content ... ok
test backend_task_repository::tests::missing_tasks_returns_empty ... ok
test backend_sync::tests::pull_writes_artifacts_locally ... ok
test backend_sync::tests::read_local_bundle_sorts_specs ... ok
test change_repository::tests::resolve_target_includes_archive_when_requested ... ok
test backend_sync::tests::push_sends_local_bundle ... ok
test change_repository::tests::exists_and_get_work ... ok
test change_repository::tests::list_skips_archive_dir ... ok
test config::tests::is_valid_integration_mode_checks_correctly ... ok
test config::tests::is_valid_repository_mode_checks_correctly ... ok
test config::tests::is_valid_worktree_strategy_checks_correctly ... ok
test config::tests::resolve_worktree_template_defaults_reads_overrides ... ok
test config::tests::resolve_worktree_template_defaults_uses_defaults_when_missing ... ok
test config::tests::skill_id_resolves_returns_false_when_no_paths_exist ... ok
test config::tests::validate_config_value_accepts_archive_main_integration_mode ... ok
test config::tests::validate_config_value_accepts_positive_sync_interval ... ok
test config::tests::validate_config_value_accepts_unknown_keys ... ok
test config::tests::validate_config_value_accepts_valid_audit_mirror_branch_name ... ok
test config::tests::validate_config_value_accepts_valid_coordination_branch_name ... ok
test config::tests::validate_config_value_accepts_valid_integration_mode ... ok
test change_repository::tests::resolve_target_reports_ambiguity ... ok
test config::tests::validate_config_value_accepts_valid_memory_kind ... ok
test config::tests::validate_config_value_accepts_valid_repository_mode ... ok
test config::tests::validate_config_value_accepts_valid_strategy ... ok
test config::tests::validate_config_value_rejects_empty_memory_command_template ... ok
test config::tests::validate_config_value_rejects_empty_memory_skill_id ... ok
test config::tests::validate_config_value_rejects_invalid_archive_main_integration_mode ... ok
test config::tests::validate_config_value_rejects_invalid_audit_mirror_branch_name ... ok
test config::tests::validate_config_value_rejects_invalid_coordination_branch_name ... ok
test config::tests::validate_config_value_rejects_invalid_integration_mode ... ok
test change_repository::tests::resolve_target_module_scoped_query ... ok
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
test config::tests::validate_memory_config_rejects_missing_skill ... ok
test coordination::tests::create_dir_link_creates_symlink ... ok
test config::tests::validate_memory_config_passes_when_skill_resolves_in_flat_layout ... ok
test coordination::tests::format_message_broken_symlinks_contains_paths_and_hint ... ok
test coordination::tests::format_message_embedded_is_none ... ok
test coordination::tests::format_message_healthy_is_none ... ok
test coordination::tests::format_message_not_wired_contains_dir_and_hint ... ok
test coordination::tests::format_message_worktree_missing_contains_path_and_hint ... ok
test coordination::tests::format_message_wrong_target_contains_paths_and_hint ... ok
test config::tests::validate_memory_config_passes_when_skill_resolves_in_grouped_layout ... ok
test coordination::tests::create_dir_link_fails_when_dst_exists ... ok
test change_repository::tests::suggest_targets_prioritizes_slug_matches ... ok
test coordination::tests::gitignore_created_when_absent ... ok
test coordination::tests::gitignore_entries_added_when_missing ... ok
test coordination::tests::gitignore_no_duplicates_on_second_call ... ok
test coordination::tests::gitignore_preserves_existing_content ... ok
test coordination::tests::gitignore_skips_already_present_entries ... ok
test coordination::tests::health_embedded_returns_embedded ... ok
test coordination::tests::health_missing_link_is_not_wired ... ok
test coordination::tests::health_worktree_missing_when_dir_absent ... ok
test coordination::tests::health_not_wired_when_real_dirs_present ... ok
test coordination::tests::health_broken_symlinks_when_target_missing ... ok
test coordination::tests::health_healthy_when_all_symlinks_correct ... ok
test coordination::tests::remove_is_noop_when_dirs_absent ... ok
test coordination::tests::remove_is_noop_for_real_dirs ... ok
test coordination::tests::health_wrong_target_when_symlink_points_elsewhere ... ok
test coordination::tests::wire_creates_symlinks_for_all_dirs ... ok
test coordination::tests::wire_handles_empty_real_dir ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_is_noop_when_nothing_staged ... ok
test coordination::tests::remove_restores_real_dirs_with_content ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_commit_fails ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_git_add_fails ... ok
test coordination::tests::wire_is_idempotent ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_stages_and_commits_when_changes_exist ... ok
test coordination_worktree::coordination_worktree_tests::create_fetches_branch_from_origin_when_not_local ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback_in_sha256_repo ... ok
test coordination::tests::wire_migrates_real_dir_content ... ok
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_fetch_fails_unexpectedly ... ok
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_orphan_commit_fails ... ok
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_worktree_add_fails ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_when_not_on_remote ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_when_origin_not_configured ... ok
test coordination_worktree::coordination_worktree_tests::create_uses_existing_local_branch ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_storage_is_embedded ... ok
test audit::reconcile::tests::reconcile_missing_tasks_file ... ok
test audit::reconcile::tests::reconcile_detects_drift ... ok
test coordination_worktree::coordination_worktree_tests::remove_falls_back_to_force_when_clean_remove_fails ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_worktree_dir_does_not_exist ... ok
test audit::reconcile::tests::reconcile_no_drift ... ok
test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_force_remove_also_fails ... ok
test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_prune_fails ... ok
test coordination_worktree::coordination_worktree_tests::remove_runs_worktree_remove_then_prune ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_is_noop_when_storage_is_embedded ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_fetches_commits_and_pushes_when_healthy ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_rate_limits_when_recent_and_clean ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_force_bypasses_rate_limit ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_returns_error_when_links_point_to_wrong_target ... ok
test create::create_sub_module_tests::create_sub_module_accepts_full_module_folder_name ... ok
test create::create_sub_module_tests::create_sub_module_creates_directory_and_module_md ... ok
test create::create_sub_module_tests::create_sub_module_errors_on_duplicate_name ... ok
test create::create_sub_module_tests::create_sub_module_errors_on_unknown_parent_module ... ok
test create::create_sub_module_tests::create_sub_module_rejects_invalid_name ... ok
test distribution::tests::pi_adapter_asset_exists_in_embedded_templates ... ok
test distribution::tests::pi_agent_templates_discoverable ... ok
test distribution::tests::pi_manifests_commands_match_opencode_commands ... ok
test distribution::tests::pi_manifests_includes_adapter_skills_and_commands ... ok
test distribution::tests::pi_manifests_skills_match_opencode_skills ... ok
test create::create_sub_module_tests::create_sub_module_allocates_sequential_numbers ... ok
test errors::tests::core_error_helpers_construct_expected_variants ... ok
test event_forwarder::tests::checkpoint_missing_returns_zero ... ok
test create::create_sub_module_tests::create_sub_module_with_description_writes_purpose ... ok
test distribution::tests::ensure_manifest_script_is_executable_only_adds_execute_bits ... ok
test event_forwarder::tests::checkpoint_roundtrip ... ok
test event_forwarder::tests::forward_no_events_returns_zero ... ok
test audit::worktree::tests::aggregate_worktree_with_events ... ok
test audit::stream::tests::poll_returns_empty_when_no_new_events ... ok
test event_forwarder::tests::forward_result_equality ... ok
test audit::stream::tests::poll_detects_new_events ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_calls_auto_commit_when_worktree_mode_and_dir_exists ... ok
test event_forwarder::tests::forward_persists_checkpoint_per_batch ... ok
test event_forwarder::tests::forward_reports_duplicates ... ok
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
test git::tests::setup_coordination_branch_core_wraps_process_error ... ok
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
test event_forwarder::tests::forward_retries_transient_failure ... ok
test grep::tests::collect_change_artifact_files_finds_all_md_files ... ok
test grep::tests::search_files_finds_matching_lines ... ok
test grep::tests::search_files_rejects_invalid_regex ... ok
test grep::tests::search_files_includes_correct_line_numbers ... ok
test grep::tests::search_files_respects_limit ... ok
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
test grep::tests::search_files_returns_empty_for_no_matches ... ok
test harness::opencode::tests::build_args_without_model ... ok
test harness::opencode::tests::harness_name_is_opencode ... ok
test harness::stub::tests::name_returns_stub ... ok
test harness::stub::tests::run_sets_nonzero_duration ... ok
test harness::stub::tests::run_sets_timed_out_false ... ok
test harness::stub::tests::streams_output_returns_false ... ok
test harness::types::tests::as_str_all_variants ... ok
test harness::types::tests::display_matches_as_str ... ok
test harness::types::tests::from_str_invalid_returns_error ... ok
test harness::stub::tests::from_env_or_default_with_explicit_path ... ok
test harness::types::tests::from_str_valid_variants ... ok
test harness::types::tests::harness_help_matches_user_facing ... ok
test harness::types::tests::is_not_retriable_for_normal_codes ... ok
test harness::types::tests::is_retriable_for_all_retriable_codes ... ok
test harness::types::tests::parse_error_display ... ok
test installers::json_tests::classify_project_file_ownership_handles_user_owned_paths ... ok
test installers::json_tests::merge_json_objects_appends_and_deduplicates_array_entries ... ok
test installers::json_tests::merge_json_objects_keeps_existing_and_adds_template_keys ... ok
test installers::json_tests::write_claude_settings_preserves_invalid_json_on_update ... ok
test installers::markers::tests::errors_when_only_one_marker_found ... ok
test installers::json_tests::write_claude_settings_merges_existing_file_on_update ... ok
test installers::markers::tests::idempotent_when_applying_same_content_twice ... ok
test installers::markers::tests::inserts_block_when_missing ... ok
test installers::markers::tests::marker_must_be_on_own_line ... ok
test installers::markers::tests::replaces_existing_block_preserving_unmanaged_content ... ok
test installers::markers::tests::updates_file_on_disk ... ok
test installers::tests::gitignore_audit_session_added ... ok
test installers::tests::gitignore_both_session_entries ... ok
test installers::tests::gitignore_created_when_missing ... ok
test installers::tests::gitignore_exact_line_matching_trims_whitespace ... ok
test installers::tests::gitignore_does_not_duplicate_on_repeated_calls ... ok
test installers::tests::gitignore_full_audit_setup ... ok
test installers::tests::gitignore_ignores_local_configs ... ok
test installers::tests::gitignore_legacy_audit_events_unignore_noop_when_absent ... ok
test installers::tests::gitignore_legacy_audit_events_unignore_removed ... ok
test installers::tests::gitignore_noop_when_already_present ... ok
test installers::tests::release_tag_is_prefixed_with_v ... ok
test installers::tests::should_install_project_rel_filters_by_tool_id ... ok
test installers::tests::should_install_project_rel_filters_pi ... ok
test installers::tests::gitignore_preserves_existing_content_and_adds_newline_if_missing ... ok
test installers::tests::update_model_in_yaml_replaces_or_inserts ... ok
test installers::tests::update_agent_model_field_updates_frontmatter_when_present ... ok
test installers::tests::write_one_marker_managed_files_error_when_markers_missing_in_update_mode ... ok
test installers::tests::write_one_marker_managed_files_refuse_overwrite_without_markers ... ok
test installers::tests::write_one_marker_managed_files_update_existing_markers ... ok
test installers::tests::write_one_non_marker_files_skip_on_init_update_mode ... ok
test installers::tests::write_one_non_marker_ito_managed_files_overwrite_on_init_update_mode ... ok
test list::tests::counts_requirements_from_headings ... ok
test list::tests::iso_millis_matches_expected_shape ... ok
test installers::tests::write_one_non_marker_user_owned_files_preserve_on_update_mode ... ok
test list::tests::list_changes_sorts_by_name_and_recent ... ok
test list::tests::parse_modular_change_module_id_allows_overflow_change_numbers ... ok
test memory::rendering_tests::capture_command_empty_lists_render_as_empty_strings ... ok
test memory::rendering_tests::capture_command_expands_files_as_repeated_flags ... ok
test memory::rendering_tests::capture_command_expands_folders_with_explicit_flag_name ... ok
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
test list::tests::list_changes_filters_by_progress_status ... ok
test module_repository::tests::test_exists ... ok
test module_repository::tests::test_get ... ok
test module_repository::tests::regression_parent_module_retains_direct_changes_while_sub_module_owns_sub_changes ... ok
test module_repository::tests::test_get_not_found ... ok
test module_repository::tests::test_get_uses_full_name_input ... ok
test module_repository::tests::test_list ... ok
test orchestrate::gates::tests::remediation_includes_failed_gate_and_downstream_run_gates ... ok
test orchestrate::gates::tests::remediation_includes_failed_gate_even_when_policy_is_skip ... ok
test orchestrate::gates::tests::remediation_returns_empty_when_failed_gate_not_found ... ok
test orchestrate::gates::tests::remediation_skips_downstream_skip_gates ... ok
test module_repository::tests::test_list_with_change_counts ... ok
test process::tests::captures_non_zero_exit ... ok
test process::tests::captures_stdout_and_stderr ... ok
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
test event_forwarder::tests::forward_batches_correctly ... ok
test ralph::runner::runner_tests::filter_unprocessed_changes ... ok
test ralph::runner::runner_tests::finalize_queue_results_errors_with_failed_change_ids ... ok
test ralph::runner::runner_tests::infer_module_no_hyphen ... ok
test ralph::runner::runner_tests::infer_module_ok ... ok
test ralph::runner::runner_tests::now_ms_returns_positive_value ... ok
test ralph::runner::runner_tests::print_helpers ... ok
test ralph::runner::runner_tests::promise_empty_stdout ... ok
test event_forwarder::tests::forward_respects_checkpoint ... ok
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
test ralph::state::tests::is_safe_change_id_segment_rejects_backslash ... ok
test ralph::state::tests::is_safe_change_id_segment_accepts_valid ... ok
test ralph::state::tests::is_safe_change_id_segment_rejects_empty ... ok
test ralph::state::tests::is_safe_change_id_segment_rejects_too_long ... ok
test ralph::state::tests::load_context_returns_empty_when_missing ... ok
test ralph::state::tests::load_state_returns_none_when_missing ... ok
test ralph::state::tests::ralph_context_path_correct ... ok
test ralph::state::tests::ralph_state_dir_uses_safe_fallback_for_invalid_change_ids ... ok
test ralph::state::tests::ralph_state_json_path_correct ... ok
test ralph::state::tests::append_context_no_op_on_whitespace ... ok
test ralph::validation::tests::discover_commands_falls_back_to_agents_md ... ok
test ralph::state::tests::load_state_backfills_missing_new_fields ... ok
test ralph::validation::tests::discover_commands_falls_back_to_claude_md ... ok
test ralph::state::tests::save_and_load_state_round_trip ... ok
test ralph::validation::tests::discover_commands_ito_config_json ... ok
test ralph::validation::tests::extract_commands_from_json_multiple_paths ... ok
test ralph::validation::tests::extract_commands_from_markdown_finds_make_check ... ok
test ralph::validation::tests::extract_commands_from_markdown_finds_make_test ... ok
test ralph::validation::tests::extract_commands_from_markdown_ignores_other_lines ... ok
test ralph::validation::tests::normalize_commands_value_array ... ok
test ralph::validation::tests::normalize_commands_value_non_string ... ok
test ralph::validation::tests::discover_commands_returns_empty_when_nothing_configured ... ok
test ralph::validation::tests::normalize_commands_value_null ... ok
test process::tests::missing_executable_is_spawn_failure ... ok
test ralph::validation::tests::normalize_commands_value_string ... ok
test ralph::validation::tests::discover_commands_priority_ito_json_first ... ok
test ralph::validation::tests::project_validation_discovers_commands_from_repo_json ... ok
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
test task_repository::tests::test_get_task_counts_checkbox_format ... ok
test task_repository::tests::test_get_task_counts_enhanced_format ... ok
test task_repository::tests::test_has_tasks ... ok
test task_repository::tests::test_missing_tasks_file_returns_zero ... ok
test tasks::tests::read_tasks_markdown_rejects_traversal_like_change_id ... ok
test tasks::tests::read_tasks_markdown_returns_contents_for_existing_file ... ok
test tasks::tests::read_tasks_markdown_returns_error_for_missing_file ... ok
test tasks::tests::returns_empty_when_no_ready_tasks_exist ... ok
test tasks::tests::returns_ready_tasks_for_ready_changes ... ok
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
test viewer::collector::tests::collect_proposal_artifacts_orders_sections_and_preserves_content ... ok
test viewer::collector::tests::collect_proposal_artifacts_skips_missing_optional_files ... ok
test viewer::html::tests::html_viewer_availability_depends_on_pandoc ... ok
test viewer::html::tests::html_viewer_open_errors_when_pandoc_missing ... ok
test viewer::html::tests::html_viewer_reports_expected_description ... ok
test viewer::html::tests::html_viewer_reports_expected_name ... ok
test viewer::tests::concrete_viewers_report_expected_names ... ok
test viewer::tests::default_registry_includes_html_viewer ... ok
test ralph::validation::tests::shell_timeout_is_failure ... ok
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
test ralph::validation::tests::run_extra_validation_failure ... ok
test ralph::validation::tests::run_extra_validation_success ... ok
test audit::reconcile::tests::reconcile_fix_clears_extra_task_drift ... ok
test audit::reconcile::tests::reconcile_fix_writes_compensating_events ... ok
test viewer::tests::run_with_stdin_closes_pipe_after_write ... ok
test event_forwarder::tests::forward_skips_when_fully_forwarded ... ok
test event_forwarder::tests::forward_sends_all_new_events ... ok
test event_forwarder::tests::forward_stops_on_permanent_failure ... ok
test coordination_worktree::coordination_worktree_tests::integration_create_and_remove_coordination_worktree ... ok
test audit::stream::tests::read_initial_events_returns_last_n ... ok
test audit::store::tests::legacy_worktree_log_is_removed_after_successful_migration ... ok
test coordination_worktree::coordination_worktree_tests::integration_auto_commit_coordination ... ok
test event_forwarder::tests::forward_reads_events_from_routed_local_store ... ok
test audit::store::tests::read_all_merges_and_replays_fallback_events_when_branch_recovers ... ok
test audit::stream::tests::poll_detects_new_events_from_routed_store ... ok

test result: ok. 589 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.80s

     Running tests/archive.rs (target/debug/deps/archive-bb908e19f60ab3de)

running 3 tests
test check_task_completion_handles_checkbox_and_enhanced_formats ... ok
test generate_archive_name_prefixes_with_date ... ok
test discover_and_copy_specs_and_archive_change ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target/debug/deps/audit_mirror-e0f2007be96afc56)

running 6 tests
test audit_mirror_default_local_store_falls_back_without_creating_worktree_log ... ok
test audit_mirror_disabled_does_not_create_remote_branch ... ok
test audit_mirror_failures_do_not_break_local_append ... ok
test local_store_does_not_fall_back_when_internal_branch_exists_without_log_file ... ok
test audit_mirror_default_local_store_writes_to_internal_branch_without_worktree_log ... ok
test audit_mirror_enabled_pushes_to_configured_branch ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.77s

     Running tests/audit_storage.rs (target/debug/deps/audit_storage-d1066ef9170e3525)

running 3 tests
test filters_events_from_injected_store ... ok
test memory_store_append_persists_events ... ok
test reads_events_from_injected_store_without_filesystem_path ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target/debug/deps/backend_archive-7d213b5a86171714)

running 6 tests
test backend_archive_fails_when_pull_unavailable ... ok
test backend_archive_with_skip_specs_does_not_copy_specs ... ok
test backend_archive_happy_path_produces_committable_state ... ok
test backend_archive_does_not_mutate_local_module_markdown ... ok
test backend_archive_fails_when_backend_unavailable_for_mark_archived ... ok
test backend_archive_creates_backup_before_overwriting ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/backend_auth.rs (target/debug/deps/backend_auth-f62b4033e87eb880)

running 13 tests
test resolve_admin_tokens_deduplicates ... ok
test resolve_admin_tokens_merges_all_sources ... ok
test resolve_admin_tokens_skips_empty_config_entries ... ok
test resolve_token_seed_falls_back_to_config ... ok
test resolve_token_seed_returns_none_when_all_empty ... ok
test resolve_token_seed_cli_takes_precedence ... ok
test init_skips_when_tokens_exist ... ok
test write_auth_sets_restrictive_permissions ... ok
test write_auth_rejects_non_object_backend_server ... ok
test init_generates_tokens_when_none_exist ... ok
test write_auth_rejects_non_object_root ... ok
test write_auth_creates_config_file ... ok
test write_auth_preserves_existing_config ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target/debug/deps/backend_auth_service-d4ae41eb0dbebf8d)

running 1 test
test init_rejects_non_object_backend_server ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target/debug/deps/backend_client_mode-4feccb8304cdd0e9)

running 15 tests
test allocate_no_work_returns_none ... ok
test allocate_returns_claimed_change ... ok
test backend_change_repo_lists_and_filters ... ok
test backend_task_repo_missing_returns_zero ... ok
test backend_unavailable_detection ... ok
test claim_conflict_returns_holder_error ... ok
test claim_success_returns_holder_info ... ok
test config_disabled_returns_none ... ok
test config_enabled_missing_token_fails_with_clear_message ... ok
test config_enabled_with_token_resolves ... ok
test retriable_status_codes ... ok
test backend_task_repo_parses_from_content ... ok
test pull_writes_artifacts_and_revision ... ok
test push_stale_revision_gives_actionable_error ... ok
test push_success_updates_local_revision ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target/debug/deps/backend_module_repository-142fb0a15ad04557)

running 5 tests
test backend_module_repository_list_sorts_by_id ... ok
test backend_module_repository_accepts_name_inputs ... ok
test backend_module_repository_list_sorts_deterministically ... ok
test backend_module_repository_normalizes_full_name_inputs ... ok
test read_module_markdown_falls_back_without_local_file ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target/debug/deps/backend_sub_module_support-b6cbe69060670271)

running 9 tests
test backend_module_repository_list_includes_sub_module_summaries ... ok
test backend_module_repository_list_sub_modules_for_unknown_module_returns_error ... ok
test backend_module_repository_get_sub_module_not_found_returns_error ... ok
test backend_module_repository_get_sub_module_by_composite_id ... ok
test backend_module_repository_list_sub_modules_returns_sorted_summaries ... ok
test sqlite_store_persists_sub_module_id_on_change ... ok
test sqlite_store_list_changes_filters_by_sub_module_id ... ok
test sqlite_store_sub_module_change_roundtrips_through_artifact_bundle ... ok
test sqlite_store_legacy_change_has_no_sub_module_id ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target/debug/deps/change_repository_lifecycle-3ad7a416a1aedd7b)

running 2 tests
test remote_runtime_ignores_local_change_dirs ... ok
test filesystem_change_repository_filters_archived ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (target/debug/deps/change_repository_orchestrate_metadata-3b7eeb07eb743455)

running 1 test
test change_repository_exposes_orchestrate_metadata_from_ito_yaml ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target/debug/deps/change_repository_parity-35ea3d4714a6ad6a)

running 18 tests
test backend_list_by_module_normalizes_module_id ... ok
test backend_resolve_lifecycle_filter_respected ... ok
test backend_resolve_empty_input_returns_not_found ... ok
test backend_resolve_numeric_short_form_matches_canonical_id ... ok
test backend_resolve_numeric_short_form_ambiguous ... ok
test backend_resolve_module_scoped_slug_not_found ... ok
test backend_resolve_module_scoped_slug_query ... ok
test sqlite_get_with_archived_filter_returns_not_found ... ok
test sqlite_list_archived_filter_returns_empty ... ok
test sqlite_resolve_numeric_short_form_matches_canonical_id ... ok
test sqlite_resolve_all_filter_finds_active_changes ... ok
test sqlite_get_with_all_filter_finds_change ... ok
test sqlite_resolve_archived_filter_returns_not_found ... ok
test sqlite_list_all_filter_returns_active_changes ... ok
test sqlite_resolve_empty_input_returns_not_found ... ok
test sqlite_resolve_numeric_short_form_ambiguous ... ok
test sqlite_resolve_prefix_match ... ok
test sqlite_list_by_module_normalizes_module_id ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/change_target_resolution_parity.rs (target/debug/deps/change_target_resolution_parity-421699c569d2d487)

running 2 tests
test sqlite_resolver_honors_archived_lifecycle_like_filesystem ... ok
test change_target_resolution_matches_across_repository_modes ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/coordination_worktree.rs (target/debug/deps/coordination_worktree-5975c00a55b1bb53)

running 15 tests
test symlink_tests::module_repo_exists_through_symlink ... ok
test symlink_tests::module_repo_get_through_symlink ... ok
test symlink_tests::change_repo_exists_through_symlink ... ok
test symlink_tests::module_repo_list_through_symlink ... ok
test symlink_tests::change_written_through_symlink_lands_in_worktree ... ok
test symlink_tests::task_repo_missing_tasks_file_returns_zero_through_symlink ... ok
test symlink_tests::change_repo_get_through_symlink ... ok
test symlink_tests::module_repo_list_multiple_through_symlink ... ok
test symlink_tests::change_repo_list_through_symlink ... ok
test symlink_tests::task_repo_has_tasks_through_symlink ... ok
test symlink_tests::task_written_through_symlink_lands_in_worktree ... ok
test symlink_tests::task_repo_load_tasks_through_symlink ... ok
test symlink_tests::module_repo_change_counts_through_symlink ... ok
test symlink_tests::all_repos_consistent_through_symlinks ... ok
test symlink_tests::change_repo_list_multiple_through_symlink ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/create.rs (target/debug/deps/create-5ac295c2581c09f9)

running 15 tests
test create_change_rejects_uppercase_names ... ok
test create_change_in_sub_module_rejects_missing_parent_module ... ok
test create_module_writes_description_to_purpose_section ... ok
test create_module_returns_existing_module_when_name_matches ... ok
test create_module_creates_directory_and_module_md ... ok
test create_change_in_sub_module_rejects_missing_sub_module_dir ... ok
test create_change_rewrites_module_changes_in_ascending_change_id_order ... ok
test create_change_allocates_next_number_from_existing_change_dirs ... ok
test create_change_creates_change_dir_and_updates_module_md ... ok
test create_change_in_sub_module_checklist_is_sorted_ascending ... ok
test create_change_in_sub_module_uses_composite_id_format ... ok
test create_change_in_sub_module_writes_checklist_to_sub_module_md ... ok
test allocation_state_sub_module_keys_sort_after_parent ... ok
test create_change_in_sub_module_allocates_independent_sequence ... ok
test create_change_writes_allocation_modules_in_ascending_id_order ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/distribution.rs (target/debug/deps/distribution-f9b9a052767379fb)

running 11 tests
test codex_manifests_includes_bootstrap_and_skills ... ok
test claude_manifests_includes_hooks_and_skills ... ok
test github_manifests_includes_skills_and_commands ... ok
test opencode_manifests_includes_plugin_and_skills ... ok
test install_manifests_writes_files_to_disk ... ok
test install_manifests_renders_worktree_skill_with_context ... ok
test install_manifests_keeps_non_worktree_placeholders_verbatim ... ok
test install_manifests_make_tmux_skill_scripts_executable ... ok
test install_manifests_renders_worktree_skill_enabled ... ok
test install_manifests_creates_parent_directories ... ok
test all_manifests_use_embedded_assets ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests/event_forwarding.rs (target/debug/deps/event_forwarding-c6970aa933776f62)

running 6 tests
test forward_result_reports_diagnostics ... ok
test permanent_failure_stops_forwarding ... ok
test full_forwarding_workflow ... ok
test batch_boundaries_preserved ... ok
test transient_failure_retried_then_succeeds ... ok
test incremental_forwarding ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s

     Running tests/grep_scopes.rs (target/debug/deps/grep_scopes-9f15bca114771057)

running 4 tests
test grep_scope_change_only_searches_one_change ... ok
test grep_respects_limit_across_scopes ... ok
test grep_scope_all_searches_all_changes ... ok
test grep_scope_module_searches_all_changes_in_module ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target/debug/deps/harness_context-545831b8623ed23b)

running 6 tests
test infer_context_from_cwd_infers_change_from_path ... ok
test infer_context_from_cwd_infers_module_from_ito_modules_path ... ok
test infer_context_from_cwd_returns_no_target_when_inconclusive ... ok
test infer_context_from_cwd_prefers_path_over_git_branch ... ok
test infer_context_from_cwd_infers_change_from_git_branch ... ok
test infer_context_from_cwd_infers_module_from_git_branch ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s

     Running tests/harness_opencode.rs (target/debug/deps/harness_opencode-6f5c13f4dd80d54c)

running 8 tests
test codex_harness_errors_when_codex_missing ... ok
test copilot_harness_errors_when_copilot_missing ... ok
test claude_harness_passes_model_and_allow_all_flags ... ok
test claude_harness_errors_when_claude_missing ... ok
test opencode_harness_errors_when_opencode_missing ... ok
test opencode_harness_runs_opencode_binary_and_returns_outputs ... ok
test github_copilot_harness_passes_model_and_allow_all_flags ... ok
test codex_harness_passes_model_and_allow_all_flags ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.02s

     Running tests/harness_streaming.rs (target/debug/deps/harness_streaming-1c0b5d5428cea5bc)

running 2 tests
test no_timeout_when_process_exits_normally ... ok
test inactivity_timeout_kills_stalled_process ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.26s

     Running tests/harness_stub.rs (target/debug/deps/harness_stub-c30f2a7613933814)

running 6 tests
test stub_harness_default_returns_complete_promise ... ok
test stub_harness_errors_on_empty_steps ... ok
test stub_harness_from_env_prefers_env_over_default ... ok
test stub_harness_errors_on_missing_and_invalid_json ... ok
test stub_step_defaults_match_json_schema ... ok
test stub_harness_from_json_path_runs_steps_and_repeats_last ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/import.rs (target/debug/deps/import-f1b2c7ccf7d0ce7b)

running 10 tests
test skips_already_imported_active_change_when_remote_bundle_matches ... ok
test rerun_archives_existing_remote_active_change_without_repush_when_bundle_matches ... ok
test active_local_change_fails_when_backend_only_has_archived_copy ... ok
test dry_run_previews_without_importing ... ok
test dry_run_uses_preview_logic_without_mutating_backend ... ok
test pushes_when_remote_active_bundle_differs ... ok
test archived_directory_with_empty_canonical_change_id_is_ignored ... ok
test import_summary_records_failures_without_aborting_remaining_changes ... ok
test imports_active_and_archived_changes_with_lifecycle_fidelity ... ok
test ignores_unrecognized_archive_directories_during_discovery ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/io.rs (target/debug/deps/io-2236f3068a97b57f)

running 3 tests
test read_to_string_or_default_returns_empty_for_missing_file ... ok
test read_to_string_optional_returns_none_for_missing_file ... ok
test write_atomic_std_creates_parent_and_replaces_contents ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target/debug/deps/orchestrate_run_state-55a74a408992e8da)

running 7 tests
test orchestrate_max_parallel_aliases_resolve ... ok
test orchestrate_dependency_cycle_is_rejected ... ok
test orchestrate_resume_skips_terminal_gates ... ok
test orchestrate_run_id_generation_matches_expected_format ... ok
test orchestrate_event_log_appends_without_truncation ... ok
test orchestrate_run_state_creates_expected_layout ... ok
test orchestrate_change_state_is_written_and_readable ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target/debug/deps/planning_init-89cbafa119c3ee0a)

running 3 tests
test read_planning_status_returns_error_for_missing_roadmap ... ok
test read_planning_status_returns_contents_for_existing_roadmap ... ok
test init_planning_structure_writes_files ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/ralph.rs (target/debug/deps/ralph-be5f667b52cc59cc)

running 30 tests
test run_ralph_continue_ready_errors_when_no_eligible_changes_but_work_remains ... ok
test run_ralph_errors_when_max_iterations_is_zero ... ok
test run_ralph_continue_ready_errors_when_targeting_change_or_module ... ok
test run_ralph_add_and_clear_context_paths ... ok
test run_ralph_opencode_counts_git_changes_when_in_repo ... ignored, Flaky in pre-commit: counts real uncommitted changes instead of test fixture
test run_ralph_gives_up_after_max_retriable_retries ... ok
test run_ralph_continues_after_harness_failure_by_default ... ok
test run_ralph_continue_ready_errors_when_repo_shifts_to_no_eligible_changes ... ok
test run_ralph_fails_after_error_threshold ... ok
test run_ralph_continue_ready_exits_when_repo_becomes_complete_before_preflight ... ok
test run_ralph_module_resolves_single_change ... ok
test run_ralph_non_retriable_exit_still_counts_against_threshold ... ok
test run_ralph_retries_retriable_exit_code_with_exit_on_error ... ok
test run_ralph_status_path_works_with_no_state ... ok
test run_ralph_prompt_includes_task_context_and_guidance ... ok
test run_ralph_returns_error_on_harness_failure ... ok
test run_ralph_retries_retriable_exit_code_without_counting_against_threshold ... ok
test run_ralph_resets_retriable_counter_on_success ... ok
test run_ralph_module_multiple_changes_errors_when_non_interactive ... ok
test run_ralph_skip_validation_exits_immediately ... ok
test state_helpers_append_and_clear_context ... ok
test run_ralph_continue_ready_reorients_when_repo_state_shifts ... ok
test run_ralph_continue_ready_processes_all_eligible_changes_across_repo ... ok
test run_ralph_continue_module_processes_all_ready_changes ... ok
test run_ralph_continue_ready_accumulates_failures_after_processing_remaining_changes ... ok
test run_ralph_continues_when_completion_validation_fails ... ok
test run_ralph_loop_writes_state_and_honors_min_iterations ... ok
test run_ralph_completion_promise_trims_whitespace ... ok
test run_ralph_worktree_disabled_uses_fallback_cwd ... ok
test run_ralph_worktree_enabled_state_written_to_effective_ito ... ok

test result: ok. 29 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.12s

     Running tests/repo_index.rs (target/debug/deps/repo_index-fd4e043b4d94b0ca)

running 1 test
test repo_index_loads_and_excludes_archive_change_dir ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target/debug/deps/repo_integrity-2d3b3a3e061715a9)

running 3 tests
test invalid_change_dir_names_are_reported ... ok
test change_referring_to_missing_module_is_an_error ... ok
test duplicate_numeric_change_id_is_reported_for_all_conflicting_dirs ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target/debug/deps/repo_paths-fb1f07c85698c734)

running 11 tests
test coordination_worktree_path_correct_structure_with_home_fallback ... ok
test coordination_worktree_path_falls_back_to_local_share_when_xdg_unset ... ok
test coordination_worktree_path_correct_structure_with_xdg ... ok
test coordination_worktree_path_uses_explicit_worktree_path_when_set ... ok
test coordination_worktree_path_last_resort_uses_ito_path ... ok
test coordination_worktree_path_uses_xdg_data_home_when_set ... ok
test coordination_worktree_path_ignores_xdg_when_explicit_path_set ... ok
test resolve_worktree_paths_respects_bare_control_siblings_strategy ... ok
Initialized empty Git repository in /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpyqGzJr/
test resolve_env_from_cwd_errors_in_bare_repo_without_ito_dir ... ok
test resolve_env_from_cwd_uses_nearest_ito_root_when_git_is_unavailable ... ok
Initialized empty Git repository in /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpvLQy2A/.git/
test resolve_env_from_cwd_prefers_git_toplevel ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

     Running tests/repository_runtime.rs (target/debug/deps/repository_runtime-be4c763d081a4e20)

running 6 tests
test remote_runtime_uses_remote_factory ... ok
test sqlite_mode_requires_db_path ... ok
test filesystem_runtime_builds_repository_set ... ok
test sqlite_runtime_builds_repository_set ... ok
test repository_modes_return_consistent_change_names ... ok
test resolve_target_parity_between_filesystem_and_sqlite ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/repository_runtime_config_validation.rs (target/debug/deps/repository_runtime_config_validation-960aa3cc8f00a101)

running 1 test
test invalid_repository_mode_fails_fast ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/show.rs (target/debug/deps/show-22113dd2550b37fc)

running 17 tests
test parse_change_show_json_emits_deltas_with_operations ... ok
test parse_contract_refs_preserves_commas_inside_identifiers ... ok
test parse_delta_spec_requirement_id_is_extracted ... ok
test parse_requirement_block_multiple_requirements_with_ids ... ok
test parse_requirement_metadata_prefers_first_values_and_accepts_asterisk_bullets ... ok
test parse_requirement_block_extracts_requirement_id ... ok
test parse_spec_show_json_extracts_overview_requirements_and_scenarios ... ok
test parse_requirement_block_requirement_id_absent_gives_none ... ok
test read_module_markdown_returns_error_for_nonexistent_module ... ok
test bundle_main_specs_show_json_returns_not_found_when_no_specs_exist ... ok
test load_delta_spec_file_uses_parent_dir_name_as_spec ... ok
test bundle_main_specs_show_json_returns_io_error_when_spec_md_is_missing ... ok
test read_module_markdown_returns_empty_for_missing_module_md ... ok
test read_module_markdown_returns_contents_for_existing_module ... ok
test bundle_main_specs_show_json_is_id_sorted_and_contains_absolute_paths ... ok
test bundle_main_specs_markdown_includes_metadata_comments_and_excludes_deltas ... ok
test read_change_delta_spec_files_lists_specs_sorted ... ok

test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target/debug/deps/spec_repository_backends-c6818389cc643375)

running 2 tests
test remote_runtime_exposes_spec_repository_without_local_specs ... ok
test filesystem_runtime_exposes_promoted_specs ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target/debug/deps/spec_show_repository-b1904cdaddfc9087)

running 3 tests
test bundle_specs_show_json_from_repository_sorts_ids ... ok
test bundle_specs_markdown_from_repository_adds_metadata_comments ... ok
test read_spec_markdown_from_repository_reads_remote_spec ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target/debug/deps/sqlite_archive_mirror-d956e7d08092e7de)

running 1 test
test sqlite_archive_promotes_specs_and_marks_change_archived ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target/debug/deps/sqlite_task_mutations-0b7bbed8ba1f98e7)

running 3 tests
test sqlite_task_mutation_service_returns_not_found_for_missing_tasks ... ok
test sqlite_task_mutation_service_initializes_missing_tasks ... ok
test sqlite_task_mutation_service_updates_existing_markdown ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-67eda39bc41dbd37)

running 2 tests
test compute_command_stats_counts_command_end_events ... ok
test collect_jsonl_files_finds_nested_jsonl_files ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target/debug/deps/task_repository_summary-95f5bd2fb9f29729)

running 1 test
test repository_status_builds_summary_and_next_task ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target/debug/deps/tasks_api-bd8ead6872f8b435)

running 15 tests
test list_ready_tasks_across_changes_handles_empty_repo ... ok
test init_tasks_returns_true_when_file_already_exists ... ok
test init_tasks_creates_file_when_missing ... ok
test tasks_api_rejects_non_tasks_tracking_validator_for_schema_tracking ... ok
test add_task_appends_new_task_with_next_id ... ok
test get_next_task_returns_none_when_all_tasks_complete ... ok
test shelve_task_rejects_shelving_complete_task ... ok
test add_task_creates_wave_if_not_exists ... ok
test complete_task_accepts_note_parameter ... ok
test shelve_task_accepts_reason_parameter ... ok
test start_task_rejects_starting_shelved_task_directly ... ok
test get_next_task_returns_first_ready_task_for_enhanced_format ... ok
test shelve_and_unshelve_task_round_trip_for_enhanced_format ... ok
test start_and_complete_task_enforced_by_dependencies_for_enhanced_format ... ok
test tasks_api_operates_on_schema_apply_tracks_file ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/tasks_checkbox_format.rs (target/debug/deps/tasks_checkbox_format-48281f0d57319aea)

running 3 tests
test checkbox_tasks_do_not_support_shelving ... ok
test checkbox_tasks_enforce_single_in_progress_and_next_task_logic_index_fallback ... ok
test checkbox_tasks_enforce_single_in_progress_and_next_task_logic_explicit_ids ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target/debug/deps/tasks_orchestration-3999942f641c0981)

running 26 tests
test get_task_status_returns_error_when_file_missing ... ok
test init_tasks_rejects_invalid_change_id ... ok
test get_next_task_returns_none_when_all_complete ... ok
test init_tasks_does_not_overwrite_existing_file ... ok
test add_task_rejects_checkbox_format ... ok
test get_next_task_returns_current_in_progress_for_checkbox ... ok
test init_tasks_creates_file_when_missing ... ok
test shelve_task_rejects_checkbox_format ... ok
test complete_task_handles_checkbox_format ... ok
test add_task_assigns_next_id_in_wave ... ok
test get_next_task_returns_first_ready_for_enhanced ... ok
test get_task_status_returns_diagnostics_for_malformed_file ... ok
test add_task_defaults_to_wave_1 ... ok
test complete_task_errors_with_parse_errors ... ok
test shelve_task_rejects_complete_task ... ok
test start_task_errors_with_parse_errors ... ok
test add_task_errors_with_parse_errors ... ok
test shelve_task_errors_with_parse_errors ... ok
test start_task_rejects_shelved_task ... ok
test complete_task_handles_enhanced_format ... ok
test start_task_rejects_already_complete ... ok
test add_task_creates_wave_when_missing ... ok
test start_task_validates_task_is_ready ... ok
test unshelve_task_errors_with_parse_errors ... ok
test unshelve_task_rejects_not_shelved ... ok
test unshelve_task_transitions_to_pending ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-9f2019f85720f737)

running 1 test
test compute_apply_instructions_reports_blocked_states_and_progress ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-35a12530eba5e6f4)

running 2 tests
test compute_change_status_rejects_invalid_change_name ... ok
test compute_change_status_marks_ready_and_blocked_based_on_generated_files ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target/debug/deps/templates_review_context-97f6e8db807d81a0)

running 1 test
test compute_review_context_collects_artifacts_validation_tasks_and_specs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-36c0da440e7dbadb)

running 9 tests
test resolve_schema_rejects_absolute_and_backslash_names ... ok
test resolve_schema_rejects_path_traversal_name ... ok
test resolve_schema_uses_embedded_when_no_overrides_exist ... ok
test resolve_instructions_reads_embedded_templates ... ok
test resolve_instructions_exposes_enhanced_spec_driven_templates ... ok
test resolve_templates_rejects_traversal_template_path ... ok
test resolve_schema_prefers_project_over_user_override ... ok
test resolve_instructions_rejects_traversal_template_path ... ok
test export_embedded_schemas_writes_then_skips_without_force ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-f2f679d65100a642)

running 9 tests
test built_in_minimalist_and_event_driven_spec_templates_use_delta_shape ... ok
test list_schemas_detail_all_sources_are_embedded ... ok
test list_schemas_detail_entries_have_artifacts ... ok
test list_schemas_detail_returns_all_embedded_schemas ... ok
test list_schemas_detail_is_sorted ... ok
test list_schemas_detail_entries_have_descriptions ... ok
test list_schemas_detail_json_round_trips ... ok
test list_schemas_detail_recommended_default_is_spec_driven ... ok
test list_schemas_detail_spec_driven_has_expected_artifacts ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target/debug/deps/templates_user_guidance-cf81a0bcce9439ca)

running 7 tests
test load_user_guidance_for_artifact_rejects_path_traversal_ids ... ok
test load_user_guidance_strips_ito_internal_comment_block ... ok
test load_user_guidance_strips_managed_header_block ... ok
test load_user_guidance_for_artifact_reads_scoped_file ... ok
test load_user_guidance_prefers_user_prompts_guidance_file ... ok
test load_user_guidance_for_artifact_strips_managed_header_block ... ok
test load_composed_user_guidance_combines_scoped_and_shared ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target/debug/deps/traceability_e2e-8fc11d42d04ae5a6)

running 15 tests
test legacy_checkbox_change_validate_passes_without_traceability_checks ... ok
test legacy_checkbox_change_trace_output_is_unavailable ... ok
test traced_change_all_covered_trace_output_is_ready ... ok
test partial_ids_trace_output_is_invalid ... ok
test traced_change_uncovered_req_trace_output_shows_uncovered ... ok
test traced_change_unresolved_ref_trace_output_shows_unresolved ... ok
test shelved_task_leaves_requirement_uncovered ... ok
test duplicate_requirement_ids_trace_output_has_diagnostics ... ok
test traced_change_unresolved_ref_is_error_in_validate ... ok
test partial_ids_validate_reports_error ... ok
test traced_change_uncovered_req_is_error_in_strict ... ok
test duplicate_requirement_ids_produce_error_in_validate ... ok
test traced_change_uncovered_req_is_warning_in_non_strict ... ok
test shelved_task_uncovered_req_is_warning_in_validate ... ok
test traced_change_all_covered_validate_passes ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/validate.rs (target/debug/deps/validate-254d2cfeabee182c)

running 23 tests
test validate_module_errors_when_sub_module_has_invalid_naming ... ok
test validate_spec_markdown_reports_missing_purpose_and_requirements ... ok
test validate_spec_markdown_strict_treats_warnings_as_invalid ... ok
test validate_module_reports_missing_scope_and_short_purpose ... ok
test validate_change_requires_at_least_one_delta ... ok
test validate_change_skips_optional_validator_when_artifact_is_missing ... ok
test validate_module_passes_when_sub_modules_have_valid_module_md ... ok
test validate_change_with_validation_yaml_and_no_delta_validator_does_not_require_deltas ... ok
test validate_tasks_file_returns_error_for_missing_file ... ok
test validate_tasks_file_returns_diagnostics_for_malformed_content ... ok
test validate_module_errors_when_sub_module_missing_module_md ... ok
test validate_tasks_file_returns_empty_for_valid_tasks ... ok
test validate_change_with_unknown_schema_and_no_validation_yaml_does_not_require_deltas ... ok
test validate_change_rejects_unsafe_apply_tracks_for_schema_validation_tracking ... ok
test validate_change_validates_apply_tracks_file_when_configured ... ok
test validate_tasks_file_issues_cite_tasks_tracking_validator_id ... ok
test validate_change_requires_shall_or_must_in_requirement_text ... ok
test validate_module_warns_when_sub_module_purpose_too_short ... ok
test validate_change_rejects_unsafe_apply_tracks_for_legacy_delta_schemas ... ok
test validate_change_uses_validation_yaml_delta_specs_validator_when_configured ... ok
test validate_change_uses_apply_tracks_for_legacy_delta_schemas ... ok
test empty_tracking_file_is_warning_in_non_strict_and_error_in_strict ... ok
test validate_tasks_file_uses_apply_tracks_when_set ... ok

test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-6892e9025fcc274c)

running 11 tests
test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
test scenario_grammar_rule_warns_on_ui_mechanics_but_respects_ui_tags ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error ... ok
test contract_refs_rule_rejects_unknown_schemes ... ok
test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok
test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/validate_rules_extension.rs (target/debug/deps/validate_rules_extension-f05e7c13a4efe547)

running 2 tests
test validation_yaml_proposal_entry_dispatches_rule_configuration ... ok
test validation_yaml_rules_extension_warns_for_unknown_rule_names ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (target/debug/deps/validate_tracking_rules-4408fcfe66def8db)

running 7 tests
test task_quality_rule_emits_single_rule_error_when_tracking_file_is_unreadable ... ok
test task_quality_rule_enforces_done_when_and_verify_for_impl_tasks ... ok
test task_quality_rule_errors_on_missing_status ... ok
test task_quality_rule_respects_warning_floor_without_promoting_advisories ... ok
test task_quality_rule_errors_on_unknown_requirement_ids ... ok
test task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify ... ok
test task_quality_rule_treats_gradle_files_as_implementation_work ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/wiki_install.rs (target/debug/deps/wiki_install-f1869aceda4b9d20)

running 2 tests
test init_upgrade_preserves_existing_wiki_content ... ok
test update_preserves_existing_wiki_content_and_installs_missing_scaffold ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/worktree_ensure_e2e.rs (target/debug/deps/worktree_ensure_e2e-415d396e3697aff4)

running 3 tests
test ensure_worktree_disabled_returns_cwd ... ok
test ensure_worktree_creates_and_initializes_with_include_files ... ok
test ensure_worktree_with_setup_script ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

   Doc-tests ito_core

running 52 tests
test ito-rs/crates/ito-core/src/backend_http.rs - backend_http::task_list_to_parse_result (line 695) ... ignored
test ito-rs/crates/ito-core/src/backend_http.rs - backend_http::task_mutation_from_api (line 832) ... ignored
test ito-rs/crates/ito-core/src/git.rs - git::CoordinationGitError::new (line 53) ... ignored
test ito-rs/crates/ito-core/src/git.rs - git::ensure_coordination_branch_on_origin_with_runner (line 255) ... ignored
test ito-rs/crates/ito-core/src/git.rs - git::ensure_coordination_branch_on_origin (line 134) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::fetch_coordination_branch_with_runner (line 294) ... ignored
test ito-rs/crates/ito-core/src/git.rs - git::fetch_coordination_branch_core (line 155) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::ensure_coordination_branch_on_origin_core (line 226) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::reserve_change_on_coordination_branch (line 107) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::push_coordination_branch_with_runner (line 361) - compile ... ok
test ito-rs/crates/ito-core/src/harness/streaming_cli.rs - harness::streaming_cli::monitor_timeout (line 296) ... ignored
test ito-rs/crates/ito-core/src/harness/types.rs - harness::types::Harness::streams_output (line 202) ... ignored
test ito-rs/crates/ito-core/src/ralph/validation.rs - ralph::validation::run_shell_with_timeout (line 328) ... ignored
test ito-rs/crates/ito-core/src/ralph/runner.rs - ralph::runner::run_ralph (line 188) - compile ... ok
test ito-rs/crates/ito-core/src/show/mod.rs - show::extract_section_text (line 612) ... ignored
test ito-rs/crates/ito-core/src/show/mod.rs - show::parse_requirement_block (line 471) ... ignored
test ito-rs/crates/ito-core/src/tasks.rs - tasks::apply_add_task (line 686) ... ignored
test ito-rs/crates/ito-core/src/tasks.rs - tasks::checked_tasks_path (line 34) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::build_order (line 463) ... ignored
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::compute_change_status (line 369) ... ignored
test ito-rs/crates/ito-core/src/errors.rs - errors::CoreError::sqlite (line 125) ... ok
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
test ito-rs/crates/ito-core/src/errors.rs - errors::CoreError::serde (line 100) ... ok
test ito-rs/crates/ito-core/src/validate/issue.rs - validate::issue (line 8) - compile ... ok
test ito-rs/crates/ito-core/src/git.rs - git::push_coordination_branch_core (line 172) ... ok
test ito-rs/crates/ito-core/src/harness/types.rs - harness::types::HarnessRunResult::is_retriable (line 160) ... ok
test ito-rs/crates/ito-core/src/harness/streaming_cli.rs - harness::streaming_cli::CliHarness (line 22) ... ok
test ito-rs/crates/ito-core/src/harness/github_copilot.rs - harness::github_copilot::GitHubCopilotHarness (line 10) ... ok
test ito-rs/crates/ito-core/src/ralph/duration.rs - ralph::duration::parse_duration (line 16) ... ok
test ito-rs/crates/ito-core/src/tasks.rs - tasks::complete_task (line 829) ... ok
test ito-rs/crates/ito-core/src/harness/opencode.rs - harness::opencode::OpencodeHarness (line 10) ... ok
test ito-rs/crates/ito-core/src/tasks.rs - tasks::start_task (line 792) ... ok
test ito-rs/crates/ito-core/src/harness/claude_code.rs - harness::claude_code::ClaudeCodeHarness (line 10) ... ok
test ito-rs/crates/ito-core/src/harness/codex.rs - harness::codex::CodexHarness (line 10) ... ok
test ito-rs/crates/ito-core/src/templates/mod.rs - templates::list_available_changes (line 157) ... ok
test ito-rs/crates/ito-core/src/git.rs - git::push_coordination_branch (line 84) ... ok
test ito-rs/crates/ito-core/src/git.rs - git::reserve_change_on_coordination_branch_core (line 195) ... ok
test ito-rs/crates/ito-core/src/process.rs - process::SystemProcessRunner::run_with_timeout (line 189) ... ok

test result: ok. 23 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out; finished in 0.06s

all doctests ran in 0.64s; merged doctests compilation took 0.25s
```

After review, added a direct ownership unit assertion for .ito/wiki/index.md alongside the integration preservation tests.

```bash
cargo test -p ito-core installers::json_tests::classify_project_file_ownership_handles_user_owned_paths && cargo test -p ito-core --test wiki_install
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running unittests src/lib.rs (target/debug/deps/ito_core-97a60e89e32a20a6)

running 1 test
test installers::json_tests::classify_project_file_ownership_handles_user_owned_paths ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 588 filtered out; finished in 0.00s

     Running tests/archive.rs (target/debug/deps/archive-bb908e19f60ab3de)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target/debug/deps/audit_mirror-e0f2007be96afc56)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (target/debug/deps/audit_storage-d1066ef9170e3525)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target/debug/deps/backend_archive-7d213b5a86171714)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (target/debug/deps/backend_auth-f62b4033e87eb880)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target/debug/deps/backend_auth_service-d4ae41eb0dbebf8d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target/debug/deps/backend_client_mode-4feccb8304cdd0e9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target/debug/deps/backend_module_repository-142fb0a15ad04557)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target/debug/deps/backend_sub_module_support-b6cbe69060670271)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target/debug/deps/change_repository_lifecycle-3ad7a416a1aedd7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (target/debug/deps/change_repository_orchestrate_metadata-3b7eeb07eb743455)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target/debug/deps/change_repository_parity-35ea3d4714a6ad6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (target/debug/deps/change_target_resolution_parity-421699c569d2d487)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/coordination_worktree.rs (target/debug/deps/coordination_worktree-5975c00a55b1bb53)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/create.rs (target/debug/deps/create-5ac295c2581c09f9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/distribution.rs (target/debug/deps/distribution-f9b9a052767379fb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (target/debug/deps/event_forwarding-c6970aa933776f62)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (target/debug/deps/grep_scopes-9f15bca114771057)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target/debug/deps/harness_context-545831b8623ed23b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (target/debug/deps/harness_opencode-6f5c13f4dd80d54c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (target/debug/deps/harness_streaming-1c0b5d5428cea5bc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (target/debug/deps/harness_stub-c30f2a7613933814)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (target/debug/deps/import-f1b2c7ccf7d0ce7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (target/debug/deps/io-2236f3068a97b57f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target/debug/deps/orchestrate_run_state-55a74a408992e8da)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target/debug/deps/planning_init-89cbafa119c3ee0a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph.rs (target/debug/deps/ralph-be5f667b52cc59cc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s

     Running tests/repo_index.rs (target/debug/deps/repo_index-fd4e043b4d94b0ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target/debug/deps/repo_integrity-2d3b3a3e061715a9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target/debug/deps/repo_paths-fb1f07c85698c734)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (target/debug/deps/repository_runtime-be4c763d081a4e20)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (target/debug/deps/repository_runtime_config_validation-960aa3cc8f00a101)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (target/debug/deps/show-22113dd2550b37fc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target/debug/deps/spec_repository_backends-c6818389cc643375)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target/debug/deps/spec_show_repository-b1904cdaddfc9087)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target/debug/deps/sqlite_archive_mirror-d956e7d08092e7de)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target/debug/deps/sqlite_task_mutations-0b7bbed8ba1f98e7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-67eda39bc41dbd37)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target/debug/deps/task_repository_summary-95f5bd2fb9f29729)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target/debug/deps/tasks_api-bd8ead6872f8b435)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (target/debug/deps/tasks_checkbox_format-48281f0d57319aea)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target/debug/deps/tasks_orchestration-3999942f641c0981)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-9f2019f85720f737)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-35a12530eba5e6f4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target/debug/deps/templates_review_context-97f6e8db807d81a0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-36c0da440e7dbadb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-f2f679d65100a642)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target/debug/deps/templates_user_guidance-cf81a0bcce9439ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target/debug/deps/traceability_e2e-8fc11d42d04ae5a6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/validate.rs (target/debug/deps/validate-254d2cfeabee182c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s

     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-6892e9025fcc274c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/validate_rules_extension.rs (target/debug/deps/validate_rules_extension-f05e7c13a4efe547)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (target/debug/deps/validate_tracking_rules-4408fcfe66def8db)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/wiki_install.rs (target/debug/deps/wiki_install-f1869aceda4b9d20)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_ensure_e2e.rs (target/debug/deps/worktree_ensure_e2e-415d396e3697aff4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/wiki_install.rs (target/debug/deps/wiki_install-f1869aceda4b9d20)

running 2 tests
test update_preserves_existing_wiki_content_and_installs_missing_scaffold ... ok
test init_upgrade_preserves_existing_wiki_content ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```
