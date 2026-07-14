# Iteration 25: UI Mechanics Rule Repair

*2026-05-11T13:08:56Z by Showboat 0.6.1*
<!-- showboat-id: 7a4ec893-e5bd-4532-93b0-fb3707710b01 -->

Restored ui_mechanics as a separate opt-in artifact rule while preserving explicit ui-tag gating and inline-code-span suppression.

```bash
cargo test -p ito-core --test validate_delta_rules ui_mechanics_rule -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.18s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 2 tests
test ui_mechanics_rule_keeps_advisories_as_warnings_when_configured_error ... ok
test ui_mechanics_rule_warns_only_for_ui_tags ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

```

```bash
ito validate --changes 001-33_enhance-spec-driven-workflow-validation
```

```output
All items valid (14 checked)
```
