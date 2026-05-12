# Final Validation Repair

*2026-05-11T09:58:02Z by Showboat 0.6.1*
<!-- showboat-id: 7a217412-cf46-47cf-9c7a-e59fa4f7a686 -->

Verified the final 001-33 repair batch: scenario grammar accepts plain keywords, contract-ref parsing surfaces unknown schemes while preserving short incidental colon fragments, and Change Shape Public Contract parsing accepts dash or star bullets.

```bash
cargo test -p ito-core --test show && cargo test -p ito-core --test validate_delta_rules
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.21s
     Running tests/show.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/target/debug/deps/show-2dc99ecb9e0315f2)

running 22 tests
test parse_requirement_block_multiple_requirements_with_ids ... ok
test parse_requirement_block_requirement_id_absent_gives_none ... ok
test parse_requirement_block_extracts_requirement_id ... ok
test parse_delta_spec_requirement_id_is_extracted ... ok
test parse_contract_refs_splits_lowercase_unknown_scheme_after_known_ref ... ok
test parse_contract_refs_splits_unknown_schemes_at_length_threshold ... ok
test parse_contract_refs_preserves_commas_inside_identifiers ... ok
test parse_contract_refs_accepts_comma_without_space_before_known_scheme ... ok
test parse_contract_refs_splits_unknown_scheme_after_known_ref ... ok
test parse_change_show_json_emits_deltas_with_operations ... ok
test parse_contract_refs_preserves_lowercase_colon_text_inside_identifiers ... ok
test parse_requirement_metadata_prefers_first_values_and_accepts_asterisk_bullets ... ok
test parse_spec_show_json_extracts_overview_requirements_and_scenarios ... ok
test bundle_main_specs_show_json_returns_not_found_when_no_specs_exist ... ok
test load_delta_spec_file_uses_parent_dir_name_as_spec ... ok
test read_module_markdown_returns_error_for_nonexistent_module ... ok
test bundle_main_specs_show_json_returns_io_error_when_spec_md_is_missing ... ok
test read_module_markdown_returns_empty_for_missing_module_md ... ok
test read_module_markdown_returns_contents_for_existing_module ... ok
test bundle_main_specs_markdown_includes_metadata_comments_and_excludes_deltas ... ok
test bundle_main_specs_show_json_is_id_sorted_and_contains_absolute_paths ... ok
test read_change_delta_spec_files_lists_specs_sorted ... ok

test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.18s
     Running tests/validate_delta_rules.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 14 tests
test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
test ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error ... ok
test contract_refs_rule_rejects_lowercase_unknown_scheme_after_known_ref ... ok
test scenario_grammar_rule_accepts_steps_without_bullets ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test scenario_grammar_rule_warns_on_ui_mechanics_only_for_ui_tags ... ok
test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
test contract_refs_rule_rejects_unknown_scheme_after_known_ref ... ok
test contract_refs_rule_rejects_unknown_schemes ... ok
test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok
test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

```

```bash
make check
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
cargo test with coverage (ito-rs)........................................Passed
cargo test affected (ito-rs).............................................Passed
check max lines (ito-rs).................................................Passed
architecture guardrails..................................................Passed
cargo deny (license/advisory checks).....................................Passed
```
