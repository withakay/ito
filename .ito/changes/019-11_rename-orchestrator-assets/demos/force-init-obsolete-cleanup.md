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
