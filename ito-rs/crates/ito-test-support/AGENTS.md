# ito-test-support — Support Crate (dev-dep only)

Reusable test infrastructure: mock repos, PTY helpers, output normalization, snapshot utilities. **Not for production code.**
See [`ito-rs/AGENTS.md`](../../AGENTS.md). See [`.ito/architecture.md`](../../../.ito/architecture.md).

## Key Exports
|mock_repos: in-memory Change/Module/TaskRepository mocks; helpers: make_change, make_module, make_tasks_result
|pty: PTY helpers for driving interactive CLI commands in tests
|CmdOutput: captured output with normalized() method
|rust_candidate_command()/run_rust_candidate(): deterministic binary execution with env vars
|normalize_text(): strip ANSI, normalize newlines, replace HOME paths
|collect_file_bytes(): all file bytes under root for snapshot comparison
|reset_dir(), copy_dir_all(): test directory management

## Dependencies
|ito-domain (for mock trait impls)

## Constraints
**MUST NOT:** be used in production code | appear in [dependencies] (only [dev-dependencies]) | contain business logic/domain models
**MUST:** keep mocks in sync when ito-domain repo traits change | provide deterministic+reproducible output

## Quality
```bash
make test   # exercised transitively by all integration tests
```
|rust-test-engineer: adding/modifying test infra |rust-quality-checker: mocks in sync with trait defs
