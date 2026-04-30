# Task 1.3: Agent Template Activation Contract

*2026-04-29T18:20:24Z by Showboat 0.6.1*
<!-- showboat-id: bfd591c5-607b-4db8-836a-87282f46f5ee -->

Added explicit activation metadata to generated Ito agent templates so direct entrypoints declare activation: direct while delegated role agents declare activation: delegated.

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-templates agent_templates_declare_activation_contract -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.18s
     Running unittests src/lib.rs (target/debug/deps/ito_templates-7eaa5889a2394c40)

running 1 test
test agent_surface_tests::agent_templates_declare_activation_contract ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 88 filtered out; finished in 0.00s

     Running tests/instructions_apply_memory.rs (target/debug/deps/instructions_apply_memory-d1e1807ce87f211a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/managed_markers.rs (target/debug/deps/managed_markers-7a5705c6aff70672)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/prefix_rule.rs (target/debug/deps/prefix_rule-b0565b06adac7694)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stamp.rs (target/debug/deps/stamp-95514587e0df9f18)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (target/debug/deps/template_markdown-bc4ea5e74b0d0fe5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (target/debug/deps/user_guidance_template-36770e8c31892375)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (target/debug/deps/worktree_template_rendering-825fa89e3cbc9b79)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

```

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-cli --test init_agent_activation -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.30s
     Running tests/init_agent_activation.rs (target/debug/deps/init_agent_activation-e20dd9c42c9d9078)

running 1 test
test init_update_with_tools_all_preserves_agent_activation_contract ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.53s

```

```bash
make check-max-lines
```

```output
python3 "ito-rs/tools/check_max_lines.py" --max-lines "1000" --root "ito-rs" --baseline "ito-rs/tools/max_lines_baseline.txt"
Warning: 8 Rust files exceed limits but remain within baseline:
  - ito-rs/crates/ito-core/src/ralph/runner.rs: 1426 (baseline 1426)
  - ito-rs/crates/ito-cli/tests/ralph_smoke.rs: 1408 (baseline 1408)
  - ito-rs/crates/ito-core/src/installers/mod.rs: 1376 (baseline 1380)
  - ito-rs/crates/ito-config/src/config/types.rs: 1371 (baseline 1371)
  - ito-rs/crates/ito-cli/tests/init_more.rs: 1311 (baseline 1336)
  - ito-rs/crates/ito-core/src/coordination_worktree.rs: 1283 (baseline 1283)
  - ito-rs/crates/ito-core/tests/ralph.rs: 1279 (baseline 1279)
  - ito-rs/crates/ito-templates/src/instructions_tests.rs: 1235 (baseline 1414)
Warning: 14 Rust files over soft limit (1000 lines):
  - ito-rs/crates/ito-cli/src/app/instructions.rs: 1199 (consider splitting)
  - ito-rs/crates/ito-cli/src/cli.rs: 1199 (consider splitting)
  - ito-rs/crates/ito-templates/src/lib.rs: 1170 (consider splitting)
  - ito-rs/crates/ito-core/src/create/mod.rs: 1131 (consider splitting)
  - ito-rs/crates/ito-core/src/validate/mod.rs: 1129 (consider splitting)
  - ito-rs/crates/ito-domain/src/tasks/parse.rs: 1097 (consider splitting)
  - ito-rs/crates/ito-core/src/config.rs: 1077 (consider splitting)
  - ito-rs/crates/ito-core/src/tasks.rs: 1075 (consider splitting)
  - ito-rs/crates/ito-cli/src/commands/tasks.rs: 1061 (consider splitting)
  - ito-rs/crates/ito-core/src/coordination_worktree_tests.rs: 1039 (consider splitting)
  - ito-rs/crates/ito-core/src/backend_http.rs: 1025 (consider splitting)
  - ito-rs/crates/ito-core/src/templates/mod.rs: 1015 (consider splitting)
  - ito-rs/crates/ito-core/tests/validate.rs: 1010 (consider splitting)
  - ito-rs/crates/ito-core/src/audit/mirror.rs: 1003 (consider splitting)
```

Review found an update-path gap, so existing agent frontmatter now receives activation metadata from the rendered template and OpenCode delegated agents are verified to keep mode: subagent.

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-core installers::agent_frontmatter -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.49s
     Running unittests src/lib.rs (target/debug/deps/ito_core-2c0501004319aa04)

running 3 tests
test installers::agent_frontmatter::tests::update_yaml_field_replaces_or_inserts ... ok
test installers::agent_frontmatter::tests::activation_field_is_copied_from_rendered_template ... ok
test installers::agent_frontmatter::tests::update_agent_model_field_updates_frontmatter_when_present ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 599 filtered out; finished in 0.00s

     Running tests/archive.rs (target/debug/deps/archive-1e441e8f2599fd3d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target/debug/deps/audit_mirror-591d1eab1ac17556)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (target/debug/deps/audit_storage-74b1669dd273dc32)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target/debug/deps/backend_archive-0aa5f84517587eac)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (target/debug/deps/backend_auth-16e90dd520389c08)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target/debug/deps/backend_auth_service-1b852594cb27a447)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target/debug/deps/backend_client_mode-ae08737de38c0dc3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target/debug/deps/backend_module_repository-2263ca4bc4e714a8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target/debug/deps/backend_sub_module_support-56350f4b268d6810)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target/debug/deps/change_repository_lifecycle-078bc5da9d44cea0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (target/debug/deps/change_repository_orchestrate_metadata-bf6a6e8e2b0bf8fb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target/debug/deps/change_repository_parity-813ba746e3ec2d0e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (target/debug/deps/change_target_resolution_parity-49fe3128624782cb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/coordination_worktree.rs (target/debug/deps/coordination_worktree-55cba6c7719c5a20)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/create.rs (target/debug/deps/create-c5e723a8111a8f02)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/distribution.rs (target/debug/deps/distribution-35c416baf8598c0d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (target/debug/deps/event_forwarding-4b7474e7578f6cc3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (target/debug/deps/grep_scopes-08743232d114310f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target/debug/deps/harness_context-2858105d86e7712e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (target/debug/deps/harness_opencode-8fb9191c06ae6b86)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (target/debug/deps/harness_streaming-d017be767dd483e8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (target/debug/deps/harness_stub-4b6033dfc270acc1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (target/debug/deps/import-cf2cf6e922e62d26)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (target/debug/deps/io-57a32d4a33a5c750)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target/debug/deps/orchestrate_run_state-df8dd2e341748585)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target/debug/deps/planning_init-bea70fbe91dfbe19)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph.rs (target/debug/deps/ralph-4d13ca2184206900)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s

     Running tests/repo_index.rs (target/debug/deps/repo_index-f2a40a2feaa53be3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target/debug/deps/repo_integrity-5fb282e2c0954442)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target/debug/deps/repo_paths-1cf0c23e35fb53cd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (target/debug/deps/repository_runtime-8d5da2a976d7b2f0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (target/debug/deps/repository_runtime_config_validation-056dfd56217cf6bb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (target/debug/deps/show-35a3d9f0d0cba11b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target/debug/deps/spec_repository_backends-14e65931066406ed)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target/debug/deps/spec_show_repository-0dbb9311a789e963)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target/debug/deps/sqlite_archive_mirror-4dcbe8af9e93d25a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target/debug/deps/sqlite_task_mutations-6fa3f601f0420373)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-fdd65ea6b27f1871)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target/debug/deps/task_repository_summary-bea4ca4b42bc8d53)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target/debug/deps/tasks_api-99836ad3bbb88984)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (target/debug/deps/tasks_checkbox_format-f15f136a82ed3479)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target/debug/deps/tasks_orchestration-e62428fe6fec2c99)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-62f16cea2c1fd28e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-124bdabc9a3b5538)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target/debug/deps/templates_review_context-8ceff7bfe5f90877)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-403d2934dca93895)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-6a8963b69d82d7d2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target/debug/deps/templates_user_guidance-c04a0b9ff309699b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target/debug/deps/traceability_e2e-97b1a17c0541bf62)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/validate.rs (target/debug/deps/validate-d020b20e4aeab49f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s

     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-01440800bf6c477a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/validate_rules_extension.rs (target/debug/deps/validate_rules_extension-e75d7555338dea96)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (target/debug/deps/validate_tracking_rules-41350eeb32244295)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/worktree_ensure_e2e.rs (target/debug/deps/worktree_ensure_e2e-8e817f71bae0c07d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

```

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p ito-cli --test init_agent_activation -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.33s
     Running tests/init_agent_activation.rs (target/debug/deps/init_agent_activation-e20dd9c42c9d9078)

running 2 tests
test init_update_adds_activation_to_existing_agent_frontmatter ... ok
test init_update_with_tools_all_preserves_agent_activation_contract ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.62s

```

Full repository verification passed after fixing markdownlint issues surfaced by make check.

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools make check
```

```output
check for added large files..............................................Passed
check for merge conflicts................................................Passed
check toml...............................................................Passed
check yaml...............................................................Passed
check json...............................................................Passed
fix end of files.........................................................Passed
mixed line ending........................................................Passed
trim trailing whitespace.................................................Passed
pretty format json.......................................................Passed
yamllint.................................................................Passed
markdownlint-cli2........................................................Passed
cargo fmt (ito-rs).......................................................Passed
forbid local version metadata in Cargo.toml..............................Passed
cargo clippy (ito-rs)....................................................Passed
cargo doc warnings as errors (ito-rs)....................................Passed
cargo test with coverage (ito-rs)........................................Passed
cargo test affected (ito-rs).............................................Passed
check max lines (ito-rs).................................................Passed
architecture guardrails..................................................Passed
cargo deny (license/advisory checks).....................................Passed
```
