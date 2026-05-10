# Task 1.3: Validation Rules and Proposal Plumbing

*2026-04-25T21:58:08Z by Showboat 0.6.1*
<!-- showboat-id: fc320322-39e4-4dba-9f2a-2607ff132f88 -->

Extended validation.yaml parsing with per-artifact rules, proposal-level config, and a rule-aware validation issue envelope so later rule tasks can plug in without reshaping the validator API.

```bash
rg -n 'proposal:|rules:|rule_id|with_rule_id|run_configured_rules' ito-rs/crates/ito-core/src/{templates/types.rs,validate/mod.rs,validate/issue.rs}
```

```output
ito-rs/crates/ito-core/src/validate/issue.rs:35:        rule_id: None,
ito-rs/crates/ito-core/src/validate/issue.rs:90:pub fn with_rule_id(mut i: ValidationIssue, rule_id: impl Into<String>) -> ValidationIssue {
ito-rs/crates/ito-core/src/validate/issue.rs:91:    i.rule_id = Some(rule_id.into());
ito-rs/crates/ito-core/src/validate/issue.rs:114:    if let Some(rule_id) = i.rule_id.as_ref() {
ito-rs/crates/ito-core/src/validate/issue.rs:116:            "rule_id".to_string(),
ito-rs/crates/ito-core/src/validate/issue.rs:117:            serde_json::Value::String(rule_id.clone()),
ito-rs/crates/ito-core/src/validate/issue.rs:177:    fn rule_id_helper_marks_issue_and_is_reflected_in_metadata() {
ito-rs/crates/ito-core/src/validate/issue.rs:178:        let base = with_rule_id(error("spec.md", "invalid scenario"), "scenario_grammar");
ito-rs/crates/ito-core/src/validate/issue.rs:181:        assert_eq!(out.rule_id.as_deref(), Some("scenario_grammar"));
ito-rs/crates/ito-core/src/validate/issue.rs:186:            meta.get("rule_id").and_then(|value| value.as_str()),
ito-rs/crates/ito-core/src/validate/mod.rs:33:pub use issue::{error, info, issue, warning, with_line, with_loc, with_metadata, with_rule_id};
ito-rs/crates/ito-core/src/validate/mod.rs:72:    pub rule_id: Option<String>,
ito-rs/crates/ito-core/src/validate/mod.rs:428:                run_configured_rules(
ito-rs/crates/ito-core/src/validate/mod.rs:456:        run_configured_rules(
ito-rs/crates/ito-core/src/validate/mod.rs:488:                        run_configured_rules(
ito-rs/crates/ito-core/src/validate/mod.rs:561:                        run_configured_rules(
ito-rs/crates/ito-core/src/validate/mod.rs:629:fn run_configured_rules(
ito-rs/crates/ito-core/src/validate/mod.rs:635:    rules: Option<&BTreeMap<String, ValidationLevelYaml>>,
ito-rs/crates/ito-core/src/validate/mod.rs:653:                        "Unknown validation rule '{rule_name}' for {} (validator: {}). Supported rules: {supported}",
ito-rs/crates/ito-core/src/validate/mod.rs:865:                rule_id: None,
ito-rs/crates/ito-core/src/templates/types.rs:469:    pub proposal: Option<ValidationArtifactYaml>,
ito-rs/crates/ito-core/src/templates/types.rs:537:    pub rules: Option<BTreeMap<String, ValidationLevelYaml>>,
ito-rs/crates/ito-core/src/templates/types.rs:553:    pub rules: Option<BTreeMap<String, ValidationLevelYaml>>,
ito-rs/crates/ito-core/src/templates/types.rs:615:    rules:
ito-rs/crates/ito-core/src/templates/types.rs:621:  rules:
ito-rs/crates/ito-core/src/templates/types.rs:648:proposal:
ito-rs/crates/ito-core/src/templates/types.rs:650:  rules:
```

```bash
cargo test --manifest-path Cargo.toml -p ito-core --test validate validation_yaml_rules_extension && cargo test --manifest-path Cargo.toml -p ito-core --test validate validation_yaml_proposal_entry && cargo test --manifest-path Cargo.toml -p ito-core validation_yaml_parses
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running tests/validate.rs (target/debug/deps/validate-515280c011262d04)

running 1 test
test validation_yaml_rules_extension_warns_for_unknown_rule_names ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 24 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.13s
     Running tests/validate.rs (target/debug/deps/validate-515280c011262d04)

running 1 test
test validation_yaml_proposal_entry_dispatches_rule_configuration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 24 filtered out; finished in 0.00s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.14s
     Running unittests src/lib.rs (target/debug/deps/ito_core-0992febe54bed43a)

running 3 tests
test templates::types::tests::validation_yaml_parses_minimal_config ... ok
test templates::types::tests::validation_yaml_parses_rules_extension_without_breaking_existing_shape ... ok
test templates::types::tests::validation_yaml_parses_proposal_entry_with_rules ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 580 filtered out; finished in 0.00s

     Running tests/archive.rs (target/debug/deps/archive-5a9fdc6b6f808cb3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target/debug/deps/audit_mirror-0d2cf541244c5074)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (target/debug/deps/audit_storage-fcedb37f5f3f0871)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target/debug/deps/backend_archive-81b44238f38da62b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (target/debug/deps/backend_auth-b473b8d780bebec4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target/debug/deps/backend_auth_service-bc0cd4e8a206d090)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target/debug/deps/backend_client_mode-0cb02f06b6a24321)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target/debug/deps/backend_module_repository-0c4b89d7cd31ed1c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target/debug/deps/backend_sub_module_support-09e4ad5a3c4a547c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target/debug/deps/change_repository_lifecycle-7c42f915e07b55a4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (target/debug/deps/change_repository_orchestrate_metadata-8abce2cb501bfd86)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target/debug/deps/change_repository_parity-cb0d1e9e95186364)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (target/debug/deps/change_target_resolution_parity-9012dd3843607587)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/coordination_worktree.rs (target/debug/deps/coordination_worktree-10654af80535d30b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/create.rs (target/debug/deps/create-9b424bc92c54b480)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/distribution.rs (target/debug/deps/distribution-edb1390eaaa7a142)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (target/debug/deps/event_forwarding-0bffa50ce1bcfbd3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (target/debug/deps/grep_scopes-58ae52200cd2be99)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target/debug/deps/harness_context-02224f7d9eb0db2d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (target/debug/deps/harness_opencode-f75a6a2824fc3c11)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (target/debug/deps/harness_streaming-11fd59d4f152fbe0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (target/debug/deps/harness_stub-199286dca2e7019c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (target/debug/deps/import-0482815809d808e4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (target/debug/deps/io-f6325f7375fe79ce)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target/debug/deps/orchestrate_run_state-5da71d95256483c9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target/debug/deps/planning_init-1928baeecdd24438)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph.rs (target/debug/deps/ralph-aa307e1d422e97e0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s

     Running tests/repo_index.rs (target/debug/deps/repo_index-df6d88440041a44f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target/debug/deps/repo_integrity-8520684c7a7d518c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target/debug/deps/repo_paths-a04b81f96d80e304)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (target/debug/deps/repository_runtime-2f754e73011db373)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (target/debug/deps/repository_runtime_config_validation-bc32fa161758f24d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (target/debug/deps/show-0f592c293d11c9eb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target/debug/deps/spec_repository_backends-d50810908e2bb791)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target/debug/deps/spec_show_repository-055de67c9a9c58ef)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target/debug/deps/sqlite_archive_mirror-b5dda71a9a51cc13)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target/debug/deps/sqlite_task_mutations-02dd3767bb8d0dd2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-673b576cecc43465)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target/debug/deps/task_repository_summary-de776dd115f8eea1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target/debug/deps/tasks_api-9e4809996e26376b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (target/debug/deps/tasks_checkbox_format-15a807a88d37ad9f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target/debug/deps/tasks_orchestration-b10be8bb6bc30a7c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-2b760f979c8744ac)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-eda86531c2840d29)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target/debug/deps/templates_review_context-95fd2ac37d4b7c72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-9026c3592bc7b48c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-a6f579eac48c56f8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target/debug/deps/templates_user_guidance-af36dc9a89fed7e1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target/debug/deps/traceability_e2e-d149b6d39377b822)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/validate.rs (target/debug/deps/validate-515280c011262d04)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 25 filtered out; finished in 0.00s

     Running tests/worktree_ensure_e2e.rs (target/debug/deps/worktree_ensure_e2e-b58783c35e81b8c0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

```
