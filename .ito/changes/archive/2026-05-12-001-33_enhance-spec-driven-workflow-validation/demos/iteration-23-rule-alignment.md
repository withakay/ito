# Iteration 23: Scenario Grammar Rule Alignment

*2026-05-11T12:30:48Z by Showboat 0.6.1*
<!-- showboat-id: 0675745c-866b-4e9a-a297-c527c9439265 -->

Folded UI-mechanics advisory checks into the scenario_grammar rule, removed the separate ui_mechanics rule surface, ignored inline code spans during UI-mechanics matching, and allowed delta-spec rules on any artifact id using ito.delta-specs.v1.

```bash
cargo test -p ito-core --test validate_delta_rules scenario_grammar_rule && cargo test -p ito-core --test validate_rules_extension
```

```output
    Blocking waiting for file lock on artifact directory
    Finished `test` profile [optimized + debuginfo] target(s) in 1.85s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 6 tests
test scenario_grammar_rule_accepts_asterisk_bullets ... ok
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test scenario_grammar_rule_accepts_steps_without_bullets ... ok
test scenario_grammar_rule_keeps_ui_mechanics_advisories_as_warnings_when_configured_error ... ok
test scenario_grammar_rule_warns_on_ui_mechanics_only_for_ui_tags ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.03s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/validate_rules_extension.rs (target/debug/deps/validate_rules_extension-e9951ba33c4f865e)

running 3 tests
test validation_yaml_proposal_entry_dispatches_rule_configuration ... ok
test validation_yaml_rules_extension_warns_for_unknown_rule_names ... ok
test validation_yaml_delta_rules_work_for_non_specs_artifact_ids ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```
