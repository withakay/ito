# Task 2.1: ViewerBackend trait

*2026-03-22T12:56:33Z by Showboat 0.6.1*
<!-- showboat-id: 0f55f8e4-d8d7-4f4f-9718-557ebd57c19f -->

Added a public ViewerBackend trait in ito-core with the required name, description, availability, and open methods for proposal viewers.

```bash
cargo test -p ito-core viewer::
```

```output
    Finished `test` profile [optimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/ito_core-719960b1bbcfd818)

running 4 tests
test viewer::tests::viewer_backend_trait_exposes_required_methods ... ok
test viewer::collector::tests::collect_proposal_artifacts_errors_for_unknown_change ... ok
test viewer::collector::tests::collect_proposal_artifacts_skips_missing_optional_files ... ok
test viewer::collector::tests::collect_proposal_artifacts_orders_sections_and_preserves_content ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 411 filtered out; finished in 0.00s

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
path = Path('ito-rs/crates/ito-core/src/viewer/mod.rs')
for idx, line in enumerate(path.read_text().splitlines(), start=1):
    if idx > 80:
        break
    print(f'{idx}: {line}')
PY
```

```output
1: //! Proposal viewer support.
2: 
3: use crate::errors::CoreResult;
4: 
5: /// Artifact collection helpers for proposal viewing.
6: pub mod collector;
7: 
8: pub use collector::collect_proposal_artifacts;
9: 
10: /// A pluggable backend that can render collected proposal artifacts.
11: pub trait ViewerBackend {
12:     /// Stable CLI/backend identifier.
13:     fn name(&self) -> &str;
14: 
15:     /// Human-readable summary shown in prompts and help.
16:     fn description(&self) -> &str;
17: 
18:     /// Whether the viewer can run in the current environment.
19:     fn is_available(&self) -> bool;
20: 
21:     /// Open or render the provided proposal content.
22:     fn open(&self, content: &str) -> CoreResult<()>;
23: }
24: 
25: #[cfg(test)]
26: mod tests {
27:     use super::*;
28: 
29:     use crate::errors::CoreResult;
30: 
31:     struct DummyViewer;
32: 
33:     impl ViewerBackend for DummyViewer {
34:         fn name(&self) -> &str {
35:             "dummy"
36:         }
37: 
38:         fn description(&self) -> &str {
39:             "Dummy viewer for tests"
40:         }
41: 
42:         fn is_available(&self) -> bool {
43:             true
44:         }
45: 
46:         fn open(&self, _content: &str) -> CoreResult<()> {
47:             Ok(())
48:         }
49:     }
50: 
51:     #[test]
52:     fn viewer_backend_trait_exposes_required_methods() {
53:         let viewer = DummyViewer;
54:         assert_eq!(viewer.name(), "dummy");
55:         assert_eq!(viewer.description(), "Dummy viewer for tests");
56:         assert!(viewer.is_available());
57:         viewer.open("hello").unwrap();
58:     }
59: }
```
