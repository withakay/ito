# Validation parity fix

*2026-05-11T08:24:37Z by Showboat 0.6.1*
<!-- showboat-id: 3e2a6638-3e7a-419b-bacc-f42f43bd51cb -->

Removed duplicate bulk-path tasks validation so schema-driven validate_change remains the single source for tracking diagnostics, and added a regression test comparing single-change and bulk validation output.

```bash
RUSTFLAGS='-D warnings' cargo test -p ito-cli --test validate_more validate_change_and_bulk_do_not_duplicate_schema_tracking_issues
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.74s
     Running tests/validate_more.rs (target/debug/deps/validate_more-983cdb7b6c2434ca)

running 1 test
test validate_change_and_bulk_do_not_duplicate_schema_tracking_issues ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.62s

```

```bash
ito validate --changes 001-33_enhance-spec-driven-workflow-validation
```

```output
All items valid (14 checked)
```
