# Final Validator Fixes

*2026-05-11T23:59:19Z by Showboat 0.6.1*
<!-- showboat-id: 44ceae6f-8240-4aba-b22f-8e2b0cc9b4fa -->

Fixed DDD discovery validator edge cases found during final review: placeholder standalone artifacts now fall through to embedded handoffs, embedded candidate paths are deterministic, rejected-alias declarations no longer self-trigger, and the ito-plan command asset is embedded with the skill.

```bash
cargo test -p ito-core --test validate_domain_discovery_rules
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.19s
     Running tests/validate_domain_discovery_rules.rs (target/debug/deps/validate_domain_discovery_rules-adec62e6c5766afb)

running 10 tests
test context_boundary_consistency_rule_is_silent_for_single_context_discovery ... ok
test ubiquitous_language_rule_ignores_rejected_alias_declaration_itself ... ok
test context_boundary_consistency_rule_passes_for_explicit_relationship_and_translation ... ok
test context_boundary_consistency_rule_warns_for_incomplete_cross_context_framing ... ok
test placeholder_standalone_discovery_does_not_hide_embedded_handoff ... ok
test context_boundary_consistency_rule_uses_header_columns_for_owner_tables ... ok
test domain_rules_are_silent_without_domain_discovery_handoff ... ok
test domain_rules_can_run_from_artifact_rules_for_event_driven_schemas ... ok
test domain_rules_parse_embedded_compact_discovery_handoff ... ok
test domain_rules_parse_embedded_full_discovery_sections ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

```

```bash
cargo test -p ito-templates
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-7eaa5889a2394c40)

running 85 tests
test agents::tests::render_template_replaces_variant ... ok
test agents::tests::default_configs_has_all_combinations ... ok
test instructions::tests::list_instruction_templates_is_sorted_and_non_empty ... ok
test agents::tests::render_template_removes_variant_line_if_not_set ... ok
test agents::tests::render_template_replaces_model ... ok
test instructions::tests::render_instruction_template_returns_not_found_for_missing_template ... ok
test instructions::tests::render_template_str_is_strict_on_undefined ... ok
test instructions::tests::finish_template_includes_capture_reminder_when_memory_capture_configured ... ok
test instructions::tests::archive_template_renders_generic_guidance_without_change ... ok
test instructions::tests::archive_template_renders_targeted_instruction_with_change ... ok
test instructions::manifesto_tests::manifesto_template_renders_minimal_context ... ok
test instructions::tests::orchestrate_template_renders ... ok
test instructions::tests::render_instruction_template_str_trims_block_whitespace ... ok
test instructions::tests::archive_template_lists_available_changes_in_generic_mode ... ok
test instructions::tests::finish_template_prompts_for_archive ... ok
test instructions::tests::new_proposal_template_moves_to_worktree_after_create ... ok
test instructions::tests::apply_template_checkout_subdir_branches_from_default_branch ... ok
test instructions::manifesto_tests::manifesto_template_renders_embedded_instruction_entries ... ok
test instructions::tests::apply_template_bare_control_siblings_branches_from_default_branch ... ok
test instructions::tests::finish_template_includes_archive_check_when_prompt_suppressed ... ok
test instructions::tests::apply_template_requires_change_worktree_when_apply_setup_disabled ... ok
test instructions::tests::artifact_template_renders_when_instruction_is_empty ... ok
test instructions::tests::render_template_str_preserves_trailing_newline ... ok
test instructions::tests::render_template_str_renders_from_serialize_ctx ... ok
test instructions::tests::repo_sweep_template_renders ... ok
test instructions::tests::template_fetchers_work_for_known_and_unknown_paths ... ok
test instructions::tests::schemas_template_includes_fix_and_platform_guidance ... ok
test instructions::tests::worktree_init_template_includes_fresh_worktree_rules ... ok
test project_templates::tests::default_context_is_disabled ... ok
test instructions::tests::review_template_renders_conditional_sections ... ok
test instructions::tests::worktrees_template_bare_control_siblings_branches_from_default_branch ... ok
test project_templates::tests::render_project_template_passes_non_utf8_through ... ok
test project_templates::tests::render_project_template_passes_plain_text_through ... ok
test project_templates::tests::render_project_template_renders_simple_variable ... ok
test project_templates::tests::render_project_template_renders_conditional ... ok
test project_templates::tests::render_project_template_strict_on_undefined ... ok
test tests::extract_managed_block_preserves_trailing_newline_from_content ... ok
test tests::extract_managed_block_rejects_inline_markers ... ok
test tests::default_project_files_contains_expected_files ... ok
test tests::default_home_files_returns_a_vec ... ok
test tests::default_project_includes_orchestrate_user_prompt ... ok
test tests::every_shipped_agent_has_ito_prefix ... ok
test project_templates::tests::render_agents_md_with_checkout_siblings ... ok
test project_templates::tests::render_agents_md_with_worktrees_disabled ... ok
test project_templates::tests::render_agents_md_with_checkout_subdir ... ok
test tests::default_project_agents_mentions_fix_and_feature_entrypoints ... ok
test tests::every_shipped_command_has_ito_prefix ... ok
test project_templates::tests::render_agents_md_with_bare_control_siblings ... ok
test tests::every_shipped_skill_has_ito_prefix ... ok
test tests::agent_templates_remind_harnesses_to_use_ito_patch_and_write_for_active_artifacts ... ok
test tests::extract_managed_block_returns_inner_content ... ok
test tests::extract_managed_block_returns_empty_for_empty_inner ... ok
test tests::fix_and_feature_commands_are_embedded ... ok
test tests::get_preset_file_returns_contents ... ok
test tests::get_schema_file_returns_contents ... ok
test tests::loop_command_template_uses_ito_loop_command_name ... ok
test tests::loop_skill_template_includes_yaml_frontmatter ... ok
test tests::memory_skill_is_embedded ... ok
test tests::every_shipped_markdown_has_managed_markers ... ok
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
test tests::stamp_version_idempotent_on_canonical_match ... ok
test tests::every_shipped_markdown_has_exactly_one_marker_pair ... ok
test tests::stamp_version_handles_prerelease_semver ... ok
test tests::stamp_version_idempotent_on_canonical_with_trailing_whitespace ... ok
test tests::stamp_version_inserts_when_missing ... ok
test tests::stamp_version_noop_without_marker ... ok
test tests::stamp_version_preserves_frontmatter ... ok
test tests::stamp_version_preserves_trailing_content ... ok
test tests::stamp_version_rewrites_spaced_form_to_canonical ... ok
test tests::stamp_version_round_trip_on_real_skill ... ok
test tests::stamp_version_rewrites_older_version ... ok
test tests::tmux_skill_and_scripts_are_embedded ... ok

test result: ok. 85 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/instructions_apply_memory.rs (target/debug/deps/instructions_apply_memory-d1e1807ce87f211a)

running 2 tests
test apply_template_omits_capture_reminder_when_search_only_configured ... ok
test apply_template_renders_capture_reminder_when_configured ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-7a5705c6aff70672)

running 5 tests
test commands_have_managed_markers ... ok
test schema_files_have_managed_markers ... ok
test agents_have_managed_markers ... ok
test default_project_files_have_managed_markers ... ok
test skills_have_managed_markers ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-b0565b06adac7694)

running 3 tests
test commands_satisfy_ito_prefix_rule ... ok
test agents_satisfy_ito_prefix_rule ... ok
test skills_satisfy_ito_prefix_rule ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-95514587e0df9f18)

running 8 tests
test stamp_inserts_when_no_existing_stamp ... ok
test stamp_idempotent_when_same_version ... ok
test stamp_preserves_rest_of_file ... ok
test stamp_no_op_when_no_managed_block ... ok
test stamp_rewrites_older_version_stamp ... ok
test stamp_rewrites_spaced_stamp_to_canonical ... ok
test stamp_round_trip_on_real_skill ... ok
test stamp_works_with_frontmatter_before_marker ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-bc4ea5e74b0d0fe5)

running 1 test
test template_markdown_is_well_formed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-36770e8c31892375)

running 2 tests
test user_guidance_template_exists_and_has_markers ... ok
test user_prompt_stub_templates_exist ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-825fa89e3cbc9b79)

running 8 tests
test agents_md_disabled ... ok
test skill_disabled ... ok
test skill_checkout_siblings ... ok
test agents_md_bare_control_siblings ... ok
test agents_md_checkout_siblings ... ok
test agents_md_checkout_subdir ... ok
test skill_checkout_subdir ... ok
test skill_bare_control_siblings ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_templates

running 7 tests
test ito-rs/crates/ito-templates/src/lib.rs - get_command_file (line 173) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - commands_files (line 107) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_skill_file (line 74) ... ok
test ito-rs/crates/ito-templates/src/project_templates.rs - project_templates::WorktreeTemplateContext::default (line 47) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_adapter_file (line 91) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - schema_files (line 123) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_schema_file (line 156) ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.93s; merged doctests compilation took 0.67s
```

```bash
ito validate 001-34_add-ddd-discovery-workflow --type change
```

```output
Change '001-34_add-ddd-discovery-workflow' is valid
```
