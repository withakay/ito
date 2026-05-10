# Task 2.2: Proposal Capability Consistency Rule

*2026-04-25T22:11:33Z by Showboat 0.6.1*
<!-- showboat-id: 3a8ad391-7c7c-4d8f-ac93-9cffec00b174 -->

Added proposal capability parsing backed by ChangeRepository proposal content and exact delta/baseline directory comparisons for listed, unlisted, and new-vs-modified mismatches.

```bash
cargo test --manifest-path Cargo.toml -p ito-core --test validate capabilities_consistency_rule
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running tests/validate.rs (target/debug/deps/validate-515280c011262d04)

running 4 tests
test capabilities_consistency_rule_errors_for_listed_capability_without_delta ... ok
test capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets ... ok
test capabilities_consistency_rule_errors_for_unlisted_delta_capability ... ok
test capabilities_consistency_rule_checks_new_vs_modified_against_baseline ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s

```

```bash
rg -n 'capabilities_consistency|read_change_proposal_markdown|Delta capability|listed as new|inline-code token' ito-rs/crates/ito-core/src/validate/mod.rs ito-rs/crates/ito-core/tests/validate.rs
```

```output
ito-rs/crates/ito-core/tests/validate.rs:1112:fn capabilities_consistency_rule_errors_for_listed_capability_without_delta() {
ito-rs/crates/ito-core/tests/validate.rs:1140:    capabilities_consistency: error
ito-rs/crates/ito-core/tests/validate.rs:1162:        issue.rule_id.as_deref() == Some("capabilities_consistency")
ito-rs/crates/ito-core/tests/validate.rs:1170:fn capabilities_consistency_rule_errors_for_unlisted_delta_capability() {
ito-rs/crates/ito-core/tests/validate.rs:1198:    capabilities_consistency: error
ito-rs/crates/ito-core/tests/validate.rs:1237:        issue.rule_id.as_deref() == Some("capabilities_consistency")
ito-rs/crates/ito-core/tests/validate.rs:1245:fn capabilities_consistency_rule_checks_new_vs_modified_against_baseline() {
ito-rs/crates/ito-core/tests/validate.rs:1273:    capabilities_consistency: error
ito-rs/crates/ito-core/tests/validate.rs:1337:        issue.rule_id.as_deref() == Some("capabilities_consistency")
ito-rs/crates/ito-core/tests/validate.rs:1338:            && issue.message.contains("listed as new")
ito-rs/crates/ito-core/tests/validate.rs:1342:        issue.rule_id.as_deref() == Some("capabilities_consistency")
ito-rs/crates/ito-core/tests/validate.rs:1349:fn capabilities_consistency_rule_skips_placeholders_and_warns_on_plain_bullets() {
ito-rs/crates/ito-core/tests/validate.rs:1377:    capabilities_consistency: error
ito-rs/crates/ito-core/tests/validate.rs:1401:        issue.rule_id.as_deref() == Some("capabilities_consistency")
ito-rs/crates/ito-core/tests/validate.rs:1403:            && issue.message.contains("inline-code token")
ito-rs/crates/ito-core/tests/validate.rs:1407:            issue.rule_id.as_deref() == Some("capabilities_consistency")
ito-rs/crates/ito-core/src/validate/mod.rs:23:    read_change_proposal_markdown,
ito-rs/crates/ito-core/src/validate/mod.rs:59:const DELTA_SPECS_PROPOSAL_RULES: &[&str] = &["capabilities_consistency"];
ito-rs/crates/ito-core/src/validate/mod.rs:765:        "capabilities_consistency" => rep.extend(validate_capabilities_consistency_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:931:fn validate_capabilities_consistency_rule(
ito-rs/crates/ito-core/src/validate/mod.rs:937:    let Some(proposal) = read_change_proposal_markdown(change_repo, change_id)? else {
ito-rs/crates/ito-core/src/validate/mod.rs:951:            "capabilities_consistency",
ito-rs/crates/ito-core/src/validate/mod.rs:962:                "capabilities_consistency",
ito-rs/crates/ito-core/src/validate/mod.rs:973:                "capabilities_consistency",
ito-rs/crates/ito-core/src/validate/mod.rs:977:                    "Capability '{capability}' is listed as new but already exists in .ito/specs/{capability}/"
ito-rs/crates/ito-core/src/validate/mod.rs:987:                "capabilities_consistency",
ito-rs/crates/ito-core/src/validate/mod.rs:998:                "capabilities_consistency",
ito-rs/crates/ito-core/src/validate/mod.rs:1020:            "capabilities_consistency",
ito-rs/crates/ito-core/src/validate/mod.rs:1023:            format!("Delta capability '{capability}' is not listed in the proposal"),
ito-rs/crates/ito-core/src/validate/mod.rs:1087:                "Capability bullet is missing an inline-code token: {rest}"
```
