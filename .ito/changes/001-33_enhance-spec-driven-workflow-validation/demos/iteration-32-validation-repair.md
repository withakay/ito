# Iteration 32: Validation Repair

*2026-05-11T14:59:04Z by Showboat 0.6.1*
<!-- showboat-id: c6f11569-0492-480d-ae21-de0272bb96b7 -->

Repaired two final validation gaps: requirement-scoped Rules / Invariants and State Transitions are parsed structurally, and invalid Change Shape values warn through the existing opt-in capabilities_consistency proposal rule.

```bash
cargo test --manifest-path ito-rs/Cargo.toml -p ito-core --test show parse_change_show_json_preserves_requirement_scoped_rules_and_state_transitions && cargo test --manifest-path ito-rs/Cargo.toml -p ito-core --test validate_delta_rules capabilities_consistency_rule_warns_on_invalid_change_shape_values
```

```output
error: manifest path `ito-rs/Cargo.toml` does not exist
```

```bash
cargo test -p ito-core --test show parse_change_show_json_preserves_requirement_scoped_rules_and_state_transitions && cargo test -p ito-core --test validate_delta_rules capabilities_consistency_rule_warns_on_invalid_change_shape_values
```

```output
   Compiling ito-core v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-core)
    Finished `test` profile [optimized + debuginfo] target(s) in 5.00s
     Running tests/show.rs (target/debug/deps/show-2dc99ecb9e0315f2)

running 1 test
test parse_change_show_json_preserves_requirement_scoped_rules_and_state_transitions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s

   Compiling ito-core v0.1.28 (/Users/jack/Code/withakay/ito/ito-worktrees/001-33_enhance-spec-driven-workflow-validation/ito-rs/crates/ito-core)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.93s
     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-5a109e2ad4e3a04d)

running 1 test
test capabilities_consistency_rule_warns_on_invalid_change_shape_values ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

```
