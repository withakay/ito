# 001-28: Embed tmux skill and helper scripts

*2026-03-22T10:56:55Z by Showboat 0.6.1*
<!-- showboat-id: 7fe7bdab-6f7e-42c4-9eb6-2383bfe80b39 -->

Embedded the upstream tmux skill into ito-templates, added helper scripts, taught the skill to respect tools.tmux.enabled, and verified OpenCode installs the scripts with executable permissions.

```bash
cd ito-rs && cargo test -p ito-templates tmux_skill_and_scripts_are_embedded 2>&1
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.03s
     Running unittests src/lib.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/ito_templates-cb3052cb4f694ff3)

running 1 test
test tests::tmux_skill_and_scripts_are_embedded ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 38 filtered out; finished in 0.00s

     Running tests/template_markdown.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/template_markdown-6a50b3a39df3821a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/user_guidance_template.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/user_guidance_template-bdfe2fbaa8fbf4b2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/worktree_template_rendering.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/worktree_template_rendering-647c504bf051f08d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

```

```bash
cd ito-rs && cargo test -p ito-core install_default_templates_makes_tmux_skill_scripts_executable 2>&1
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/ito_core-719960b1bbcfd818)

running 1 test
test installers::tests::install_default_templates_makes_tmux_skill_scripts_executable ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 411 filtered out; finished in 0.01s

     Running tests/archive.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/archive-d3a9d1bfd6d907f4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/audit_mirror-007a5e2ed3d4817f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/audit_storage-91ed3da2c77c28dd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/backend_archive-b9dad70afc462772)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/backend_auth-e2d6bdbc8abeca19)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/backend_auth_service-aae0188ffc0b1e59)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/backend_client_mode-77bb6bae3bf64b46)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/backend_module_repository-b255d6f900f72e34)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/change_repository_lifecycle-69f65c6646a1902e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/change_repository_parity-ff55a14e6030c31f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/change_target_resolution_parity-f82a1e025337d40d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/create.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/create-0199a28dc5adf5c9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/distribution.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/distribution-8c94ad7e63563442)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/event_forwarding-37dab5c4369bc8a2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/grep_scopes-5b13865ba331aac1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/harness_context-af76bc60705db536)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/harness_opencode-f512cf551e25d2f9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/harness_streaming-2ba933b0521dc87f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/harness_stub-a7f00e9b9b9efc4b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/import-6fe001007edeee4d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/io-e9eefd5e4045f5f9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/planning_init.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/planning_init-a9a9867c2380fb65)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/ralph-0cb1d906fb238a71)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s

     Running tests/repo_index.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/repo_index-dc94359d93830623)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/repo_integrity-cd9f28e9dcb208ea)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/repo_paths-76fb7dee68b31f70)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/repository_runtime-667f35bd3bcddc80)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/repository_runtime_config_validation-c28a4856a332077d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/show-9da22de02c48a865)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/spec_repository_backends-11614744a3d4de86)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/spec_show_repository-7b27bb5a64b3ff54)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/sqlite_archive_mirror-a5f791989cda6134)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/sqlite_task_mutations-b998e9ee0f03833d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/stats-e0e7c13850ca1157)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/task_repository_summary-9c118b6df5864e42)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/tasks_api-607e6c7c0943484a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/tasks_checkbox_format-57ea8b4fa038aff9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/tasks_orchestration-e192639b82895b70)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/templates_apply_instructions-226edf3504a7c6f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/templates_change_status-f9616b1ab04fb3cf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/templates_review_context-35f989c0098496bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/templates_schema_resolution-47595fad4489e9f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/templates_schemas_listing-a3cb4ab82a4bbd5c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/templates_user_guidance-3dbb41abd7f02f46)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/validate.rs (/Users/jack/Code/withakay/ito/ito-worktrees/001-31_tools-config-tmux/target/debug/deps/validate-e67f0b1e9606dd78)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.00s

```

```bash
grep -n 'Ito Integration\|tools.tmux.enabled' ito-rs/crates/ito-templates/assets/skills/tmux/SKILL.md
```

```output
13:## Ito Integration
15:Before suggesting any tmux-based workflow step in an Ito project, check the resolved Ito config for `tools.tmux.enabled`.
17:- If `tools.tmux.enabled = false`, omit tmux suggestions entirely and do not recommend tmux-based alternatives.
18:- If `tools.tmux.enabled = true` (or the key is absent), follow the guidance in this skill as normal.
```
