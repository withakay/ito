# Validation Repair Evidence

*2026-05-11T09:05:10Z by Showboat 0.6.1*
<!-- showboat-id: 08011e7e-c493-4483-8e68-177794bb95dd -->

Repaired final review blockers: scenario grammar now accepts plain GIVEN/WHEN/THEN/AND steps as well as bold Markdown steps, and Contract Refs parsing splits unknown scheme-like entries after valid refs so validation can report them.

```bash
cargo test -p ito-core --test validate_delta_rules contract_refs_rule_rejects_unknown_scheme_after_known_ref -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.18s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 1 test
test contract_refs_rule_rejects_unknown_scheme_after_known_ref ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

```

```bash
cargo test -p ito-core --test validate_delta_rules scenario_grammar_rule_accepts_steps_without_bullets -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.15s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 1 test
test scenario_grammar_rule_accepts_steps_without_bullets ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

```
