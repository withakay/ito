# Task 2.1: Coordination-first archive lifecycle

*2026-04-24T06:28:01Z by Showboat 0.6.1*
<!-- showboat-id: 53083ab0-0275-4b11-b6b5-469310c6321d -->

Updated archive so worktree-backed setups synchronize the coordination branch after archive mutation and then present main-integration follow-up based on the configured archive mode. Legacy non-wired test repos still use the existing local archive path.

```bash
cargo test -p ito-cli archive -- --nocapture
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.20s
     Running unittests src/main.rs (target/debug/deps/ito-6d4676fcba121558)

running 3 tests

Next steps:
  Integrate the archived result for '025-09_add-worktree-sync-command' directly into main.

Next steps:
  Create an integration branch from main, apply the archived result for '025-09_add-worktree-sync-command', and open a PR.

Next steps:
  Create an integration branch from main, open a PR for '025-09_add-worktree-sync-command', and request auto-merge if policy allows.

Next steps:
  The archive is disseminated through the coordination branch; main integration for '025-09_add-worktree-sync-command' is pending manual follow-up.
test app::archive::tests::only_filesystem_mode_requires_local_changes_dir ... ok
test app::archive::tests::archive_follow_up_messages_cover_all_modes ... ok
test commands::config::config_tests::config_schema_includes_archive_main_integration_mode_default ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 62 filtered out; finished in 0.00s

     Running tests/agent_instruction_bootstrap.rs (target/debug/deps/agent_instruction_bootstrap-9d0a1e6df3a6997d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_context.rs (target/debug/deps/agent_instruction_context-8e07738d929c3d00)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_repo_sweep.rs (target/debug/deps/agent_instruction_repo_sweep-c89feac25192a721)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/agent_instruction_worktrees.rs (target/debug/deps/agent_instruction_worktrees-dd6ca92ee032fc44)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/aliases.rs (target/debug/deps/aliases-60641ba65fc52a06)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/archive_completed.rs (target/debug/deps/archive_completed-5c0ee0499a12130e)

running 7 tests
test archive_completed_conflict_with_positional ... ok
test archive_completed_no_completed_changes ... ok
test archive_completed_decline_confirmation_cancels ... ok
test archive_completed_empty_confirmation_cancels ... ok
test archive_completed_archives_all_completed ... ok
test archive_completed_accept_yes_confirmation_archives ... ok
test archive_completed_skip_specs ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.80s

     Running tests/archive_remote_mode.rs (target/debug/deps/archive_remote_mode-25ef0a1c271a7985)

running 1 test
ito-backend (multi-tenant) listening at http://127.0.0.1:61260/
  data_dir: /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpRDgvyS
  storage: filesystem
  admin_tokens: 1, token_seed: true
  allowed orgs: 1
test remote_archive_succeeds_without_local_active_change_markdown ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s

     Running tests/archive_smoke.rs (target/debug/deps/archive_smoke-768f7db9ce0360e3)

running 1 test
test archive_with_specs_and_validation_smoke ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

     Running tests/audit_more.rs (target/debug/deps/audit_more-9868b46e3eef7e21)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_remote_mode.rs (target/debug/deps/audit_remote_mode-f2cb0a563d8f0ac7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_import.rs (target/debug/deps/backend_import-b71f6eddf3cc7067)

running 1 test
ito-backend (multi-tenant) listening at http://127.0.0.1:61265/
  data_dir: /private/var/folders/fm/kc7zzw6n5lscp57b5_skwl8m0000gn/T/.tmpkNoWbx
  storage: filesystem
  admin_tokens: 1, token_seed: true
  allowed orgs: 1
test backend_import_writes_active_and_archived_changes_to_backend ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.16s

     Running tests/backend_qa_walkthrough.rs (target/debug/deps/backend_qa_walkthrough-99093e55c472f7c7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_serve.rs (target/debug/deps/backend_serve-e0450e45a263fa0a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_status_more.rs (target/debug/deps/backend_status_more-aa519b75c3f3154c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 20 filtered out; finished in 0.00s

     Running tests/cli_smoke.rs (target/debug/deps/cli_smoke-96370ce6c164066c)

running 1 test
test agent_instruction_status_archive_smoke ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.14s

     Running tests/cli_snapshots.rs (target/debug/deps/cli_snapshots-7c1730df16091dce)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/config_more.rs (target/debug/deps/config_more-2b07fe477bfa3aa6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/coverage_smoke.rs (target/debug/deps/coverage_smoke-7ee31e863e055e38)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/create_more.rs (target/debug/deps/create_more-1eb549e6ee2b19f4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/grep_more.rs (target/debug/deps/grep_more-12d6fa086b067b60)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/help.rs (target/debug/deps/help-47cf4e4502e137a2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/init_coordination.rs (target/debug/deps/init_coordination-a8e1fe050cfe618d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/init_gitignore_session_json.rs (target/debug/deps/init_gitignore_session_json-02d4d8af74b870e1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/init_more.rs (target/debug/deps/init_more-990e9107201c627c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out; finished in 0.00s

     Running tests/init_tmux.rs (target/debug/deps/init_tmux-0cb603e1b3d3489e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/init_upgrade_more.rs (target/debug/deps/init_upgrade_more-bf0b5ccaa73da775)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/instructions_more.rs (target/debug/deps/instructions_more-5a6197adeb4e71b2)

running 3 tests
test agent_instruction_archive_with_invalid_change_fails ... ok
test agent_instruction_archive_without_change_prints_generic_guidance ... ok
test agent_instruction_archive_with_change_prints_targeted_instruction ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.04s

     Running tests/list_regression.rs (target/debug/deps/list_regression-267bcdf7f3cacc50)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/misc_more.rs (target/debug/deps/misc_more-ebc8cb0d8172f0d7)

running 1 test
test archive_prompts_on_incomplete_tasks_and_proceeds_when_confirmed ... ignored, PTY interactive test — can hang in CI; run with --ignored locally

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/new_more.rs (target/debug/deps/new_more-84efd7f8317ba173)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/parity_help_version.rs (target/debug/deps/parity_help_version-02a624cd49df9249)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/parity_tasks.rs (target/debug/deps/parity_tasks-45e2840c229ce443)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/path_more.rs (target/debug/deps/path_more-1e8cceb044034fbf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/plan_state_more.rs (target/debug/deps/plan_state_more-5353e3be3e8ea721)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph_smoke.rs (target/debug/deps/ralph_smoke-92469ed76d149f4f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/serve_more.rs (target/debug/deps/serve_more-1f71060fb81d761e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show_specs_bundle.rs (target/debug/deps/show_specs_bundle-70fb160f2eeb6046)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/show_specs_remote_mode.rs (target/debug/deps/show_specs_remote_mode-5fa7552010144487)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/source_file_size.rs (target/debug/deps/source_file_size-a7080b328c132c9d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-0d4b9f631759b0d6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_more.rs (target/debug/deps/tasks_more-b7d7c0d6dad51920)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/tasks_remote_mode.rs (target/debug/deps/tasks_remote_mode-e5b4156ce931f306)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_schemas_export.rs (target/debug/deps/templates_schemas_export-51dce4828d63c6e5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/trace_more.rs (target/debug/deps/trace_more-0a0264026e6b215f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/update_smoke.rs (target/debug/deps/update_smoke-1406c2447b2bab42)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/user_guidance_injection.rs (target/debug/deps/user_guidance_injection-0d0e4f282997dfba)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/validate_more.rs (target/debug/deps/validate_more-9f3775453f1e4c72)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/view_proposal.rs (target/debug/deps/view_proposal-0fca87d4c168555a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

```
