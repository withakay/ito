# ito-test-support — Support Crate

Test helpers for the Ito workspace. Provides mock repositories, PTY helpers, output normalization, and snapshot utilities. **Not for production code.**

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Provide reusable test infrastructure so integration and snapshot tests across the workspace are consistent, deterministic, and easy to write.

## Key Exports

| Export | Responsibility |
|---|---|
| `mock_repos` | In-memory mock implementations of `ChangeRepository`, `ModuleRepository`, `TaskRepository`; helper constructors (`make_change`, `make_module`, `make_tasks_result`) |
| `pty` | PTY helpers for driving interactive CLI commands in tests |
| `CmdOutput` | Captured command output with `normalized()` method |
| `rust_candidate_command()` | Build a `Command` for the Rust candidate binary |
| `run_rust_candidate()` | Run the binary with deterministic env vars |
| `normalize_text()` | Strip ANSI codes, normalize newlines, replace HOME paths |
| `collect_file_bytes()` | Collect all file bytes under a root for snapshot comparison |
| `reset_dir()`, `copy_dir_all()` | Test directory management |

## Workspace Dependencies

- `ito-domain` — for domain trait implementations in mocks

## Architectural Constraints

### MUST NOT

- Be used in production code paths — **dev-dependency only**
- Appear in any crate's `[dependencies]` section (only `[dev-dependencies]`)
- Contain business logic or domain models of its own

### MUST

- Keep mock implementations up to date when repository traits change in `ito-domain`
- Provide deterministic, reproducible test output (strip ANSI, normalize paths)

## Quality Checks

```bash
make test               # all workspace tests (this crate is exercised transitively)
```

Use the `rust-test-engineer` subagent when adding or modifying test infrastructure. Use the `rust-quality-checker` subagent to ensure mock implementations remain in sync with their trait definitions.
