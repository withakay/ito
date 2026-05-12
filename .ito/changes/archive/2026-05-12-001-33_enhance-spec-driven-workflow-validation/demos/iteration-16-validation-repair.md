# Iteration 16: Validation Repair

*2026-05-11T11:06:06Z by Showboat 0.6.1*
<!-- showboat-id: d85b1ae0-8084-4f39-ac35-0085af46a54e -->

Fixed two final review findings: scenario grammar now accepts asterisk bullets, and task_quality no longer duplicates the base missing-status diagnostic.

```bash
-c
```

```output
bash: -c: option requires an argument
```

```bash
cargo test -p ito-core --test validate_delta_rules scenario_grammar_rule && cargo test -p ito-core --test validate_tracking_rules task_quality_rule
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.20s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 5 tests
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test scenario_grammar_rule_accepts_asterisk_bullets ... ok
test scenario_grammar_rule_accepts_steps_without_bullets ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test scenario_grammar_rule_warns_on_ui_mechanics_only_for_ui_tags ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.02s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.17s
     Running tests/validate_tracking_rules.rs (target/debug/deps/validate_tracking_rules-5c752bbcd3cd2621)

running 7 tests
test task_quality_rule_emits_single_rule_error_when_tracking_file_is_unreadable ... ok
test task_quality_rule_respects_warning_floor_without_promoting_advisories ... ok
test task_quality_rule_errors_on_missing_status ... ok
test task_quality_rule_errors_on_unknown_requirement_ids ... ok
test task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify ... ok
test task_quality_rule_treats_gradle_files_as_implementation_work ... ok
test task_quality_rule_enforces_done_when_and_verify_for_impl_tasks ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

```

```bash
ito validate --changes 001-33_enhance-spec-driven-workflow-validation
```

```output
All items valid (14 checked)
```
