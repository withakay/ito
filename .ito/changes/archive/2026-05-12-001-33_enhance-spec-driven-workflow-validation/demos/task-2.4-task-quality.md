# Task 2.4: Task Quality Validation Rule

*2026-04-25T22:21:12Z by Showboat 0.6.1*
<!-- showboat-id: 64bf04a0-635c-4771-8027-a057ccd6b7bd -->

Implemented the enhanced task quality severity table using parsed task fields, parser diagnostics for missing status, implementation-file detection, vague verify matching, and change-local requirement ID checks.

```bash
cargo test --manifest-path Cargo.toml -p ito-core --test validate task_quality_rule
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.15s
     Running tests/validate.rs (target/debug/deps/validate-515280c011262d04)

running 4 tests
test task_quality_rule_enforces_done_when_and_verify_for_impl_tasks ... ok
test task_quality_rule_errors_on_unknown_requirement_ids ... ok
test task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify ... ok
test task_quality_rule_errors_on_missing_status ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out; finished in 0.01s

```

```bash
rg -n 'task_quality|Missing Status|Vague Verify|unknown requirement ID|IMPLEMENTATION_FILE_RE' ito-rs/crates/ito-core/src/validate/mod.rs ito-rs/crates/ito-core/tests/validate.rs
```

```output
ito-rs/crates/ito-core/tests/validate.rs:1657:fn task_quality_rule_errors_on_missing_status() {
ito-rs/crates/ito-core/tests/validate.rs:1700:    task_quality: error
ito-rs/crates/ito-core/tests/validate.rs:1737:- **Verify**: `cargo test -p ito-core --test validate task_quality_rule`
ito-rs/crates/ito-core/tests/validate.rs:1748:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:1750:            && issue.message.contains("Missing Status")
ito-rs/crates/ito-core/tests/validate.rs:1755:fn task_quality_rule_enforces_done_when_and_verify_for_impl_tasks() {
ito-rs/crates/ito-core/tests/validate.rs:1798:    task_quality: error
ito-rs/crates/ito-core/tests/validate.rs:1842:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:1847:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:1852:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:1859:fn task_quality_rule_warns_for_vague_verify_missing_files_and_non_impl_verify() {
ito-rs/crates/ito-core/tests/validate.rs:1902:    task_quality: error
ito-rs/crates/ito-core/tests/validate.rs:1956:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:1958:            && issue.message.contains("Vague Verify")
ito-rs/crates/ito-core/tests/validate.rs:1961:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:1966:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:1971:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:1979:fn task_quality_rule_errors_on_unknown_requirement_ids() {
ito-rs/crates/ito-core/tests/validate.rs:2022:    task_quality: error
ito-rs/crates/ito-core/tests/validate.rs:2059:- **Verify**: `cargo test -p ito-core --test validate task_quality_rule`
ito-rs/crates/ito-core/tests/validate.rs:2071:        issue.rule_id.as_deref() == Some("task_quality")
ito-rs/crates/ito-core/tests/validate.rs:2073:            && issue.message.contains("unknown requirement ID 'auth:missing'")
ito-rs/crates/ito-core/src/validate/mod.rs:60:const TASKS_TRACKING_RULES: &[&str] = &["task_quality"];
ito-rs/crates/ito-core/src/validate/mod.rs:78:static IMPLEMENTATION_FILE_RE: LazyLock<Regex> = LazyLock::new(|| {
ito-rs/crates/ito-core/src/validate/mod.rs:795:        "task_quality" => rep.extend(validate_task_quality_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:1249:fn validate_task_quality_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:1282:                "task_quality",
ito-rs/crates/ito-core/src/validate/mod.rs:1285:                format!("Missing Status for task '{}'", task.id),
ito-rs/crates/ito-core/src/validate/mod.rs:1297:                "task_quality",
ito-rs/crates/ito-core/src/validate/mod.rs:1306:                "task_quality",
ito-rs/crates/ito-core/src/validate/mod.rs:1315:                "task_quality",
ito-rs/crates/ito-core/src/validate/mod.rs:1322:        let implementation_task = task.files.iter().any(|file| IMPLEMENTATION_FILE_RE.is_match(file));
ito-rs/crates/ito-core/src/validate/mod.rs:1332:                "task_quality",
ito-rs/crates/ito-core/src/validate/mod.rs:1340:                "task_quality",
ito-rs/crates/ito-core/src/validate/mod.rs:1343:                format!("Task '{}' has a Vague Verify value '{}'", task.id, verify),
ito-rs/crates/ito-core/src/validate/mod.rs:1353:                "task_quality",
ito-rs/crates/ito-core/src/validate/mod.rs:1356:                format!("Task '{}' references unknown requirement ID '{}'", task.id, requirement_id),
ito-rs/crates/ito-core/src/validate/mod.rs:1673:                                "Task '{}' references unknown requirement ID '{}'",
```
