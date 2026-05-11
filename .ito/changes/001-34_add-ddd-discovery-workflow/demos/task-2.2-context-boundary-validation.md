# Task 2.2: Context Boundary Validation

*2026-05-11T19:59:28Z by Showboat 0.6.1*
<!-- showboat-id: f83bba49-cf2b-4404-9d6b-f2118432ead7 -->

Added the opt-in context_boundary_consistency proposal rule. It reads domain-discovery.md and warns when cross-context discovery lacks affected context ownership, relationship framing, or translation boundary details.

```bash
cargo test -p ito-core --test validate_delta_rules context_boundary_consistency_rule
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.16s
     Running tests/validate_delta_rules.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/target/debug/deps/validate_delta_rules-01440800bf6c477a)

running 3 tests
test context_boundary_consistency_rule_passes_for_explicit_relationship_and_translation ... ok
test context_boundary_consistency_rule_warns_for_incomplete_cross_context_framing ... ok
test context_boundary_consistency_rule_is_silent_for_single_context_discovery ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

```
