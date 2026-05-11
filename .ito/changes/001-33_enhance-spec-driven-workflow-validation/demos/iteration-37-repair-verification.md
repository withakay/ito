# Iteration 37: Validation Repair Verification

*2026-05-11T18:00:05Z by Showboat 0.6.1*
<!-- showboat-id: 8b541b79-6ae7-492b-9cb0-b0fa5c53c580 -->

Repaired three review findings: generated enhanced task templates now include Files/Verify/Done When for checkpoints, scenario grammar accepts ordered-list GIVEN/WHEN/THEN steps, and contract-ref parsing exposes short ref-like unknown schemes such as rpc:Call so validation can reject them.

```bash
cargo test -p ito-core --test validate_delta_rules scenario_grammar_rule_accepts_ordered_list_steps && cargo test -p ito-core --test validate_delta_rules contract_refs_rule_rejects_short_unknown_scheme_after_known_ref && cargo test -p ito-core --test show parse_contract_refs_splits_short_unknown_scheme_with_ref_like_identifier && cargo test -p ito-domain --test tasks enhanced_template_parses_and_has_checkpoint_warning
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.18s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 1 test
test scenario_grammar_rule_accepts_ordered_list_steps ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.15s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 1 test
test contract_refs_rule_rejects_short_unknown_scheme_after_known_ref ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.15s
     Running tests/show.rs (target/debug/deps/show-2dc99ecb9e0315f2)

running 1 test
test parse_contract_refs_splits_short_unknown_scheme_with_ref_like_identifier ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running tests/tasks.rs (target/debug/deps/tasks-5cb039ab908fe47d)

running 1 test
test enhanced_template_parses_and_has_checkpoint_warning ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s

```

```bash
ito validate 001-33_enhance-spec-driven-workflow-validation --strict && ito validate repo
```

```output
Change '001-33_enhance-spec-driven-workflow-validation' is valid
Repository validation passed.
```
