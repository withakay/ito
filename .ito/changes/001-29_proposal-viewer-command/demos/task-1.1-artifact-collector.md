# Task 1.1: Proposal artifact collector

*2026-03-22T11:37:43Z by Showboat 0.6.1*
<!-- showboat-id: 3e17ebc1-47ae-42f0-9c34-7c007f86f6e9 -->

Added a new ito-core viewer collector that bundles proposal.md, tasks.md, and sorted spec delta files into one markdown document with section separators.

```bash
cargo test -p ito-core viewer::collector
```

```output
warning: missing documentation for a module
  --> ito-rs/crates/ito-core/src/viewer/mod.rs:3:1
   |
 3 | pub mod collector;
   | ^^^^^^^^^^^^^^^^^
   |
note: the lint level is defined here
  --> ito-rs/crates/ito-core/src/lib.rs:10:9
   |
10 | #![warn(missing_docs)]
   |         ^^^^^^^^^^^^

warning: `ito-core` (lib) generated 1 warning
warning: `ito-core` (lib test) generated 1 warning (1 duplicate)
    Finished `test` profile [optimized + debuginfo] target(s) in 0.07s
     Running unittests src/lib.rs (target/debug/deps/ito_core-719960b1bbcfd818)

running 3 tests
test viewer::collector::tests::collect_proposal_artifacts_errors_for_unknown_change ... ok
test viewer::collector::tests::collect_proposal_artifacts_skips_missing_optional_files ... ok
test viewer::collector::tests::collect_proposal_artifacts_orders_sections_and_preserves_content ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 411 filtered out; finished in 0.00s

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
path = Path('ito-rs/crates/ito-core/src/viewer/collector.rs')
for idx, line in enumerate(path.read_text().splitlines(), start=1):
    if idx > 80:
        break
    print(f'{idx}: {line}')
PY
```

```output
1: use std::path::{Path, PathBuf};
2: 
3: use crate::errors::{CoreError, CoreResult};
4: 
5: /// Collect proposal artifacts for a change into a single markdown document.
6: pub fn collect_proposal_artifacts(change_id: &str, ito_root: &Path) -> CoreResult<String> {
7:     let change_dir = ito_common::paths::change_dir(ito_root, change_id);
8:     if !change_dir.is_dir() {
9:         return Err(CoreError::not_found(format!("Change '{change_id}' not found")));
10:     }
11: 
12:     let mut sections = Vec::new();
13: 
14:     for relative_path in artifact_paths(&change_dir)? {
15:         let absolute_path = change_dir.join(&relative_path);
16:         let content = ito_common::io::read_to_string(&absolute_path).map_err(|e| {
17:             CoreError::io(
18:                 format!("reading proposal artifact {}", absolute_path.display()),
19:                 std::io::Error::other(e),
20:             )
21:         })?;
22:         sections.push(render_section(&relative_path, &content));
23:     }
24: 
25:     Ok(sections.join("\n\n"))
26: }
27: 
28: fn artifact_paths(change_dir: &Path) -> CoreResult<Vec<PathBuf>> {
29:     let mut paths = Vec::new();
30: 
31:     for file_name in ["proposal.md", "tasks.md"] {
32:         let path = change_dir.join(file_name);
33:         if path.is_file() {
34:             paths.push(PathBuf::from(file_name));
35:         }
36:     }
37: 
38:     let specs_dir = change_dir.join("specs");
39:     if specs_dir.is_dir() {
40:         let mut spec_dirs: Vec<_> = std::fs::read_dir(&specs_dir)
41:             .map_err(|e| CoreError::io(format!("reading {}", specs_dir.display()), e))?
42:             .filter_map(Result::ok)
43:             .filter(|entry| entry.path().is_dir())
44:             .collect();
45:         spec_dirs.sort_by_key(|entry| entry.file_name());
46: 
47:         for entry in spec_dirs {
48:             let relative_path = PathBuf::from("specs")
49:                 .join(entry.file_name())
50:                 .join("spec.md");
51:             let absolute_path = change_dir.join(&relative_path);
52:             if absolute_path.is_file() {
53:                 paths.push(relative_path);
54:             }
55:         }
56:     }
57: 
58:     Ok(paths)
59: }
60: 
61: fn render_section(relative_path: &Path, content: &str) -> String {
62:     format!(
63:         "---\n# {}\n\n{}",
64:         relative_path.to_string_lossy(),
65:         content.trim_end()
66:     )
67: }
68: 
69: #[cfg(test)]
70: mod tests {
71:     use super::*;
72:     use tempfile::TempDir;
73: 
74:     #[test]
75:     fn collect_proposal_artifacts_orders_sections_and_preserves_content() {
76:         let temp_dir = TempDir::new().unwrap();
77:         let ito_root = temp_dir.path().join(".ito");
78:         let change_dir = ito_root.join("changes/001-29_test-change");
79:         std::fs::create_dir_all(change_dir.join("specs/auth")).unwrap();
80:         std::fs::create_dir_all(change_dir.join("specs/zebra")).unwrap();
```
