# Force init obsolete specialist cleanup remediation

*2026-04-29T06:55:19Z by Showboat 0.6.1*
<!-- showboat-id: 69830d9b-ebdf-4c23-a288-a9ec09129cc6 -->

Verified the reviewer finding with a new force-init regression test, then broadened obsolete specialist cleanup so reinstall paths remove stale ito-orchestrator-* specialist assets during forceful init as well as update flows.

```bash
cargo test -p ito-cli --test init_obsolete_cleanup
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 1.66s
     Running tests/init_obsolete_cleanup.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/init_obsolete_cleanup-fecffacf4ba25ffc)

running 2 tests
test init_force_with_tools_all_removes_obsolete_specialist_orchestrator_assets ... ok
test init_update_with_tools_all_removes_obsolete_specialist_orchestrator_assets ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.75s

```

```bash
python3 -c 'from pathlib import Path; files=[("ito-rs/crates/ito-core/src/installers/mod.rs", 890, 910), ("ito-rs/crates/ito-cli/tests/init_obsolete_cleanup.rs", 1, 80)]; exec("for path,start,end in files:\n print(f\"== {path} ==\")\n lines=Path(path).read_text().splitlines()\n for idx in range(start-1, min(end, len(lines))): print(f\"{idx+1}: {lines[idx]}\")\n print()")'
```

```output
== ito-rs/crates/ito-core/src/installers/mod.rs ==
890:     for (tool_id, harness) in tool_harness_map {
891:         if !opts.tools.contains(tool_id) {
892:             continue;
893:         }
894: 
895:         let agent_dir = project_root.join(harness.project_agent_path());
896:         let should_remove_obsolete_specialists =
897:             mode == InstallMode::Update || opts.update || opts.force;
898: 
899:         // Get agent template files for this harness
900:         let files = get_agent_files(harness);
901: 
902:         for (rel_path, contents) in files {
903:             if should_remove_obsolete_specialists
904:                 && let Some(obsolete_rel_path) = obsolete_specialist_agent_rel_path(rel_path)
905:             {
906:                 remove_obsolete_specialist_agent(&agent_dir, obsolete_rel_path)?;
907:             }
908: 
909:             let target = agent_dir.join(rel_path);
910: 

== ito-rs/crates/ito-cli/tests/init_obsolete_cleanup.rs ==
1: #[path = "support/mod.rs"]
2: mod fixtures;
3: 
4: use ito_test_support::run_rust_candidate;
5: 
6: use crate::fixtures::specialist_asset_paths;
7: 
8: const COORDINATOR_PATHS: &[&str] = &[
9:     ".opencode/agent/ito-orchestrator.md",
10:     ".claude/agents/ito-orchestrator.md",
11:     ".github/agents/ito-orchestrator.md",
12:     ".pi/agents/ito-orchestrator.md",
13:     ".agents/skills/ito-orchestrator/SKILL.md",
14: ];
15: 
16: #[test]
17: fn init_update_with_tools_all_removes_obsolete_specialist_orchestrator_assets() {
18:     assert_specialist_cleanup(&["--update"]);
19: }
20: 
21: #[test]
22: fn init_force_with_tools_all_removes_obsolete_specialist_orchestrator_assets() {
23:     assert_specialist_cleanup(&["--force"]);
24: }
25: 
26: fn assert_specialist_cleanup(extra_args: &[&str]) {
27:     let base = fixtures::make_empty_repo();
28:     let repo = tempfile::tempdir().expect("work");
29:     let home = tempfile::tempdir().expect("home");
30:     let rust_path = assert_cmd::cargo::cargo_bin!("ito");
31: 
32:     fixtures::reset_repo(repo.path(), base.path());
33: 
34:     let obsolete = specialist_asset_paths("ito-orchestrator-");
35:     for rel in &obsolete {
36:         fixtures::write(repo.path().join(rel), "obsolete specialist asset\n");
37:     }
38: 
39:     let repo_path = repo.path().to_string_lossy();
40:     let mut argv = vec!["init", repo_path.as_ref(), "--tools", "all"];
41:     argv.extend_from_slice(extra_args);
42:     let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
43:     assert_eq!(out.code, 0, "stderr={}", out.stderr);
44: 
45:     for rel in &obsolete {
46:         assert!(
47:             !repo.path().join(rel).exists(),
48:             "expected obsolete specialist asset {rel} to be removed"
49:         );
50:     }
51: 
52:     for rel in specialist_asset_paths("ito-") {
53:         assert!(repo.path().join(&rel).exists(), "expected {rel} to install");
54:     }
55: 
56:     for rel in COORDINATOR_PATHS {
57:         assert!(
58:             repo.path().join(rel).exists(),
59:             "expected coordinator asset {rel} to remain installed"
60:         );
61:     }
62: }

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

Follow-up remediation review tightened the cleanup into a harness-level pre-pass, documented the migration scope, and added a unit test proving broken obsolete specialist symlinks are removed cleanly.

```bash
cargo test -p ito-core removes_broken_specialist_symlinks_and_prunes_empty_dirs && cargo test -p ito-cli --test init_obsolete_cleanup
```

```output
   Compiling ito-core v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/ito-rs/crates/ito-core)
    Finished `test` profile [optimized + debuginfo] target(s) in 4.75s
     Running unittests src/lib.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/ito_core-2c0501004319aa04)

running 1 test
test installers::agents_cleanup::tests::removes_broken_specialist_symlinks_and_prunes_empty_dirs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 599 filtered out; finished in 0.00s

     Running tests/archive.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/archive-1e441e8f2599fd3d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/audit_mirror-591d1eab1ac17556)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/audit_storage-74b1669dd273dc32)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/backend_archive-0aa5f84517587eac)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/backend_auth-16e90dd520389c08)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/backend_auth_service-1b852594cb27a447)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/backend_client_mode-ae08737de38c0dc3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/backend_module_repository-2263ca4bc4e714a8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_sub_module_support.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/backend_sub_module_support-56350f4b268d6810)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/change_repository_lifecycle-078bc5da9d44cea0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_orchestrate_metadata.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/change_repository_orchestrate_metadata-bf6a6e8e2b0bf8fb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/change_repository_parity-813ba746e3ec2d0e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/change_target_resolution_parity-49fe3128624782cb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/coordination_worktree.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/coordination_worktree-55cba6c7719c5a20)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/create.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/create-c5e723a8111a8f02)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/distribution.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/distribution-35c416baf8598c0d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/event_forwarding-4b7474e7578f6cc3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/grep_scopes-08743232d114310f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/harness_context-2858105d86e7712e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/harness_opencode-8fb9191c06ae6b86)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/harness_streaming-d017be767dd483e8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/harness_stub-4b6033dfc270acc1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/import-cf2cf6e922e62d26)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/io-57a32d4a33a5c750)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/orchestrate_run_state.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/orchestrate_run_state-df8dd2e341748585)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/planning_init.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/planning_init-bea70fbe91dfbe19)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/ralph-4d13ca2184206900)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 0.00s

     Running tests/repo_index.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/repo_index-f2a40a2feaa53be3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/repo_integrity-5fb282e2c0954442)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/repo_paths-1cf0c23e35fb53cd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/repository_runtime-8d5da2a976d7b2f0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/repository_runtime_config_validation-056dfd56217cf6bb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/show-35a3d9f0d0cba11b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/spec_repository_backends-14e65931066406ed)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/spec_show_repository-0dbb9311a789e963)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/sqlite_archive_mirror-4dcbe8af9e93d25a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/sqlite_task_mutations-6fa3f601f0420373)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/stats-fdd65ea6b27f1871)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/task_repository_summary-bea4ca4b42bc8d53)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/tasks_api-99836ad3bbb88984)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/tasks_checkbox_format-f15f136a82ed3479)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/tasks_orchestration-e62428fe6fec2c99)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/templates_apply_instructions-62f16cea2c1fd28e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/templates_change_status-124bdabc9a3b5538)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/templates_review_context-8ceff7bfe5f90877)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/templates_schema_resolution-403d2934dca93895)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/templates_schemas_listing-6a8963b69d82d7d2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/templates_user_guidance-c04a0b9ff309699b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/traceability_e2e.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/traceability_e2e-97b1a17c0541bf62)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/validate.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/validate-d020b20e4aeab49f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s

     Running tests/validate_delta_rules.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/validate_delta_rules-01440800bf6c477a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/validate_rules_extension.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/validate_rules_extension-e75d7555338dea96)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/validate_tracking_rules.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/validate_tracking_rules-41350eeb32244295)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/worktree_ensure_e2e.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/worktree_ensure_e2e-8e817f71bae0c07d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

   Compiling ito-core v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/ito-rs/crates/ito-core)
   Compiling ito-backend v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/ito-rs/crates/ito-backend)
   Compiling ito-web v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/ito-rs/crates/ito-web)
   Compiling ito-cli v0.1.30 (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/ito-rs/crates/ito-cli)
    Finished `test` profile [optimized + debuginfo] target(s) in 16.28s
     Running tests/init_obsolete_cleanup.rs (/Users/jack/Code/withakay/ito/ito-worktrees/019-11_rename-orchestrator-assets/target/debug/deps/init_obsolete_cleanup-fecffacf4ba25ffc)

running 2 tests
test init_force_with_tools_all_removes_obsolete_specialist_orchestrator_assets ... ok
test init_update_with_tools_all_removes_obsolete_specialist_orchestrator_assets ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.73s

```

```bash
python3 -c 'from pathlib import Path; files=[("ito-rs/crates/ito-core/src/installers/agents_cleanup.rs", 1, 120), ("ito-rs/crates/ito-core/src/installers/mod.rs", 890, 910), ("ito-rs/crates/ito-cli/tests/init_obsolete_cleanup.rs", 1, 70)]; exec("for path,start,end in files:\n print(f\"== {path} ==\")\n lines=Path(path).read_text().splitlines()\n for idx in range(start-1, min(end, len(lines))): print(f\"{idx+1}: {lines[idx]}\")\n print()")'
```

```output
== ito-rs/crates/ito-core/src/installers/agents_cleanup.rs ==
1: use std::{io::ErrorKind, path::Path};
2: 
3: use crate::errors::{CoreError, CoreResult};
4: 
5: /// Legacy specialist asset paths renamed from `ito-orchestrator-*` to `ito-*`.
6: ///
7: /// This migration intentionally excludes the top-level `ito-orchestrator` and
8: /// `ito-orchestrator-workflow` assets, which keep their existing names.
9: const OBSOLETE_SPECIALIST_AGENT_REL_PATHS: &[&str] = &[
10:     "ito-orchestrator-planner.md",
11:     "ito-orchestrator-researcher.md",
12:     "ito-orchestrator-reviewer.md",
13:     "ito-orchestrator-worker.md",
14:     "ito-orchestrator-planner/SKILL.md",
15:     "ito-orchestrator-researcher/SKILL.md",
16:     "ito-orchestrator-reviewer/SKILL.md",
17:     "ito-orchestrator-worker/SKILL.md",
18: ];
19: 
20: pub(super) fn remove_obsolete_specialist_agents(agent_dir: &Path) -> CoreResult<()> {
21:     for obsolete_rel_path in OBSOLETE_SPECIALIST_AGENT_REL_PATHS {
22:         remove_obsolete_specialist_agent(agent_dir, obsolete_rel_path)?;
23:     }
24: 
25:     Ok(())
26: }
27: 
28: pub(super) fn remove_obsolete_specialist_agent(
29:     agent_dir: &Path,
30:     obsolete_rel_path: &str,
31: ) -> CoreResult<()> {
32:     let obsolete = agent_dir.join(obsolete_rel_path);
33:     let metadata = match std::fs::symlink_metadata(&obsolete) {
34:         Ok(metadata) => metadata,
35:         Err(err) if err.kind() == ErrorKind::NotFound => return Ok(()),
36:         Err(err) => {
37:             return Err(CoreError::io(
38:                 format!("reading {}", obsolete.display()),
39:                 err,
40:             ));
41:         }
42:     };
43: 
44:     let file_type = metadata.file_type();
45:     if file_type.is_file() || file_type.is_symlink() {
46:         std::fs::remove_file(&obsolete)
47:             .map_err(|e| CoreError::io(format!("removing {}", obsolete.display()), e))?;
48:     } else {
49:         return Err(CoreError::Validation(format!(
50:             "expected obsolete specialist agent path to be a file or symlink: {}. Remove the directory manually and rerun the install.",
51:             obsolete.display()
52:         )));
53:     }
54:     prune_empty_agent_dirs(agent_dir, obsolete.parent())
55: }
56: 
57: fn prune_empty_agent_dirs(agent_dir: &Path, start: Option<&Path>) -> CoreResult<()> {
58:     let mut current = start.map(Path::to_path_buf);
59: 
60:     while let Some(dir) = current {
61:         if dir == agent_dir || !dir.starts_with(agent_dir) {
62:             break;
63:         }
64:         let mut entries = std::fs::read_dir(&dir)
65:             .map_err(|e| CoreError::io(format!("reading {}", dir.display()), e))?;
66:         if entries
67:             .next()
68:             .transpose()
69:             .map_err(|e| CoreError::io(format!("reading {}", dir.display()), e))?
70:             .is_some()
71:         {
72:             break;
73:         }
74:         std::fs::remove_dir(&dir)
75:             .map_err(|e| CoreError::io(format!("removing {}", dir.display()), e))?;
76:         current = dir.parent().map(Path::to_path_buf);
77:     }
78:     Ok(())
79: }
80: 
81: #[cfg(test)]
82: mod tests {
83:     use super::*;
84: 
85:     #[cfg(unix)]
86:     #[test]
87:     fn removes_broken_specialist_symlinks_and_prunes_empty_dirs() {
88:         use std::os::unix::fs::symlink;
89: 
90:         let tempdir = tempfile::tempdir().expect("tempdir");
91:         let agent_dir = tempdir.path().join(".agents/skills");
92:         let obsolete_dir = agent_dir.join("ito-orchestrator-planner");
93:         std::fs::create_dir_all(&obsolete_dir).expect("obsolete dir");
94: 
95:         let obsolete = obsolete_dir.join("SKILL.md");
96:         symlink("missing-target.md", &obsolete).expect("symlink");
97: 
98:         remove_obsolete_specialist_agent(&agent_dir, "ito-orchestrator-planner/SKILL.md")
99:             .expect("cleanup succeeds");
100: 
101:         assert!(
102:             !obsolete.exists(),
103:             "broken obsolete symlink should be removed"
104:         );
105:         assert!(
106:             std::fs::symlink_metadata(&obsolete).is_err(),
107:             "removed symlink should no longer have metadata"
108:         );
109:         assert!(
110:             !obsolete_dir.exists(),
111:             "empty legacy specialist directory should be pruned"
112:         );
113:     }
114: }

== ito-rs/crates/ito-core/src/installers/mod.rs ==
890:     for (tool_id, harness) in tool_harness_map {
891:         if !opts.tools.contains(tool_id) {
892:             continue;
893:         }
894: 
895:         let agent_dir = project_root.join(harness.project_agent_path());
896:         // Update-style installs and forceful re-inits should both clear the
897:         // legacy `ito-orchestrator-*` specialist assets before writing the new
898:         // `ito-*` names. Plain init keeps untouched user files in place.
899:         let should_remove_obsolete_specialists =
900:             mode == InstallMode::Update || opts.update || opts.force;
901:         if should_remove_obsolete_specialists {
902:             remove_obsolete_specialist_agents(&agent_dir)?;
903:         }
904: 
905:         // Get agent template files for this harness
906:         let files = get_agent_files(harness);
907: 
908:         for (rel_path, contents) in files {
909:             let target = agent_dir.join(rel_path);
910: 

== ito-rs/crates/ito-cli/tests/init_obsolete_cleanup.rs ==
1: #[path = "support/mod.rs"]
2: mod fixtures;
3: 
4: use ito_test_support::run_rust_candidate;
5: 
6: use crate::fixtures::{installed_specialist_asset_paths, obsolete_specialist_asset_paths};
7: 
8: const COORDINATOR_PATHS: &[&str] = &[
9:     ".opencode/agent/ito-orchestrator.md",
10:     ".claude/agents/ito-orchestrator.md",
11:     ".github/agents/ito-orchestrator.md",
12:     ".pi/agents/ito-orchestrator.md",
13:     ".agents/skills/ito-orchestrator/SKILL.md",
14: ];
15: 
16: #[test]
17: fn init_update_with_tools_all_removes_obsolete_specialist_orchestrator_assets() {
18:     assert_specialist_cleanup(&["--update"]);
19: }
20: 
21: #[test]
22: fn init_force_with_tools_all_removes_obsolete_specialist_orchestrator_assets() {
23:     assert_specialist_cleanup(&["--force"]);
24: }
25: 
26: fn assert_specialist_cleanup(extra_args: &[&str]) {
27:     let base = fixtures::make_empty_repo();
28:     let repo = tempfile::tempdir().expect("work");
29:     let home = tempfile::tempdir().expect("home");
30:     let rust_path = assert_cmd::cargo::cargo_bin!("ito");
31: 
32:     fixtures::reset_repo(repo.path(), base.path());
33: 
34:     let obsolete = obsolete_specialist_asset_paths();
35:     for rel in &obsolete {
36:         fixtures::write(repo.path().join(rel), "obsolete specialist asset\n");
37:     }
38: 
39:     let repo_path = repo.path().to_string_lossy();
40:     let mut argv = vec!["init", repo_path.as_ref(), "--tools", "all"];
41:     argv.extend_from_slice(extra_args);
42:     let out = run_rust_candidate(rust_path, &argv, repo.path(), home.path());
43:     assert_eq!(out.code, 0, "stderr={}", out.stderr);
44: 
45:     for rel in &obsolete {
46:         assert!(
47:             !repo.path().join(rel).exists(),
48:             "expected obsolete specialist asset {rel} to be removed"
49:         );
50:     }
51: 
52:     for rel in installed_specialist_asset_paths() {
53:         assert!(repo.path().join(&rel).exists(), "expected {rel} to install");
54:     }
55: 
56:     for rel in COORDINATOR_PATHS {
57:         assert!(
58:             repo.path().join(rel).exists(),
59:             "expected coordinator asset {rel} to remain installed"
60:         );
61:     }
62: }

```
