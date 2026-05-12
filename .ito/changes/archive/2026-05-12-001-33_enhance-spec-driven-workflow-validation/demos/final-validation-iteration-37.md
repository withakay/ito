# Final Validation: Iteration 37

*2026-05-11T18:38:27Z by Showboat 0.6.1*
<!-- showboat-id: 28697fd1-7dcb-455b-a92e-5aa78d3ca1fa -->

Added a focused regression test for short lowercase colon text in contract refs and documented scenario list-marker stripping.

```bash
cargo test -p ito-core --test show --test validate_delta_rules --test validate_tracking_rules && cargo test -p ito-domain --test tasks
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.22s
     Running tests/show.rs (target/debug/deps/show-2dc99ecb9e0315f2)

running 25 tests
test parse_contract_refs_preserves_commas_inside_identifiers ... ok
test parse_contract_refs_preserves_lowercase_colon_text_inside_identifiers ... ok
test parse_contract_refs_splits_unknown_schemes_at_length_threshold ... ok
test parse_contract_refs_accepts_comma_without_space_before_known_scheme ... ok
test parse_contract_refs_splits_unknown_scheme_after_known_ref ... ok
test parse_change_show_json_preserves_requirement_scoped_rules_and_state_transitions ... ok
test parse_delta_spec_requirement_id_is_extracted ... ok
test parse_contract_refs_splits_short_unknown_scheme_with_ref_like_identifier ... ok
test parse_contract_refs_preserves_short_lowercase_colon_text ... ok
test parse_change_show_json_emits_deltas_with_operations ... ok
test parse_contract_refs_splits_lowercase_unknown_scheme_after_known_ref ... ok
test parse_requirement_block_extracts_requirement_id ... ok
test parse_requirement_block_multiple_requirements_with_ids ... ok
test parse_requirement_block_requirement_id_absent_gives_none ... ok
test parse_requirement_metadata_prefers_first_values_and_accepts_asterisk_bullets ... ok
test parse_spec_show_json_extracts_overview_requirements_and_scenarios ... ok
test load_delta_spec_file_uses_parent_dir_name_as_spec ... ok
test bundle_main_specs_show_json_returns_not_found_when_no_specs_exist ... ok
test read_module_markdown_returns_error_for_nonexistent_module ... ok
test bundle_main_specs_show_json_returns_io_error_when_spec_md_is_missing ... ok
test read_module_markdown_returns_empty_for_missing_module_md ... ok
test read_module_markdown_returns_contents_for_existing_module ... ok
test bundle_main_specs_show_json_is_id_sorted_and_contains_absolute_paths ... ok
test bundle_main_specs_markdown_includes_metadata_comments_and_excludes_deltas ... ok
test read_change_delta_spec_files_lists_specs_sorted ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 18 tests
test capabilities_consistency_rule_warns_on_invalid_change_shape_values ... ok
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test scenario_grammar_rule_accepts_ordered_list_steps ... ok
test contract_refs_rule_rejects_lowercase_unknown_scheme_after_known_ref ... ok
test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok
test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok
test scenario_grammar_rule_accepts_steps_without_bullets ... ok
test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
test contract_refs_rule_rejects_short_unknown_scheme_after_known_ref ... ok
test contract_refs_rule_rejects_unknown_schemes ... ok
test contract_refs_rule_rejects_unknown_scheme_after_known_ref ... ok
test scenario_grammar_rule_accepts_asterisk_bullets ... ok
test ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error ... ok
test ui_mechanics_rule_warns_only_for_ui_tags ... ok

test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/validate_tracking_rules.rs (target/debug/deps/validate_tracking_rules-5c752bbcd3cd2621)

running 8 tests
test task_quality_rule_emits_single_rule_error_when_tracking_file_is_unreadable ... ok
test task_quality_rule_enforces_done_when_and_verify_for_impl_tasks ... ok
test task_quality_rule_treats_gradle_files_as_implementation_work ... ok
test task_quality_rule_errors_on_missing_status ... ok
test task_quality_rule_allows_checkpoint_without_files ... ok
test task_quality_rule_errors_on_unknown_requirement_ids ... ok
test task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify ... ok
test task_quality_rule_respects_warning_floor_without_promoting_advisories ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/tasks.rs (target/debug/deps/tasks-5cb039ab908fe47d)

running 2 tests
test update_enhanced_task_status_inserts_or_replaces_status_line ... ok
test enhanced_template_parses_and_has_checkpoint_warning ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```

```bash
ito validate --changes 001-33_enhance-spec-driven-workflow-validation && ito audit reconcile --change 001-33_enhance-spec-driven-workflow-validation
```

```output
All items valid (14 checked)
Reconcile: 001-33_enhance-spec-driven-workflow-validation
──────────────────────────────────────────────────
No drift detected. Audit log and files are in sync.
```
