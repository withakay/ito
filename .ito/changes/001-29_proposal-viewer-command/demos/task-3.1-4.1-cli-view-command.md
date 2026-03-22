# Tasks 3.1-4.1: CLI proposal view command

*2026-03-22T13:19:53Z by Showboat 0.6.1*
<!-- showboat-id: 47fa5e30-2df1-468a-b871-a836f7baf76c -->

Added the `ito view proposal` command with `--viewer`, config-aware tmux filtering, and focused integration tests for help and error paths.

The initial demo command used backticks in the title and triggered shell interpolation, so the bad entry was removed and replaced with direct file output below.

```bash
cargo test -p ito-cli view_proposal
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.11s
     Running unittests src/main.rs (target/debug/deps/ito-25b8a04516db52a3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 57 filtered out; finished in 0.00s

     Running tests/agent_instruction_bootstrap.rs (target/debug/deps/agent_instruction_bootstrap-99cb77953e118958)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s

     Running tests/agent_instruction_context.rs (target/debug/deps/agent_instruction_context-71e6b2dc0a109439)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/agent_instruction_worktrees.rs (target/debug/deps/agent_instruction_worktrees-ab6cc1319b9457e8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/aliases.rs (target/debug/deps/aliases-c724e71e246a8b11)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/archive_completed.rs (target/debug/deps/archive_completed-ade0a1e320e080d8)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/archive_remote_mode.rs (target/debug/deps/archive_remote_mode-98cd909142d9778d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/archive_smoke.rs (target/debug/deps/archive_smoke-807b0bdd13d68c54)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/audit_more.rs (target/debug/deps/audit_more-84aa55f20f106374)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_remote_mode.rs (target/debug/deps/audit_remote_mode-8f6d83c155639872)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_import.rs (target/debug/deps/backend_import-f42b776c04905e69)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/backend_qa_walkthrough.rs (target/debug/deps/backend_qa_walkthrough-110859814d15898c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_serve.rs (target/debug/deps/backend_serve-ae6dd1caf876377d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/backend_status_more.rs (target/debug/deps/backend_status_more-d2305c13214bf022)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 20 filtered out; finished in 0.00s

     Running tests/cli_smoke.rs (target/debug/deps/cli_smoke-e3758a86ff526486)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/cli_snapshots.rs (target/debug/deps/cli_snapshots-727ea997060c77d7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s

     Running tests/config_more.rs (target/debug/deps/config_more-ad5aafc9d460c6a7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/coverage_smoke.rs (target/debug/deps/coverage_smoke-37b7ec66f7a66843)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/create_more.rs (target/debug/deps/create_more-d425df4eb295c904)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/grep_more.rs (target/debug/deps/grep_more-fb63b70443cb4789)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/help.rs (target/debug/deps/help-b1df3102bdb70b48)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/init_gitignore_session_json.rs (target/debug/deps/init_gitignore_session_json-75a1631c8915f688)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/init_more.rs (target/debug/deps/init_more-03d4a569dbbdbf78)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 27 filtered out; finished in 0.00s

     Running tests/init_tmux.rs (target/debug/deps/init_tmux-003cf5ab846cb072)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/init_upgrade_more.rs (target/debug/deps/init_upgrade_more-3a687954976241f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/instructions_more.rs (target/debug/deps/instructions_more-f9edd9d16271b4e6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/list_regression.rs (target/debug/deps/list_regression-7ce4a4d8171dfbf5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/misc_more.rs (target/debug/deps/misc_more-8a9b5900759a3c76)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s

     Running tests/new_more.rs (target/debug/deps/new_more-b08b321f5d1f3e2a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/parity_help_version.rs (target/debug/deps/parity_help_version-3db120eaaa3d345c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/parity_tasks.rs (target/debug/deps/parity_tasks-cc111a21bca30156)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/path_more.rs (target/debug/deps/path_more-a42d14556a1f436e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/plan_state_more.rs (target/debug/deps/plan_state_more-1f41006710c8d11a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph_smoke.rs (target/debug/deps/ralph_smoke-c2c0ddd9feff8059)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/serve_more.rs (target/debug/deps/serve_more-40fe324fd52a52dc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show_specs_bundle.rs (target/debug/deps/show_specs_bundle-78b4581e056124e0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/show_specs_remote_mode.rs (target/debug/deps/show_specs_remote_mode-5a2c059cbb42b86e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/source_file_size.rs (target/debug/deps/source_file_size-e10d33c2efb31310)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-f3e43bfe176a3cdc)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_more.rs (target/debug/deps/tasks_more-1f565f99b7bda0ad)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/tasks_remote_mode.rs (target/debug/deps/tasks_remote_mode-798f5be90b93b1f4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_schemas_export.rs (target/debug/deps/templates_schemas_export-cbaee1136c239f7d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/update_smoke.rs (target/debug/deps/update_smoke-f4021c316622c2fa)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/user_guidance_injection.rs (target/debug/deps/user_guidance_injection-6c56f4a11c357f7b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/validate_more.rs (target/debug/deps/validate_more-36131285b1bf359b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/view_proposal.rs (target/debug/deps/view_proposal-6dfc4ceb5747b104)

running 3 tests
test view_proposal_help_shows_viewer_flag ... ok
test view_proposal_unknown_change_fails ... ok
test view_proposal_disabled_tmux_is_rejected ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s

```

```python
from pathlib import Path
for rel in ['ito-rs/crates/ito-cli/src/commands/view.rs', 'ito-rs/crates/ito-cli/tests/view_proposal.rs']:
    print(f'== {rel} ==')
    path = Path(rel)
    for idx, line in enumerate(path.read_text().splitlines(), start=1):
        if idx > 80:
            break
        print(f'{idx}: {line}')
    print()
```

```output
== ito-rs/crates/ito-cli/src/commands/view.rs ==
1: use crate::cli::{ViewArgs, ViewCommand, ViewProposalArgs};
2: use crate::cli_error::{CliResult, fail, to_cli_error};
3: use crate::runtime::Runtime;
4: use dialoguer::{Select, theme::ColorfulTheme};
5: use ito_config::load_cascading_project_config;
6: use ito_core::viewer::{ViewerBackend, ViewerRegistry, collect_proposal_artifacts};
7:
8: pub(crate) fn handle_view_clap(rt: &Runtime, args: &ViewArgs) -> CliResult<()> {
9:     let Some(command) = &args.command else {
10:         return fail("Missing required subcommand");
11:     };
12:
13:     match command {
14:         ViewCommand::Proposal(args) => handle_view_proposal(rt, args),
15:     }
16: }
17:
18: fn handle_view_proposal(rt: &Runtime, args: &ViewProposalArgs) -> CliResult<()> {
19:     let runtime = rt.repository_runtime().map_err(to_cli_error)?;
20:     let change_repo = runtime.repositories().changes.as_ref();
21:     let resolved_change = crate::app::common::resolve_change_target(change_repo, &args.change_id)
22:         .map_err(crate::cli_error::CliError::msg)?;
23:     let content = collect_proposal_artifacts(&resolved_change, rt.ito_path()).map_err(to_cli_error)?;
24:
25:     let project_root = rt.ito_path().parent().unwrap_or(rt.ito_path());
26:     let merged = load_cascading_project_config(project_root, rt.ito_path(), rt.ctx());
27:     let tmux_enabled = merged
28:         .merged
29:         .pointer("/tools/tmux/enabled")
30:         .and_then(|value| value.as_bool())
31:         .unwrap_or(true);
32:
33:     let registry = ViewerRegistry::for_proposals(tmux_enabled);
34:     let viewer = match &args.viewer {
35:         Some(name) => resolve_named_viewer(&registry, name)?,
36:         None => prompt_for_viewer(&registry)?,
37:     };
38:
39:     viewer.open(&content).map_err(to_cli_error)
40: }
41:
42: fn resolve_named_viewer<'a>(
43:     registry: &'a ViewerRegistry,
44:     name: &str,
45: ) -> CliResult<&'a dyn ViewerBackend> {
46:     let Some(viewer) = registry.find_by_name(name) else {
47:         return fail(format!("Unknown viewer '{name}'"));
48:     };
49:     if !registry.is_enabled(viewer.name()) {
50:         return fail("tmux is disabled in config (tools.tmux.enabled = false). Run 'ito init' to update this preference.");
51:     }
52:     if !viewer.is_available() {
53:         return fail(format!("Viewer '{name}' is unavailable. Install its backing tool and try again."));
54:     }
55:     Ok(viewer)
56: }
57:
58: fn prompt_for_viewer(registry: &ViewerRegistry) -> CliResult<&dyn ViewerBackend> {
59:     let available = registry.available_viewers();
60:     if available.is_empty() {
61:         return fail(
62:             "No proposal viewers are available. Install one of: bat, glow, tmux+nvim.",
63:         );
64:     }
65:
66:     let items: Vec<String> = available
67:         .iter()
68:         .map(|viewer| format!("{} - {}", viewer.name(), viewer.description()))
69:         .collect();
70:     let selection = Select::with_theme(&ColorfulTheme::default())
71:         .with_prompt("Choose a proposal viewer")
72:         .items(&items)
73:         .default(0)
74:         .interact()
75:         .map_err(to_cli_error)?;
76:     Ok(available[selection])
77: }

== ito-rs/crates/ito-cli/tests/view_proposal.rs ==
1: mod support;
2:
3: use assert_cmd::Command;
4: use support::write;
5:
6: #[test]
7: fn view_proposal_help_shows_viewer_flag() {
8:     let mut command = Command::cargo_bin("ito").unwrap();
9:     command.args(["view", "proposal", "--help"]);
10:
11:     command
12:         .assert()
13:         .success()
14:         .stdout(predicates::str::contains("--viewer <VIEWER>"))
15:         .stdout(predicates::str::contains("Change id (directory name)"));
16: }
17:
18: #[test]
19: fn view_proposal_unknown_change_fails() {
20:     let repo = tempfile::tempdir().expect("repo");
21:     write(repo.path().join("README.md"), "# temp\n");
22:     std::fs::create_dir_all(repo.path().join(".ito/changes")).unwrap();
23:
24:     let mut command = Command::cargo_bin("ito").unwrap();
25:     command.current_dir(repo.path());
26:     command.args(["view", "proposal", "001-99_missing", "--viewer", "bat"]);
27:
28:     command
29:         .assert()
30:         .failure()
31:         .stderr(predicates::str::contains("Change '001-99_missing' not found"));
32: }
33:
34: #[test]
35: fn view_proposal_disabled_tmux_is_rejected() {
36:     let repo = tempfile::tempdir().expect("repo");
37:     write(repo.path().join("README.md"), "# temp\n");
38:     write(
39:         repo.path().join(".ito/config.json"),
40:         r#"{"tools":{"tmux":{"enabled":false}}}"#,
41:     );
42:     write(
43:         repo.path().join(".ito/changes/001-29_demo/proposal.md"),
44:         "## Why\nDemo\n",
45:     );
46:
47:     let mut command = Command::cargo_bin("ito").unwrap();
48:     command.current_dir(repo.path());
49:     command.args(["view", "proposal", "001-29_demo", "--viewer", "tmux-nvim"]);
50:
51:     command
52:         .assert()
53:         .failure()
54:         .stderr(predicates::str::contains(
55:             "tmux is disabled in config (tools.tmux.enabled = false)",
56:         ));
57: }

```
