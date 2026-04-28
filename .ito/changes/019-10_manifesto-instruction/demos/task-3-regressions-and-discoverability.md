# Wave 3: Manifesto Regression and Discoverability

*2026-04-27T21:36:06Z by Showboat 0.6.1*
<!-- showboat-id: f0c390ea-290a-4cba-9829-faa6df9401e5 -->

Added regression coverage for planning/apply-ready/applying/finished manifesto states, selector discoverability, and redaction behavior.

```bash
cargo test -p ito-cli --test help --test instructions_more && cargo test -p ito-cli --test agent_instruction_context && cargo test -p ito-core coordination_worktree
```

```output
warning: unused import: `serde_json::Value`
  --> ito-rs/crates/ito-cli/src/app/instructions.rs:12:5
   |
12 | use serde_json::Value;
   |     ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `ArchiveInstructionConfig`, `TestingPolicy`, and `WorktreeConfig`
  --> ito-rs/crates/ito-cli/src/app/manifesto_instructions.rs:19:5
   |
19 |     ArchiveInstructionConfig, TestingPolicy, WorktreeConfig,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: `ito-cli` (bin "ito") generated 2 warnings (run `cargo fix --bin "ito" -p ito-cli` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.20s
     Running tests/help.rs (target/debug/deps/help-aa42fd8589824328)

running 7 tests
test agent_instruction_help_shows_instruction_details ... ok
test help_shows_navigation_footer ... ok
test help_prints_usage ... ok
test help_all_global_flag_works ... ok
test dash_h_help_matches_dash_dash_help ... ok
test help_all_shows_complete_reference ... ok
test help_all_json_outputs_valid_json ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.48s

     Running tests/instructions_more.rs (target/debug/deps/instructions_more-9654ca94826e629f)

running 28 tests
test agent_instruction_apply_text_is_compact_and_has_trailing_newline ... ok
test agent_instruction_change_flag_supports_shorthand ... ok
test agent_instruction_change_flag_reports_ambiguous_target ... ok
test agent_instruction_archive_without_change_prints_generic_guidance ... ok
test agent_instruction_change_flag_supports_slug_query ... ok
test agent_instruction_archive_with_change_prints_targeted_instruction ... ok
test agent_instruction_archive_with_invalid_change_fails ... ok
test agent_instruction_finish_with_change_prompts_for_archive ... ok
test agent_instruction_manifesto_full_variant_rejects_incompatible_operation ... ok
test agent_instruction_manifesto_change_scope_reports_apply_ready_state ... ok
test agent_instruction_manifesto_change_scope_json_reports_state ... ok
test agent_instruction_manifesto_change_scope_reports_applying_state ... ok
test agent_instruction_manifesto_change_scope_includes_change_state ... ok
test agent_instruction_manifesto_full_variant_embeds_allowed_default_set ... ok
test agent_instruction_manifesto_full_variant_renders_full_section ... ok
test agent_instruction_manifesto_full_variant_embeds_requested_proposal_instruction ... ok
test agent_instruction_proposal_honors_testing_policy_override ... ok
test agent_instruction_manifesto_rejects_operation_for_light_variant ... ok
test agent_instruction_proposal_without_change_prints_new_proposal_guide ... ok
test agent_instruction_proposal_without_change_supports_json_output ... ok
test agent_instruction_manifesto_redacts_explicit_coordination_path ... ok
test agent_instruction_review_requires_change_flag ... ok
test agent_instruction_text_output_renders_artifact_envelope ... ok
test agent_instruction_manifesto_uses_default_variant_and_profile ... ok
test agent_instruction_manifesto_full_variant_supports_finish_for_archived_change ... ok
test agent_instruction_manifesto_planning_profile_is_advisory ... ok
test agent_instruction_manifesto_json_includes_resolved_defaults ... ok
test agent_instruction_review_renders_review_template ... ok

test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.67s

warning: unused import: `serde_json::Value`
  --> ito-rs/crates/ito-cli/src/app/instructions.rs:12:5
   |
12 | use serde_json::Value;
   |     ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `ArchiveInstructionConfig`, `TestingPolicy`, and `WorktreeConfig`
  --> ito-rs/crates/ito-cli/src/app/manifesto_instructions.rs:19:5
   |
19 |     ArchiveInstructionConfig, TestingPolicy, WorktreeConfig,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^^^^^

warning: `ito-cli` (bin "ito") generated 2 warnings (run `cargo fix --bin "ito" -p ito-cli` to apply 2 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.20s
     Running tests/agent_instruction_context.rs (target/debug/deps/agent_instruction_context-fb7b0e7b64be5175)

running 2 tests
Switched to a new branch '023-07_harness-context-inference'
test agent_instruction_context_prefers_path_inference_in_text_output ... ok
test agent_instruction_context_supports_json_output ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.58s

    Finished `test` profile [optimized + debuginfo] target(s) in 0.19s
     Running unittests src/lib.rs (target/debug/deps/ito_core-97a60e89e32a20a6)

running 27 tests
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_worktree_add_fails ... ok
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_orphan_commit_fails ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_commit_fails ... ok
test coordination_worktree::coordination_worktree_tests::create_returns_error_when_fetch_fails_unexpectedly ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_stages_and_commits_when_changes_exist ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_returns_error_when_git_add_fails ... ok
test coordination_worktree::coordination_worktree_tests::auto_commit_is_noop_when_nothing_staged ... ok
test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_force_remove_also_fails ... ok
test coordination_worktree::coordination_worktree_tests::remove_runs_worktree_remove_then_prune ... ok
test coordination_worktree::coordination_worktree_tests::remove_returns_error_when_prune_fails ... ok
test coordination_worktree::coordination_worktree_tests::remove_falls_back_to_force_when_clean_remove_fails ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_storage_is_embedded ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback ... ok
test coordination_worktree::coordination_worktree_tests::create_uses_existing_local_branch ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_when_origin_not_configured ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_is_noop_when_storage_is_embedded ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_when_not_on_remote ... ok
test coordination_worktree::coordination_worktree_tests::create_fetches_branch_from_origin_when_not_local ... ok
test coordination_worktree::coordination_worktree_tests::create_makes_orphan_branch_via_commit_tree_fallback_in_sha256_repo ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_fetches_commits_and_pushes_when_healthy ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_force_bypasses_rate_limit ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_rate_limits_when_recent_and_clean ... ok
test coordination_worktree::coordination_worktree_tests::sync_coordination_worktree_returns_error_when_links_point_to_wrong_target ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_is_noop_when_worktree_dir_does_not_exist ... ok
test coordination_worktree::coordination_worktree_tests::maybe_auto_commit_calls_auto_commit_when_worktree_mode_and_dir_exists ... ok
test coordination_worktree::coordination_worktree_tests::integration_create_and_remove_coordination_worktree ... ok
test coordination_worktree::coordination_worktree_tests::integration_auto_commit_coordination ... ok

test result: ok. 27 passed; 0 failed; 0 ignored; 0 measured; 562 filtered out; finished in 1.40s

     Running tests/archive.rs (target/debug/deps/archive-bb908e19f60ab3de)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target/debug/deps/audit_mirror-e0f2007be96afc56)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (target/debug/deps/audit_storage-d1066ef9170e3525)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target/debug/deps/backend_archive-7d213b5a86171714)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (target/debug/deps/backend_auth-f62b4033e87eb880)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target/debug/deps/backend_auth_service-d4ae41eb0dbebf8d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target/debug/deps/backend_client_mode-4feccb8304cdd0e9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target/debug/deps/backend_module_repository-142fb0a15ad04557)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (target/debug/deps/backend_sub_module_support-b6cbe69060670271)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target/debug/deps/change_repository_lifecycle-3ad7a416a1aedd7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (target/debug/deps/change_repository_orchestrate_metadata-3b7eeb07eb743455)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target/debug/deps/change_repository_parity-35ea3d4714a6ad6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (target/debug/deps/change_target_resolution_parity-421699c569d2d487)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/coordination_worktree.rs (target/debug/deps/coordination_worktree-5975c00a55b1bb53)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/create.rs (target/debug/deps/create-5ac295c2581c09f9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/distribution.rs (target/debug/deps/distribution-f9b9a052767379fb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (target/debug/deps/event_forwarding-c6970aa933776f62)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (target/debug/deps/grep_scopes-9f15bca114771057)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target/debug/deps/harness_context-545831b8623ed23b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (target/debug/deps/harness_opencode-6f5c13f4dd80d54c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (target/debug/deps/harness_streaming-1c0b5d5428cea5bc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (target/debug/deps/harness_stub-c30f2a7613933814)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (target/debug/deps/import-f1b2c7ccf7d0ce7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (target/debug/deps/io-2236f3068a97b57f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (target/debug/deps/orchestrate_run_state-55a74a408992e8da)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target/debug/deps/planning_init-89cbafa119c3ee0a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph.rs (target/debug/deps/ralph-be5f667b52cc59cc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s

     Running tests/repo_index.rs (target/debug/deps/repo_index-fd4e043b4d94b0ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target/debug/deps/repo_integrity-2d3b3a3e061715a9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target/debug/deps/repo_paths-fb1f07c85698c734)

running 7 tests
test coordination_worktree_path_correct_structure_with_home_fallback ... ok
test coordination_worktree_path_correct_structure_with_xdg ... ok
test coordination_worktree_path_last_resort_uses_ito_path ... ok
test coordination_worktree_path_ignores_xdg_when_explicit_path_set ... ok
test coordination_worktree_path_uses_explicit_worktree_path_when_set ... ok
test coordination_worktree_path_uses_xdg_data_home_when_set ... ok
test coordination_worktree_path_falls_back_to_local_share_when_xdg_unset ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (target/debug/deps/repository_runtime-be4c763d081a4e20)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (target/debug/deps/repository_runtime_config_validation-960aa3cc8f00a101)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (target/debug/deps/show-22113dd2550b37fc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target/debug/deps/spec_repository_backends-c6818389cc643375)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target/debug/deps/spec_show_repository-b1904cdaddfc9087)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target/debug/deps/sqlite_archive_mirror-d956e7d08092e7de)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target/debug/deps/sqlite_task_mutations-0b7bbed8ba1f98e7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-67eda39bc41dbd37)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target/debug/deps/task_repository_summary-95f5bd2fb9f29729)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target/debug/deps/tasks_api-bd8ead6872f8b435)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (target/debug/deps/tasks_checkbox_format-48281f0d57319aea)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target/debug/deps/tasks_orchestration-3999942f641c0981)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-9f2019f85720f737)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-35a12530eba5e6f4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target/debug/deps/templates_review_context-97f6e8db807d81a0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-36c0da440e7dbadb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-f2f679d65100a642)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target/debug/deps/templates_user_guidance-cf81a0bcce9439ca)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (target/debug/deps/traceability_e2e-8fc11d42d04ae5a6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/validate.rs (target/debug/deps/validate-254d2cfeabee182c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s

     Running tests/validate_delta_rules.rs (target/debug/deps/validate_delta_rules-6892e9025fcc274c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/validate_rules_extension.rs (target/debug/deps/validate_rules_extension-f05e7c13a4efe547)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (target/debug/deps/validate_tracking_rules-4408fcfe66def8db)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/worktree_ensure_e2e.rs (target/debug/deps/worktree_ensure_e2e-415d396e3697aff4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

```

```bash
target/debug/ito agent instruction -h
```

```output
Generate enriched instructions

Usage: ito agent instruction [OPTIONS] <ARTIFACT>

Arguments:
  <ARTIFACT>  Artifact id (e.g. bootstrap, apply, proposal)

Options:
  -c, --change <CHANGE>        Change id (directory name)
      --no-color               Disable color output
      --help-all               Print the full CLI reference (equivalent to `ito help --all`)
      --tool <TOOL>            Tool name for bootstrap (opencode|claude|codex)
      --schema <SCHEMA>        Workflow schema name
      --json                   Output as JSON
      --variant <VARIANT>      Manifesto output variant (light|full)
      --profile <PROFILE>      Manifesto capability profile (planning|proposal-only|review-only|apply|archive|full)
      --operation <OPERATION>  Manifesto operation selector for full renders
      --context <CONTEXT>      Free-form context for `memory-capture`
      --file <FILE>            File path for `memory-capture` (repeatable)
      --folder <FOLDER>        Folder path for `memory-capture` (repeatable)
      --query <QUERY>          Search/query input for `memory-search` and `memory-query`
      --limit <LIMIT>          Limit for `memory-search` (positive integer)
      --scope <SCOPE>          Scope for `memory-search`
  -h, --help                   Print help

Artifacts:
  bootstrap                          Generate a tool bootstrap preamble
  project-setup                      Guide for setting up a new project
  backend                            Backend server and client configuration guide
  worktrees                          Guide for git worktree workflow (config-driven)
  repo-sweep                         Scan for old-only ID format assumptions in prompts and templates
  migrate-to-coordination-worktree   Guide for migrating from embedded to worktree storage
  orchestrate                        Orchestrate applying a set of changes via an orchestrator agent
  manifesto                          Generate a strict Ito manifesto for prompt-only execution
  proposal                           Show the change proposal
  specs                              Show the specification deltas
  tasks                              Show the implementation task list
  apply                              Show implementation instructions
  review                             Show review instructions
  archive                            Show archive instructions
  finish                             Cleanup worktrees and branches after merge
  memory-capture                     Capture durable knowledge through configured memory
  memory-search                      Search configured memory for ranked matches
  memory-query                       Query configured memory for a synthesized answer

Examples:
  ito agent instruction bootstrap --tool opencode
  ito agent instruction project-setup
  ito agent instruction backend
  ito agent instruction worktrees
  ito agent instruction repo-sweep
  ito agent instruction migrate-to-coordination-worktree
  ito agent instruction orchestrate
  ito agent instruction manifesto
  ito agent instruction manifesto --variant full --profile proposal-only
  ito agent instruction proposal --change 005-08_migrate-cli-to-clap
  ito agent instruction apply --change 005-08_migrate-cli-to-clap
  ito agent instruction archive
  ito agent instruction archive --change 005-08_migrate-cli-to-clap
  ito agent instruction finish --change 005-08_migrate-cli-to-clap
  ito agent instruction memory-capture --context "Decision and rationale" --file docs/config.md
  ito agent instruction memory-search --query "archive workflow" --limit 5
  ito agent instruction memory-query --query "How should agents capture memories?"
```
