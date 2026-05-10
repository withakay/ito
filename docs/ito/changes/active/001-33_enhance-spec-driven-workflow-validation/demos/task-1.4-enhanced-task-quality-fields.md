# Task 1.4: Enhanced Task Quality Field Coverage

*2026-04-25T22:01:28Z by Showboat 0.6.1*
<!-- showboat-id: 847d0c2e-cb34-4587-bb77-950f1741610e -->

The enhanced task parser already carried the quality-critical fields, so this task locked that behavior in with focused integration coverage rather than changing the parser shape.

```bash
cargo test --manifest-path Cargo.toml -p ito-domain quality_fields
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/ito_domain-533a8c60e6864ae2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 119 filtered out; finished in 0.00s

     Running tests/planning.rs (target/debug/deps/planning-23a842f04a348790)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/schema_roundtrip.rs (target/debug/deps/schema_roundtrip-fc99c7052abc1ac5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/schema_validation.rs (target/debug/deps/schema_validation-802103c2e2481ad7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

     Running tests/tasks.rs (target/debug/deps/tasks-f9bdce8b9e6a1568)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/tasks_parsing.rs (target/debug/deps/tasks_parsing-73b0fd0501743fa6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.00s

     Running tests/tasks_parsing_additional.rs (target/debug/deps/tasks_parsing_additional-125884c2519521f1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s

     Running tests/tasks_quality_fields.rs (target/debug/deps/tasks_quality_fields-557c0c1137ffe352)

running 2 tests
test quality_fields_allow_missing_optional_metadata ... ok
test quality_fields_round_trip_when_present ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tasks_update.rs (target/debug/deps/tasks_update-d73afe788d75cc39)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.00s

     Running tests/traceability.rs (target/debug/deps/traceability-edd5c3b7af26b616)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

```

```bash
rg -n 'quality_fields|Files|Dependencies|Verify|Done When|Requirements|Updated At|Status' ito-rs/crates/ito-domain/tests/tasks_quality_fields.rs
```

```output
4:fn quality_fields_round_trip_when_present() {
10:- **Verify**: `cargo test -p ito-domain quality_fields`
11:- **Done When**: Parser baseline is ready.
12:- **Updated At**: 2026-04-25
13:- **Status**: [x] complete
16:- **Verify**: `cargo test -p ito-domain quality_fields`
17:- **Done When**: Downstream validation has input coverage.
18:- **Updated At**: 2026-04-25
19:- **Status**: [ ] pending
22:- **Files**: `src/lib.rs, Cargo.toml`
23:- **Dependencies**: Task 1.0, Task 1.2
27:- **Verify**: `cargo test -p ito-domain tasks::enhanced::quality_fields`
28:- **Done When**: All quality fields are available to validators.
29:- **Requirements**: tasks-tracking:quality-critical-fields, tasks-tracking:concrete-verification
30:- **Updated At**: 2026-04-25
31:- **Status**: [>] in-progress
48:        Some("cargo test -p ito-domain tasks::enhanced::quality_fields")
61:    assert_eq!(task.status, tasks::TaskStatus::InProgress);
66:fn quality_fields_allow_missing_optional_metadata() {
72:- **Verify**: `cargo test -p ito-domain tasks::enhanced::quality_fields`
73:- **Done When**: Parsing succeeds without optional metadata.
74:- **Updated At**: 2026-04-25
75:- **Status**: [ ] pending
88:        Some("cargo test -p ito-domain tasks::enhanced::quality_fields")
95:    assert_eq!(task.status, tasks::TaskStatus::Pending);
```
