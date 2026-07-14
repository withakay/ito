# Iteration 33: Capabilities Parser Repair

*2026-05-11T15:40:53Z by Showboat 0.6.1*
<!-- showboat-id: dd94f057-0f49-4fe2-8323-a9e4e7b4dde2 -->

Repaired proposal capability parsing so bullets outside New Capabilities / Modified Capabilities do not emit inline-code warnings, while preserving warnings inside supported subsections. Hardened Change Shape field slicing with safe string access.

```bash
cargo test -p ito-core --test validate_delta_rules capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.26s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 1 test
test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.01s

```

```bash
ito validate --changes 001-33_enhance-spec-driven-workflow-validation
```

```output
All items valid (14 checked)
```
