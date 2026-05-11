# 001-32 Planning Workflow Final Validation

*2026-05-11T01:46:07Z by Showboat 0.6.1*
<!-- showboat-id: a9260947-2e45-4424-a1c9-b4685f0f96a6 -->

Validated the dedicated planning workflow change after implementation: Ito strict validation, targeted planning tests, template tests, domain planning tests, audit reconciliation, and full project checks all pass.

```bash
ito validate 001-32_add-planning-workflow --strict
```

```output
Change '001-32_add-planning-workflow' is valid
```

```bash
cargo test -p ito-cli plan -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.30s
     Running unittests src/main.rs (target/debug/deps/ito-433b715f15ce9eb4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 88 filtered out; finished in 0.00s

     Running tests/agent_instruction_apply_sync.rs (target/debug/deps/agent_instruction_apply_sync-b73cf58d6121c110)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_bootstrap.rs (target/debug/deps/agent_instruction_bootstrap-d71231d190cecc4b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_context.rs (target/debug/deps/agent_instruction_context-04fa9d6916e87a15)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_memory.rs (target/debug/deps/agent_instruction_memory-cbb071b369f0474a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/agent_instruction_orchestrate.rs (target/debug/deps/agent_instruction_orchestrate-1956cf17c511de95)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_repo_sweep.rs (target/debug/deps/agent_instruction_repo_sweep-dc28da3551354307)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/agent_instruction_worktrees.rs (target/debug/deps/agent_instruction_worktrees-6d330a49bcef9234)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/aliases.rs (target/debug/deps/aliases-14c70cee4c6466d5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/archive_completed.rs (target/debug/deps/archive_completed-fa98a609d7bc0d5e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/archive_remote_mode.rs (target/debug/deps/archive_remote_mode-be26b0deff4446f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/archive_smoke.rs (target/debug/deps/archive_smoke-5e64fd9ee7cac95d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/artifact_mutations.rs (target/debug/deps/artifact_mutations-8ce977d44ca78c9b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_more.rs (target/debug/deps/audit_more-858c92ec94ea081d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_remote_mode.rs (target/debug/deps/audit_remote_mode-f27c11b794aa154e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_import.rs (target/debug/deps/backend_import-9d35ba2e2b9c9d97)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/backend_qa_walkthrough.rs (target/debug/deps/backend_qa_walkthrough-e33ebf3d6e7c6313)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_serve.rs (target/debug/deps/backend_serve-d5be3d02b79ac461)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_status_more.rs (target/debug/deps/backend_status_more-2175f6936ee3e8a6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 20 filtered out; finished in 0.00s

     Running tests/cli_smoke.rs (target/debug/deps/cli_smoke-8715f50eeeafa9b0)

running 1 test
test create_workflow_plan_state_config_smoke ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 1.36s

     Running tests/cli_snapshots.rs (target/debug/deps/cli_snapshots-a9961412d138cb5a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/config_more.rs (target/debug/deps/config_more-f147277a45d12a55)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/coverage_smoke.rs (target/debug/deps/coverage_smoke-e50030a47e559f69)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/create_more.rs (target/debug/deps/create_more-97c8984ad2bd69b1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/grep_more.rs (target/debug/deps/grep_more-b1f8bdd2394032b4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/help.rs (target/debug/deps/help-7edcee4d2ab25d1d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/init_agent_activation.rs (target/debug/deps/init_agent_activation-2a11eeaf640dd1d6)

running 1 test
test init_update_installs_ito_plan_command_and_skill_for_all_harnesses ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.12s

     Running tests/init_coordination.rs (target/debug/deps/init_coordination-47531c7121dbd10d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/init_gitignore_session_json.rs (target/debug/deps/init_gitignore_session_json-e5cca16bb1e4c011)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/init_more.rs (target/debug/deps/init_more-564d0e1709573c89)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.00s

     Running tests/init_obsolete_cleanup.rs (target/debug/deps/init_obsolete_cleanup-aef53d2d818a6f14)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/init_tmux.rs (target/debug/deps/init_tmux-058f884714936d7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/init_upgrade_more.rs (target/debug/deps/init_upgrade_more-0d8d7ad8d4fbe3be)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/instructions_more.rs (target/debug/deps/instructions_more-57a13f0368afdeba)

running 2 tests
test agent_instruction_manifesto_planning_profile_embeds_no_mutating_artifacts ... ok
test agent_instruction_manifesto_planning_profile_is_advisory ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.11s

     Running tests/list_archive.rs (target/debug/deps/list_archive-aa095708a39e6d3d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/list_regression.rs (target/debug/deps/list_regression-1d992335e041b3a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/misc_more.rs (target/debug/deps/misc_more-7aa0bbbc6562dba7)

running 1 test
test plan_status_reports_missing_workspace ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.03s

     Running tests/new_more.rs (target/debug/deps/new_more-982202ef8adaa9bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/parity_help_version.rs (target/debug/deps/parity_help_version-92411a874d0c6e5d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/parity_tasks.rs (target/debug/deps/parity_tasks-9a5e548867f23ba8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/path_more.rs (target/debug/deps/path_more-044be90fa762924a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/plan_state_more.rs (target/debug/deps/plan_state_more-f36178f6b4aed8bf)

running 7 tests
test plan_init_reports_conflicting_planning_file ... ok
test plan_status_reports_invalid_workspace_without_init_hint_loop ... ok
test plan_status_reports_invalid_research_workspace ... ok
test plan_status_reports_missing_workspace_without_error ... ok
test plan_status_lists_markdown_documents ... ok
test plan_init_creates_structure ... ok
test plan_status_succeeds_after_init ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

     Running tests/ralph_smoke.rs (target/debug/deps/ralph_smoke-8c28a8c8bfad5b39)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/serve_more.rs (target/debug/deps/serve_more-8d1cdde98a7bffa8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show_specs_bundle.rs (target/debug/deps/show_specs_bundle-d8c98dbf360e5015)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/show_specs_remote_mode.rs (target/debug/deps/show_specs_remote_mode-c1eac5c6994391aa)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/source_file_size.rs (target/debug/deps/source_file_size-f6250e3905bf836f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-7de066585490e41f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_more.rs (target/debug/deps/tasks_more-db184e01375bf4b5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/tasks_remote_mode.rs (target/debug/deps/tasks_remote_mode-37cec52258f81d1d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_schemas_export.rs (target/debug/deps/templates_schemas_export-55dbbb0724f69cd6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/trace_more.rs (target/debug/deps/trace_more-76b2116c5b9f8002)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/update_marker_scoped.rs (target/debug/deps/update_marker_scoped-35b704848df1ce9a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/update_smoke.rs (target/debug/deps/update_smoke-6ce5e7bb13bc470a)

running 1 test
test update_installs_ito_plan_command_and_skill_for_all_harnesses ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.11s

     Running tests/user_guidance_injection.rs (target/debug/deps/user_guidance_injection-f0ab387e401f400e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/validate_more.rs (target/debug/deps/validate_more-d9c991798e7fbe93)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/validate_repo_cli.rs (target/debug/deps/validate_repo_cli-d7e6b8b4844b229d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.00s

     Running tests/view_proposal.rs (target/debug/deps/view_proposal-a70309d12012718e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/worktree_validate.rs (target/debug/deps/worktree_validate-fdc6f4a293e6901e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

```

```bash
make check
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
