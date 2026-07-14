# Task 2.3: Wiki Skill Distribution

*2026-04-26T20:28:09Z by Showboat 0.6.1*
<!-- showboat-id: 4c44bb09-9307-4dbb-b37e-15b1499f35ba -->

Added explicit tests that the ito-wiki and ito-wiki-search skills are embedded and included in every supported harness manifest.

```bash
cargo test -p ito-templates wiki_skills_are_embedded && cargo test -p ito-core --test distribution wiki_skills_are_distributed_to_all_harnesses
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-43511d335e81e446)

running 1 test
test tests::wiki_skills_are_embedded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 84 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-4be66a48dfefacf5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-89f6f29b2c677eb1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-c542d94a0d9bbd52)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-354bb8adddb77ade)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-d45bf1384b899f95)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/wiki_scaffold.rs (target/debug/deps/wiki_scaffold-78e3336b2f711952)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-ea6b170a0185265d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/distribution.rs (target/debug/deps/distribution-f9b9a052767379fb)

running 1 test
test wiki_skills_are_distributed_to_all_harnesses ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

```

```bash
cargo test -p ito-templates && cargo test -p ito-core --test distribution
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-43511d335e81e446)

running 85 tests
test agents::tests::render_template_replaces_model ... ok
test agents::tests::render_template_removes_variant_line_if_not_set ... ok
test agents::tests::default_configs_has_all_combinations ... ok
test agents::tests::render_template_replaces_variant ... ok
test instructions::tests::list_instruction_templates_is_sorted_and_non_empty ... ok
test instructions::tests::archive_template_renders_generic_guidance_without_change ... ok
test instructions::tests::archive_template_lists_available_changes_in_generic_mode ... ok
test instructions::tests::archive_template_renders_targeted_instruction_with_change ... ok
test instructions::tests::artifact_template_renders_when_instruction_is_empty ... ok
test instructions::tests::finish_template_includes_archive_check_when_prompt_suppressed ... ok
test instructions::tests::finish_template_prompts_for_archive ... ok
test instructions::tests::finish_template_includes_capture_reminder_when_memory_capture_configured ... ok
test instructions::tests::render_instruction_template_returns_not_found_for_missing_template ... ok
test instructions::tests::apply_template_requires_change_worktree_when_apply_setup_disabled ... ok
test instructions::tests::render_template_str_is_strict_on_undefined ... ok
test instructions::tests::render_template_str_preserves_trailing_newline ... ok
test instructions::tests::apply_template_renders_capture_reminder_when_configured ... ok
test instructions::tests::render_template_str_renders_from_serialize_ctx ... ok
test instructions::tests::template_fetchers_work_for_known_and_unknown_paths ... ok
test instructions::tests::orchestrate_template_renders ... ok
test instructions::tests::render_instruction_template_str_trims_block_whitespace ... ok
test instructions::tests::new_proposal_template_moves_to_worktree_after_create ... ok
test instructions::tests::apply_template_bare_control_siblings_branches_from_default_branch ... ok
test instructions::tests::repo_sweep_template_renders ... ok
test instructions::tests::apply_template_checkout_subdir_branches_from_default_branch ... ok
test instructions::tests::apply_template_omits_capture_reminder_when_search_only_configured ... ok
test instructions::tests::schemas_template_includes_fix_and_platform_guidance ... ok
test project_templates::tests::default_context_is_disabled ... ok
test instructions::tests::worktree_init_template_includes_fresh_worktree_rules ... ok
test instructions::tests::review_template_renders_conditional_sections ... ok
test project_templates::tests::render_project_template_passes_non_utf8_through ... ok
test project_templates::tests::render_project_template_passes_plain_text_through ... ok
test instructions::tests::worktrees_template_bare_control_siblings_branches_from_default_branch ... ok
test project_templates::tests::render_agents_md_with_checkout_siblings ... ok
test project_templates::tests::render_agents_md_with_checkout_subdir ... ok
test project_templates::tests::render_project_template_renders_simple_variable ... ok
test project_templates::tests::render_agents_md_with_bare_control_siblings ... ok
test tests::default_home_files_returns_a_vec ... ok
test project_templates::tests::render_agents_md_with_worktrees_disabled ... ok
test project_templates::tests::render_project_template_renders_conditional ... ok
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
test tests::get_schema_file_returns_contents ... ok
test tests::loop_command_template_uses_ito_loop_command_name ... ok
test tests::every_shipped_markdown_has_managed_markers ... ok
test tests::loop_skill_template_includes_yaml_frontmatter ... ok
test tests::memory_skill_is_embedded ... ok
test tests::normalize_ito_dir_empty_defaults_to_dot_ito ... ok
test tests::normalize_ito_dir_prefixes_dot ... ok
test tests::normalize_ito_dir_rejects_traversal_and_path_separators ... ok
test tests::every_shipped_markdown_has_exactly_one_marker_pair ... ok
test tests::orchestrate_skills_and_command_are_embedded ... ok
test tests::presets_files_contains_orchestrate_builtins ... ok
test tests::orchestrator_agent_templates_are_embedded_for_all_harnesses ... ok
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
test tests::wiki_skills_are_embedded ... ok

test result: ok. 85 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-4be66a48dfefacf5)

running 5 tests
test commands_have_managed_markers ... ok
test schema_files_have_managed_markers ... ok
test default_project_files_have_managed_markers ... ok
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
test stamp_rewrites_older_version_stamp ... ok
test stamp_preserves_rest_of_file ... ok
test stamp_rewrites_spaced_stamp_to_canonical ... ok
test stamp_round_trip_on_real_skill ... ok
test stamp_works_with_frontmatter_before_marker ... ok

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
test skill_checkout_subdir ... ok
test skill_checkout_siblings ... ok
test skill_bare_control_siblings ... ok
test agents_md_disabled ... ok
test agents_md_checkout_siblings ... ok
test agents_md_checkout_subdir ... ok
test agents_md_bare_control_siblings ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests ito_templates

running 7 tests
test ito-rs/crates/ito-templates/src/project_templates.rs - project_templates::WorktreeTemplateContext::default (line 47) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_schema_file (line 156) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - schema_files (line 123) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_skill_file (line 74) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_adapter_file (line 91) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - commands_files (line 107) ... ok
test ito-rs/crates/ito-templates/src/lib.rs - get_command_file (line 173) ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

all doctests ran in 0.44s; merged doctests compilation took 0.16s
    Finished `test` profile [optimized + debuginfo] target(s) in 0.17s
     Running tests/distribution.rs (target/debug/deps/distribution-f9b9a052767379fb)

running 12 tests
test codex_manifests_includes_bootstrap_and_skills ... ok
test claude_manifests_includes_hooks_and_skills ... ok
test github_manifests_includes_skills_and_commands ... ok
test opencode_manifests_includes_plugin_and_skills ... ok
test wiki_skills_are_distributed_to_all_harnesses ... ok
test install_manifests_renders_worktree_skill_enabled ... ok
test install_manifests_keeps_non_worktree_placeholders_verbatim ... ok
test install_manifests_renders_worktree_skill_with_context ... ok
test install_manifests_make_tmux_skill_scripts_executable ... ok
test install_manifests_writes_files_to_disk ... ok
test install_manifests_creates_parent_directories ... ok
test all_manifests_use_embedded_assets ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

```
