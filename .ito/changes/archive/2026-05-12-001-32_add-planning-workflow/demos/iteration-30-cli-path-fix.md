# Iteration 30: Planning CLI Path Fix

*2026-05-11T03:49:26Z by Showboat 0.6.1*
<!-- showboat-id: a9f7f3cb-2592-458e-afda-5907086a5abb -->

Updated ito plan init to print the resolved planning and research workspace paths instead of deriving a display label from the final Ito directory component.

```bash
cargo test -p ito-cli --test plan_state_more plan_init_prints_configured_workspace_paths -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.25s
     Running tests/plan_state_more.rs (target/debug/deps/plan_state_more-f36178f6b4aed8bf)

running 1 test
test plan_init_prints_configured_workspace_paths ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.55s

```

```bash
ito validate 001-32_add-planning-workflow --strict
```

```output
Change '001-32_add-planning-workflow' is valid
```
