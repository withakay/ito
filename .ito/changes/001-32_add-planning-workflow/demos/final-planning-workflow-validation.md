# Final Planning Workflow Validation

*2026-05-11T00:59:28Z by Showboat 0.6.1*
<!-- showboat-id: 6671e6a1-4271-4fc3-b5d7-fcd878903014 -->

Captured final validation evidence for the planning workflow change: strict Ito validation, audit reconciliation, and targeted planning-related Rust tests.

```bash
ito validate 001-32_add-planning-workflow --strict
```

```output
Change '001-32_add-planning-workflow' is valid
```

```bash
ito audit reconcile --change 001-32_add-planning-workflow
```

```output
Reconcile: 001-32_add-planning-workflow
──────────────────────────────────────────────────
No drift detected. Audit log and files are in sync.
```

```bash
cargo test -p ito-core planning_init -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.64s
     Running unittests src/lib.rs (target/debug/deps/ito_core-9f12863f95a28325)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 713 filtered out; finished in 0.00s

     Running tests/archive.rs (target/debug/deps/archive-d15f1aee2807facb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target/debug/deps/audit_mirror-eb6d006965a9a2da)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (target/debug/deps/audit_storage-cd83bee6ca02e972)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target/debug/deps/backend_archive-ecdaff9bba9d4c5d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (target/debug/deps/backend_auth-bebfc7dd53770ee7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target/debug/deps/backend_auth_service-c5bb145b2d4c62d5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target/debug/deps/backend_client_mode-413ad25d2e735a59)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target/debug/deps/backend_module_repository-caa11ac31ea9e48e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target/debug/deps/backend_sub_module_support-8b9dd65081a983d3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target/debug/deps/change_repository_lifecycle-5dbcd93fe01d8d9f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (target/debug/deps/change_repository_orchestrate_metadata-e98de8d6d18ba974)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target/debug/deps/change_repository_parity-2a50ef7194c87471)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (target/debug/deps/change_target_resolution_parity-7e1ff7bcba35e6a9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/coordination_worktree.rs (target/debug/deps/coordination_worktree-9b4935f8a988eef9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/create.rs (target/debug/deps/create-dcf0f34fcfa8d523)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/distribution.rs (target/debug/deps/distribution-d8c4f23aeb1f4e22)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (target/debug/deps/event_forwarding-447907d9077d6198)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (target/debug/deps/grep_scopes-3f09291b073fefde)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target/debug/deps/harness_context-1d4e052c6a7c0e79)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (target/debug/deps/harness_opencode-bf927588d6b3a412)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (target/debug/deps/harness_streaming-eddb369ff8db395e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (target/debug/deps/harness_stub-ed2504f9f1ac5403)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (target/debug/deps/import-d16e15f79478ce65)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (target/debug/deps/io-70571a69da2e7562)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target/debug/deps/orchestrate_run_state-e268603db761d7be)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target/debug/deps/planning_init-a7d9eb36a801cb09)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/ralph.rs (target/debug/deps/ralph-8588061109d0dd7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s

     Running tests/repo_index.rs (target/debug/deps/repo_index-f80a9056b82bf202)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target/debug/deps/repo_integrity-d4c2eb7849e47b01)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target/debug/deps/repo_paths-5a87e9740f60ee59)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (target/debug/deps/repository_runtime-36703bb4d5f1bcd8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (target/debug/deps/repository_runtime_config_validation-3764a67077b5ee03)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (target/debug/deps/show-93ecf7a643b56077)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target/debug/deps/spec_repository_backends-7588f23c27292308)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target/debug/deps/spec_show_repository-aaf6300b29764814)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target/debug/deps/sqlite_archive_mirror-b2d5619bdb2dd15a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target/debug/deps/sqlite_task_mutations-d86fdf64b7e50347)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-bd74d3f69e569677)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target/debug/deps/task_repository_summary-7c8f0b9f20ec56ac)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target/debug/deps/tasks_api-6c0b8eac1db91289)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (target/debug/deps/tasks_checkbox_format-70bbc5308799ec78)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target/debug/deps/tasks_orchestration-2d1bff24a66083d6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-2987e22f2c567c31)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-bfad922fa1af3fce)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target/debug/deps/templates_review_context-fe4bbb3b29030b3f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-0bf264ac0c8da865)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-cac23d174906ce7e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target/debug/deps/templates_user_guidance-64af293bfb18ed9a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target/debug/deps/traceability_e2e-9e3c3884834bbedf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/validate.rs (target/debug/deps/validate-5ca6ff709bd886cb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s

     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-2a0aee7106bc5b5c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/validate_rules_extension.rs (target/debug/deps/validate_rules_extension-988314b9c1bce9b5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (target/debug/deps/validate_tracking_rules-620620eb129743ab)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/worktree_ensure_e2e.rs (target/debug/deps/worktree_ensure_e2e-7c93d3f04d5edb2d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

```

```bash
cargo test -p ito-cli plan -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.55s
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 1.24s

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.11s

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
test agent_instruction_manifesto_planning_profile_is_advisory ... ok
test agent_instruction_manifesto_planning_profile_embeds_no_mutating_artifacts ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.09s

     Running tests/list_archive.rs (target/debug/deps/list_archive-aa095708a39e6d3d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/list_regression.rs (target/debug/deps/list_regression-1d992335e041b3a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/misc_more.rs (target/debug/deps/misc_more-7aa0bbbc6562dba7)

running 1 test
test plan_status_reports_missing_workspace ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.02s

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
test plan_status_reports_invalid_workspace_without_init_hint_loop ... ok
test plan_status_reports_missing_workspace_without_error ... ok
test plan_init_reports_conflicting_planning_file ... ok
test plan_status_lists_markdown_documents ... ok
test plan_status_reports_invalid_research_workspace ... ok
test plan_init_creates_structure ... ok
test plan_status_succeeds_after_init ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.09s

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
cargo test -p ito-core --test planning_init -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.44s
     Running tests/planning_init.rs (target/debug/deps/planning_init-a7d9eb36a801cb09)

running 7 tests
test read_planning_workspace_status_allows_missing_workspace ... ok
test read_planning_workspace_status_reports_conflicting_file ... ok
test init_planning_structure_creates_only_workspace ... ok
test read_planning_workspace_status_reports_conflicting_research_file ... ok
test init_planning_structure_errors_when_planning_path_is_a_file ... ok
test init_planning_structure_preserves_existing_plan_documents ... ok
test read_planning_workspace_status_lists_plan_documents ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
