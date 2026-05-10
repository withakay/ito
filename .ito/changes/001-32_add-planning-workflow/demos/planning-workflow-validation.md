# Planning Workflow Validation

*2026-05-10T19:34:42Z by Showboat 0.6.1*
<!-- showboat-id: 7e451a81-c351-4d64-8c8e-547168196845 -->

Validated the planning workflow change: strict Ito change validation, planning init/status regressions, template asset tests, formatting, and clippy all pass after removing the legacy planning document bootstrap.

```bash
ito validate 001-32_add-planning-workflow --strict
```

```output
Change '001-32_add-planning-workflow' is valid
```

```bash
cargo test -p ito-core --test planning_init -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.39s
     Running tests/planning_init.rs (target/debug/deps/planning_init-a7d9eb36a801cb09)

running 3 tests
test read_planning_workspace_status_allows_missing_workspace ... ok
test init_planning_structure_creates_only_workspace ... ok
test read_planning_workspace_status_lists_plan_documents ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

```bash
cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings -D clippy::dbg_macro -D clippy::todo -D clippy::unimplemented
```

```output
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
```

```bash
cargo test -p ito-templates
```

```output
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Finished `test` profile [optimized + debuginfo] target(s) in 0.25s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-d4acaba2b8f34524)

running 89 tests
test agents::tests::agent_surface_inventory_defines_activation_boundaries ... ok
test agents::tests::render_template_replaces_variant ... ok
test agents::tests::render_template_replaces_model ... ok
test agents::tests::render_template_removes_variant_line_if_not_set ... ok
test agents::tests::default_configs_has_all_combinations ... ok
test agent_surface_tests::agent_templates_declare_activation_contract ... ok
test instructions::tests::list_instruction_templates_is_sorted_and_non_empty ... ok
test instructions::tests::render_instruction_template_returns_not_found_for_missing_template ... ok
test agent_surface_tests::orchestration_adjacent_surfaces_are_classified ... ok
test instructions::tests::render_template_str_is_strict_on_undefined ... ok
test instructions::tests::render_instruction_template_str_trims_block_whitespace ... ok
test instructions::manifesto_tests::manifesto_template_renders_minimal_context ... ok
test instructions::manifesto_tests::manifesto_template_renders_embedded_instruction_entries ... ok
test instructions::tests::artifact_template_renders_when_instruction_is_empty ... ok
test instructions::tests::render_template_str_preserves_trailing_newline ... ok
test instructions::tests::orchestrate_template_renders_authoritative_policy ... ok
test instructions::tests::render_template_str_renders_from_serialize_ctx ... ok
test instructions::tests::apply_template_requires_change_worktree_when_apply_setup_disabled ... ok
test instructions::tests::repo_sweep_template_renders ... ok
test instructions::tests::template_fetchers_work_for_known_and_unknown_paths ... ok
test instructions::tests::schemas_template_includes_fix_and_platform_guidance ... ok
test instructions::tests::archive_template_renders_generic_guidance_without_change ... ok
test instructions::tests::worktree_init_template_includes_fresh_worktree_rules ... ok
test project_templates::tests::default_context_is_disabled ... ok
test instructions::tests::finish_template_includes_archive_check_when_prompt_suppressed ... ok
test instructions::tests::finish_template_prompts_for_archive ... ok
test instructions::tests::new_proposal_template_moves_to_worktree_after_create ... ok
test instructions::tests::apply_template_bare_control_siblings_branches_from_default_branch ... ok
test instructions::tests::archive_template_renders_targeted_instruction_with_change ... ok
test instructions::tests::archive_template_lists_available_changes_in_generic_mode ... ok
test instructions::tests::apply_template_checkout_subdir_branches_from_default_branch ... ok
test instructions::tests::worktrees_template_bare_control_siblings_branches_from_default_branch ... ok
test project_templates::tests::render_project_template_passes_plain_text_through ... ok
test project_templates::tests::render_project_template_passes_non_utf8_through ... ok
test instructions::tests::review_template_renders_conditional_sections ... ok
test project_templates::tests::render_agents_md_with_checkout_siblings ... ok
test instructions::tests::finish_template_includes_capture_reminder_when_memory_capture_configured ... ok
test project_templates::tests::render_agents_md_with_bare_control_siblings ... ok
test project_templates::tests::render_agents_md_with_worktrees_disabled ... ok
test project_templates::tests::render_project_template_renders_conditional ... ok
test project_templates::tests::render_agents_md_with_checkout_subdir ... ok
test project_templates::tests::render_project_template_renders_simple_variable ... ok
test project_templates::tests::render_project_template_strict_on_undefined ... ok
test tests::default_home_files_returns_a_vec ... ok
test tests::agent_templates_remind_harnesses_to_use_ito_patch_and_write_for_active_artifacts ... ok
test tests::default_project_agents_mentions_fix_and_feature_entrypoints ... ok
test tests::default_project_files_contains_expected_files ... ok
test tests::default_project_includes_orchestrate_user_prompt ... ok
test tests::every_shipped_agent_has_ito_prefix ... ok
test tests::every_shipped_agent_is_in_surface_inventory ... ok
test tests::every_shipped_command_has_ito_prefix ... ok
test tests::every_shipped_skill_has_ito_prefix ... ok
test tests::extract_managed_block_preserves_trailing_newline_from_content ... ok
test tests::extract_managed_block_rejects_inline_markers ... ok
test tests::extract_managed_block_returns_empty_for_empty_inner ... ok
test tests::extract_managed_block_returns_inner_content ... ok
test tests::fix_and_feature_commands_are_embedded ... ok
test tests::get_preset_file_returns_contents ... ok
test tests::get_schema_file_returns_contents ... ok
test tests::loop_command_template_uses_ito_loop_command_name ... ok
test tests::loop_skill_template_includes_yaml_frontmatter ... ok
test tests::memory_skill_is_embedded ... ok
test tests::normalize_ito_dir_empty_defaults_to_dot_ito ... ok
test tests::normalize_ito_dir_prefixes_dot ... ok
test tests::every_shipped_markdown_has_managed_markers ... ok
test tests::normalize_ito_dir_rejects_traversal_and_path_separators ... ok
test tests::orchestrate_skills_and_command_are_embedded ... ok
test tests::orchestrator_agent_templates_are_embedded_for_all_harnesses ... ok
test tests::presets_files_contains_orchestrate_builtins ... ok
test tests::proposal_intake_and_routing_skills_are_embedded ... ok
test tests::every_shipped_markdown_has_exactly_one_marker_pair ... ok
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

test result: ok. 89 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/instructions_apply_memory.rs (target/debug/deps/instructions_apply_memory-f32afe5dee7bac9a)

running 2 tests
test apply_template_omits_capture_reminder_when_search_only_configured ... ok
test apply_template_renders_capture_reminder_when_configured ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-3fd1622677f9511d)

running 5 tests
test schema_files_have_managed_markers ... ok
test commands_have_managed_markers ... ok
test default_project_files_have_managed_markers ... ok
test agents_have_managed_markers ... ok
test skills_have_managed_markers ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-d841d44327c1d98e)

running 3 tests
test commands_satisfy_ito_prefix_rule ... ok
test agents_satisfy_ito_prefix_rule ... ok
test skills_satisfy_ito_prefix_rule ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-6478e88bebf2e5f8)

running 8 tests
test stamp_no_op_when_no_managed_block ... ok
test stamp_idempotent_when_same_version ... ok
test stamp_inserts_when_no_existing_stamp ... ok
test stamp_preserves_rest_of_file ... ok
test stamp_rewrites_spaced_stamp_to_canonical ... ok
test stamp_rewrites_older_version_stamp ... ok
test stamp_works_with_frontmatter_before_marker ... ok
test stamp_round_trip_on_real_skill ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-500e867deea20982)

running 1 test
test template_markdown_is_well_formed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-f926080ffcef3297)

running 2 tests
test user_guidance_template_exists_and_has_markers ... ok
test user_prompt_stub_templates_exist ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-f5f602f2dca5de4d)

running 8 tests
test skill_bare_control_siblings ... ok
test skill_disabled ... ok
test agents_md_disabled ... ok
test skill_checkout_subdir ... ok
test agents_md_bare_control_siblings ... ok
test skill_checkout_siblings ... ok
test agents_md_checkout_subdir ... ok
test agents_md_checkout_siblings ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_templates

running 7 tests
test ito-rs/crates/ito-templates/src/lib.rs - get_command_file (line 176) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - commands_files (line 110) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_adapter_file (line 94) ... ok
test ito-rs/crates/ito-templates/src/project_templates.rs - project_templates::WorktreeTemplateContext::default (line 47) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - schema_files (line 126) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_schema_file (line 159) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_skill_file (line 77) ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 6.09s; merged doctests compilation took 0.30s
```

```bash
cargo test -p ito-cli plan -- --nocapture
```

```output
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on artifact directory
    Finished `test` profile [optimized + debuginfo] target(s) in 0.44s
     Running unittests src/main.rs (target/debug/deps/ito-433b715f15ce9eb4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 88 filtered out; finished in 0.00s

     Running tests/agent_instruction_apply_sync.rs (target/debug/deps/agent_instruction_apply_sync-b73cf58d6121c110)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_bootstrap.rs (target/debug/deps/agent_instruction_bootstrap-d71231d190cecc4b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_context.rs (target/debug/deps/agent_instruction_context-04fa9d6916e87a15)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_memory.rs (target/debug/deps/agent_instruction_memory-cbb071b369f0474a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/agent_instruction_orchestrate.rs (target/debug/deps/agent_instruction_orchestrate-1956cf17c511de95)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_repo_sweep.rs (target/debug/deps/agent_instruction_repo_sweep-dc28da3551354307)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/agent_instruction_worktrees.rs (target/debug/deps/agent_instruction_worktrees-6d330a49bcef9234)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/aliases.rs (target/debug/deps/aliases-14c70cee4c6466d5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/archive_completed.rs (target/debug/deps/archive_completed-fa98a609d7bc0d5e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/archive_remote_mode.rs (target/debug/deps/archive_remote_mode-be26b0deff4446f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/archive_smoke.rs (target/debug/deps/archive_smoke-5e64fd9ee7cac95d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/artifact_mutations.rs (target/debug/deps/artifact_mutations-8ce977d44ca78c9b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_more.rs (target/debug/deps/audit_more-858c92ec94ea081d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_remote_mode.rs (target/debug/deps/audit_remote_mode-f27c11b794aa154e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_import.rs (target/debug/deps/backend_import-9d35ba2e2b9c9d97)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/backend_qa_walkthrough.rs (target/debug/deps/backend_qa_walkthrough-e33ebf3d6e7c6313)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_serve.rs (target/debug/deps/backend_serve-d5be3d02b79ac461)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_status_more.rs (target/debug/deps/backend_status_more-2175f6936ee3e8a6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 20 filtered out; finished in 0.00s

     Running tests/cli_smoke.rs (target/debug/deps/cli_smoke-8715f50eeeafa9b0)

running 1 test
test create_workflow_plan_state_config_smoke ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 6.42s

     Running tests/cli_snapshots.rs (target/debug/deps/cli_snapshots-a9961412d138cb5a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/config_more.rs (target/debug/deps/config_more-f147277a45d12a55)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/coverage_smoke.rs (target/debug/deps/coverage_smoke-e50030a47e559f69)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/create_more.rs (target/debug/deps/create_more-97c8984ad2bd69b1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/grep_more.rs (target/debug/deps/grep_more-b1f8bdd2394032b4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/help.rs (target/debug/deps/help-7edcee4d2ab25d1d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/init_agent_activation.rs (target/debug/deps/init_agent_activation-2a11eeaf640dd1d6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/init_coordination.rs (target/debug/deps/init_coordination-47531c7121dbd10d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/init_gitignore_session_json.rs (target/debug/deps/init_gitignore_session_json-e5cca16bb1e4c011)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/init_more.rs (target/debug/deps/init_more-564d0e1709573c89)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.00s

     Running tests/init_obsolete_cleanup.rs (target/debug/deps/init_obsolete_cleanup-aef53d2d818a6f14)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/init_tmux.rs (target/debug/deps/init_tmux-058f884714936d7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/init_upgrade_more.rs (target/debug/deps/init_upgrade_more-0d8d7ad8d4fbe3be)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/instructions_more.rs (target/debug/deps/instructions_more-57a13f0368afdeba)

running 2 tests
test agent_instruction_manifesto_planning_profile_is_advisory ... ok
test agent_instruction_manifesto_planning_profile_embeds_no_mutating_artifacts ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.09s

     Running tests/list_archive.rs (target/debug/deps/list_archive-aa095708a39e6d3d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/list_regression.rs (target/debug/deps/list_regression-1d992335e041b3a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/misc_more.rs (target/debug/deps/misc_more-7aa0bbbc6562dba7)

running 1 test
test plan_status_reports_missing_workspace ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.02s

     Running tests/new_more.rs (target/debug/deps/new_more-982202ef8adaa9bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/parity_help_version.rs (target/debug/deps/parity_help_version-92411a874d0c6e5d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/parity_tasks.rs (target/debug/deps/parity_tasks-9a5e548867f23ba8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/path_more.rs (target/debug/deps/path_more-044be90fa762924a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/plan_state_more.rs (target/debug/deps/plan_state_more-f36178f6b4aed8bf)

running 4 tests
test plan_status_reports_missing_workspace_without_error ... ok
test plan_status_lists_markdown_documents ... ok
test plan_init_creates_structure ... ok
test plan_status_succeeds_after_init ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s

     Running tests/ralph_smoke.rs (target/debug/deps/ralph_smoke-8c28a8c8bfad5b39)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/serve_more.rs (target/debug/deps/serve_more-8d1cdde98a7bffa8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show_specs_bundle.rs (target/debug/deps/show_specs_bundle-d8c98dbf360e5015)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/show_specs_remote_mode.rs (target/debug/deps/show_specs_remote_mode-c1eac5c6994391aa)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/source_file_size.rs (target/debug/deps/source_file_size-f6250e3905bf836f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-7de066585490e41f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_more.rs (target/debug/deps/tasks_more-db184e01375bf4b5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/tasks_remote_mode.rs (target/debug/deps/tasks_remote_mode-37cec52258f81d1d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_schemas_export.rs (target/debug/deps/templates_schemas_export-55dbbb0724f69cd6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/trace_more.rs (target/debug/deps/trace_more-76b2116c5b9f8002)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/update_marker_scoped.rs (target/debug/deps/update_marker_scoped-35b704848df1ce9a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/update_smoke.rs (target/debug/deps/update_smoke-6ce5e7bb13bc470a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/user_guidance_injection.rs (target/debug/deps/user_guidance_injection-f0ab387e401f400e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/validate_more.rs (target/debug/deps/validate_more-d9c991798e7fbe93)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/validate_repo_cli.rs (target/debug/deps/validate_repo_cli-d7e6b8b4844b229d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

     Running tests/view_proposal.rs (target/debug/deps/view_proposal-a70309d12012718e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/worktree_validate.rs (target/debug/deps/worktree_validate-fdc6f4a293e6901e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

```
