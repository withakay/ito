# Tasks 2.2-2.5: Viewer backends and registry

*2026-03-22T13:02:58Z by Showboat 0.6.1*
<!-- showboat-id: fd283456-64cb-4de1-9284-01012fa1fd1f -->

Added bat, glow, and tmux-nvim viewer backends plus a registry that filters available viewers and supports lookup by stable name.

```bash
cargo test -p ito-core viewer::
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/ito_core-719960b1bbcfd818)

running 6 tests
test viewer::tests::concrete_viewers_report_expected_names ... ok
test viewer::tests::viewer_backend_trait_exposes_required_methods ... ok
test viewer::tests::viewer_registry_filters_and_finds_available_viewers ... ok
test viewer::collector::tests::collect_proposal_artifacts_errors_for_unknown_change ... ok
test viewer::collector::tests::collect_proposal_artifacts_skips_missing_optional_files ... ok
test viewer::collector::tests::collect_proposal_artifacts_orders_sections_and_preserves_content ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 411 filtered out; finished in 0.00s

     Running tests/archive.rs (target/debug/deps/archive-d3a9d1bfd6d907f4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/audit_mirror.rs (target/debug/deps/audit_mirror-007a5e2ed3d4817f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/audit_storage.rs (target/debug/deps/audit_storage-91ed3da2c77c28dd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/backend_archive.rs (target/debug/deps/backend_archive-b9dad70afc462772)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/backend_auth.rs (target/debug/deps/backend_auth-e2d6bdbc8abeca19)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

     Running tests/backend_auth_service.rs (target/debug/deps/backend_auth_service-aae0188ffc0b1e59)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/backend_client_mode.rs (target/debug/deps/backend_client_mode-77bb6bae3bf64b46)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/backend_module_repository.rs (target/debug/deps/backend_module_repository-b255d6f900f72e34)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s

     Running tests/change_repository_lifecycle.rs (target/debug/deps/change_repository_lifecycle-69f65c6646a1902e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/change_repository_parity.rs (target/debug/deps/change_repository_parity-ff55a14e6030c31f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

     Running tests/change_target_resolution_parity.rs (target/debug/deps/change_target_resolution_parity-f82a1e025337d40d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/create.rs (target/debug/deps/create-0199a28dc5adf5c9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/distribution.rs (target/debug/deps/distribution-8c94ad7e63563442)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/event_forwarding.rs (target/debug/deps/event_forwarding-37dab5c4369bc8a2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/grep_scopes.rs (target/debug/deps/grep_scopes-5b13865ba331aac1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/harness_context.rs (target/debug/deps/harness_context-af76bc60705db536)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/harness_opencode.rs (target/debug/deps/harness_opencode-f512cf551e25d2f9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/harness_streaming.rs (target/debug/deps/harness_streaming-2ba933b0521dc87f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/harness_stub.rs (target/debug/deps/harness_stub-a7f00e9b9b9efc4b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/import.rs (target/debug/deps/import-6fe001007edeee4d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

     Running tests/io.rs (target/debug/deps/io-e9eefd5e4045f5f9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/planning_init.rs (target/debug/deps/planning_init-a9a9867c2380fb65)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/ralph.rs (target/debug/deps/ralph-0cb1d906fb238a71)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out; finished in 0.00s

     Running tests/repo_index.rs (target/debug/deps/repo_index-dc94359d93830623)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/repo_integrity.rs (target/debug/deps/repo_integrity-cd9f28e9dcb208ea)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/repo_paths.rs (target/debug/deps/repo_paths-76fb7dee68b31f70)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

     Running tests/repository_runtime.rs (target/debug/deps/repository_runtime-667f35bd3bcddc80)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

     Running tests/repository_runtime_config_validation.rs (target/debug/deps/repository_runtime_config_validation-c28a4856a332077d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/show.rs (target/debug/deps/show-9da22de02c48a865)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s

     Running tests/spec_repository_backends.rs (target/debug/deps/spec_repository_backends-11614744a3d4de86)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/spec_show_repository.rs (target/debug/deps/spec_show_repository-7b27bb5a64b3ff54)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/sqlite_archive_mirror.rs (target/debug/deps/sqlite_archive_mirror-a5f791989cda6134)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/sqlite_task_mutations.rs (target/debug/deps/sqlite_task_mutations-b998e9ee0f03833d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/stats.rs (target/debug/deps/stats-e0e7c13850ca1157)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/task_repository_summary.rs (target/debug/deps/task_repository_summary-9c118b6df5864e42)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/tasks_api.rs (target/debug/deps/tasks_api-607e6c7c0943484a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s

     Running tests/tasks_checkbox_format.rs (target/debug/deps/tasks_checkbox_format-57ea8b4fa038aff9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s

     Running tests/tasks_orchestration.rs (target/debug/deps/tasks_orchestration-e192639b82895b70)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s

     Running tests/templates_apply_instructions.rs (target/debug/deps/templates_apply_instructions-226edf3504a7c6f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_change_status.rs (target/debug/deps/templates_change_status-f9616b1ab04fb3cf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

     Running tests/templates_review_context.rs (target/debug/deps/templates_review_context-35f989c0098496bf)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

     Running tests/templates_schema_resolution.rs (target/debug/deps/templates_schema_resolution-47595fad4489e9f5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/templates_schemas_listing.rs (target/debug/deps/templates_schemas_listing-a3cb4ab82a4bbd5c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s

     Running tests/templates_user_guidance.rs (target/debug/deps/templates_user_guidance-3dbb41abd7f02f46)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

     Running tests/validate.rs (target/debug/deps/validate-e67f0b1e9606dd78)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.00s

```

```bash
python - <<'PY'
from pathlib import Path
for rel in [
    'ito-rs/crates/ito-core/src/viewer/bat.rs',
    'ito-rs/crates/ito-core/src/viewer/glow.rs',
    'ito-rs/crates/ito-core/src/viewer/tmux_nvim.rs',
    'ito-rs/crates/ito-core/src/viewer/registry.rs',
]:
    print(f'== {rel} ==')
    path = Path(rel)
    for idx, line in enumerate(path.read_text().splitlines(), start=1):
        if idx > 40:
            break
        print(f'{idx}: {line}')
    print()
PY
```

```output
== ito-rs/crates/ito-core/src/viewer/bat.rs ==
1: use std::process::{Command, Stdio};
2:
3: use crate::errors::{CoreError, CoreResult};
4:
5: use super::ViewerBackend;
6:
7: /// Render markdown via `bat` with paging.
8: pub struct BatViewer;
9:
10: impl ViewerBackend for BatViewer {
11:     fn name(&self) -> &str {
12:         "bat"
13:     }
14:
15:     fn description(&self) -> &str {
16:         "Render the proposal in the terminal with bat"
17:     }
18:
19:     fn is_available(&self) -> bool {
20:         command_on_path("bat")
21:     }
22:
23:     fn open(&self, content: &str) -> CoreResult<()> {
24:         run_with_stdin(
25:             "bat",
26:             &["--language=markdown", "--paging=always"],
27:             content,
28:         )
29:     }
30: }
31:
32: pub(crate) fn command_on_path(binary: &str) -> bool {
33:     std::env::var_os("PATH").is_some_and(|paths| {
34:         std::env::split_paths(&paths).any(|dir| dir.join(binary).is_file())
35:     })
36: }
37:
38: pub(crate) fn run_with_stdin(binary: &str, args: &[&str], content: &str) -> CoreResult<()> {
39:     if !command_on_path(binary) {
40:         return Err(CoreError::not_found(format!(

== ito-rs/crates/ito-core/src/viewer/glow.rs ==
1: use crate::errors::CoreResult;
2:
3: use super::ViewerBackend;
4: use super::bat::run_with_stdin;
5:
6: /// Render markdown via `glow`.
7: pub struct GlowViewer;
8:
9: impl ViewerBackend for GlowViewer {
10:     fn name(&self) -> &str {
11:         "glow"
12:     }
13:
14:     fn description(&self) -> &str {
15:         "Render the proposal in the terminal with glow"
16:     }
17:
18:     fn is_available(&self) -> bool {
19:         super::bat::command_on_path("glow")
20:     }
21:
22:     fn open(&self, content: &str) -> CoreResult<()> {
23:         run_with_stdin("glow", &["-"], content)
24:     }
25: }

== ito-rs/crates/ito-core/src/viewer/tmux_nvim.rs ==
1: use std::process::Command;
2: use std::time::{SystemTime, UNIX_EPOCH};
3:
4: use crate::errors::{CoreError, CoreResult};
5:
6: use super::ViewerBackend;
7: use super::bat::command_on_path;
8:
9: /// Render markdown inside a tmux popup running Neovim in read-only mode.
10: pub struct TmuxNvimViewer;
11:
12: impl ViewerBackend for TmuxNvimViewer {
13:     fn name(&self) -> &str {
14:         "tmux-nvim"
15:     }
16:
17:     fn description(&self) -> &str {
18:         "Open the proposal in a tmux popup with Neovim"
19:     }
20:
21:     fn is_available(&self) -> bool {
22:         std::env::var_os("TMUX").is_some()
23:             && command_on_path("tmux")
24:             && command_on_path("nvim")
25:     }
26:
27:     fn open(&self, content: &str) -> CoreResult<()> {
28:         if std::env::var_os("TMUX").is_none() {
29:             return Err(CoreError::validation(
30:                 "tmux-nvim viewer requires an active tmux session",
31:             ));
32:         }
33:         if !command_on_path("nvim") {
34:             return Err(CoreError::not_found("nvim is not installed or not on PATH"));
35:         }
36:         if !command_on_path("tmux") {
37:             return Err(CoreError::not_found("tmux is not installed or not on PATH"));
38:         }
39:
40:         let temp_file = temporary_viewer_path();

== ito-rs/crates/ito-core/src/viewer/registry.rs ==
1: use super::ViewerBackend;
2:
3: /// Registry of known proposal viewer backends.
4: pub struct ViewerRegistry {
5:     viewers: Vec<Box<dyn ViewerBackend>>,
6: }
7:
8: impl ViewerRegistry {
9:     /// Create a registry from a fixed set of backends.
10:     pub fn new(viewers: Vec<Box<dyn ViewerBackend>>) -> Self {
11:         Self { viewers }
12:     }
13:
14:     /// Return viewers that are currently runnable.
15:     pub fn available_viewers(&self) -> Vec<&dyn ViewerBackend> {
16:         self.viewers
17:             .iter()
18:             .map(Box::as_ref)
19:             .filter(|viewer| viewer.is_available())
20:             .collect()
21:     }
22:
23:     /// Find a registered viewer by its stable name.
24:     pub fn find_by_name(&self, name: &str) -> Option<&dyn ViewerBackend> {
25:         self.viewers
26:             .iter()
27:             .map(Box::as_ref)
28:             .find(|viewer| viewer.name() == name)
29:     }
30: }

```
