# Wave 1: validate_repo Engine Scaffold

*2026-04-29T12:28:16Z by Showboat 0.6.1*
<!-- showboat-id: 6e5f08d6-b67d-4205-8f17-e2954bc16974 -->

Wave 1 (tasks 1.1-1.3) introduces the validate_repo module skeleton in ito-core: Rule trait, RuleId/Severity/Context, RuleRegistry with list_active_rules introspection, and StagedFiles snapshot reader. Built-in registry is empty until Wave 2 wires concrete rules. Engine handles infallible-by-construction registry execution with rule-level error surfacing.

```bash
ls ito-rs/crates/ito-core/src/validate_repo/
```

```output
mod.rs
registry.rs
rule.rs
staged.rs
```

```bash
cargo test -p ito-core --lib validate_repo 2>&1 | tail -8
```

```output
test validate_repo::staged::tests::from_z_separated_handles_only_nuls ... ok
test validate_repo::staged::tests::from_z_separated_handles_trailing_nul ... ok
test validate_repo::tests::run_repo_validation_strict_with_empty_registry_still_empty ... ok
test validate_repo::tests::run_repo_validation_with_empty_registry_returns_empty_report ... ok
test worktree_validate::tests::worktree_validate_reports_mismatch_outside_main_checkout ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 600 filtered out; finished in 0.00s

```
