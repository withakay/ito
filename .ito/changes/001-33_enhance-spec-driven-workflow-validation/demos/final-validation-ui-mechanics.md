# Final Validation: UI Mechanics Rule Split

*2026-05-11T13:29:56Z by Showboat 0.6.1*
<!-- showboat-id: ad556862-d3ab-44f2-a9aa-efbecff74dc1 -->

Verified the opt-in ui_mechanics rule split, strict change validation, and repository quality gates for change 001-33_enhance-spec-driven-workflow-validation.

```bash
cargo test -p ito-core --test validate_delta_rules
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.21s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 15 tests
test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
test scenario_grammar_rule_warns_on_excessive_step_count ... ok
test ui_mechanics_rule_warns_only_for_ui_tags ... ok
test scenario_grammar_rule_reports_missing_when_then_and_given ... ok
test scenario_grammar_rule_accepts_steps_without_bullets ... ok
test contract_refs_rule_warns_when_public_contract_has_no_requirement_anchor ... ok
test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
test contract_refs_rule_rejects_lowercase_unknown_scheme_after_known_ref ... ok
test ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error ... ok
test contract_refs_rule_rejects_unknown_scheme_after_known_ref ... ok
test scenario_grammar_rule_accepts_asterisk_bullets ... ok
test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
test contract_refs_rule_rejects_unknown_schemes ... ok
test contract_refs_rule_accepts_known_schemes_and_emits_single_advisory ... ok
test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

```

```bash
ito validate 001-33_enhance-spec-driven-workflow-validation --strict
```

```output
Change '001-33_enhance-spec-driven-workflow-validation' is valid
```
