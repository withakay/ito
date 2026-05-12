# Task 3.1: Delta Heading Validator

*2026-05-12T16:08:55Z by Showboat 0.6.1*
<!-- showboat-id: 9a4d6bf7-5312-4b41-897e-878115689e47 -->

Added main-spec validation for delta operation headings. Non-strict validation reports format warnings; strict validation reports format errors.

```bash
cargo test -p ito-core --test validate validate_spec_markdown_warns_on_delta_headings_in_main_specs
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running tests/validate.rs (target/debug/deps/validate-5ca6ff709bd886cb)

running 1 test
test validate_spec_markdown_warns_on_delta_headings_in_main_specs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s

```

```bash
cargo test -p ito-core --test validate
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running tests/validate.rs (target/debug/deps/validate-5ca6ff709bd886cb)

running 24 tests
test validate_change_requires_shall_or_must_in_requirement_text ... ok
test validate_change_requires_at_least_one_delta ... ok
test validate_spec_markdown_reports_missing_purpose_and_requirements ... ok
test validate_spec_markdown_strict_treats_warnings_as_invalid ... ok
test validate_spec_markdown_warns_on_delta_headings_in_main_specs ... ok
test validate_module_errors_when_sub_module_missing_module_md ... ok
test validate_change_skips_optional_validator_when_artifact_is_missing ... ok
test validate_module_reports_missing_scope_and_short_purpose ... ok
test validate_change_rejects_unsafe_apply_tracks_for_schema_validation_tracking ... ok
test validate_tasks_file_returns_error_for_missing_file ... ok
test validate_module_passes_when_sub_modules_have_valid_module_md ... ok
test validate_change_with_unknown_schema_and_no_validation_yaml_does_not_require_deltas ... ok
test validate_module_warns_when_sub_module_purpose_too_short ... ok
test validate_module_errors_when_sub_module_has_invalid_naming ... ok
test validate_tasks_file_returns_empty_for_valid_tasks ... ok
test validate_tasks_file_issues_cite_tasks_tracking_validator_id ... ok
test validate_change_validates_apply_tracks_file_when_configured ... ok
test validate_change_with_validation_yaml_and_no_delta_validator_does_not_require_deltas ... ok
test validate_tasks_file_returns_diagnostics_for_malformed_content ... ok
test validate_change_uses_apply_tracks_for_legacy_delta_schemas ... ok
test validate_change_rejects_unsafe_apply_tracks_for_legacy_delta_schemas ... ok
test empty_tracking_file_is_warning_in_non_strict_and_error_in_strict ... ok
test validate_change_uses_validation_yaml_delta_specs_validator_when_configured ... ok
test validate_tasks_file_uses_apply_tracks_when_set ... ok

test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

```

```bash
ito validate 000-11_normalize-main-spec-formatting --strict
```

```output
Change '000-11_normalize-main-spec-formatting' is valid
```

```bash
ito validate --specs --strict
```

```output
All items valid (203 checked)
```

```bash
if rg -n '^## (ADDED|MODIFIED|REMOVED|RENAMED) Requirements' .ito/specs; then exit 1; else printf 'No delta operation headings found in .ito/specs\n'; fi
```

```output
No delta operation headings found in .ito/specs
```
