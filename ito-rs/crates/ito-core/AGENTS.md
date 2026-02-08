# ito-core — Layer 2 (Core)

Business logic and orchestration. Implements the repository adapters, archive, audit, create, list, show, validate, workflow, harness integrations, and installers. **"Policy heavy, UI light."**

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Define the core semantics of every Ito command without owning the CLI argument surface or output formatting. This is the thickest crate in the workspace — it holds the real logic.

## Key Modules

| Module | Responsibility |
|---|---|
| `change_repository` | Filesystem-backed `ChangeRepository` implementation |
| `module_repository` | Filesystem-backed `ModuleRepository` implementation |
| `task_repository` | Filesystem-backed `TaskRepository` implementation |
| `archive` | Archive completed changes, update specifications |
| `audit` | Audit log infrastructure: writer, reader, reconciliation, validation |
| `config` | JSON configuration file CRUD |
| `create` | Scaffold new modules and changes |
| `errors` | `CoreError` wrapping `DomainError` |
| `harness` | Harness trait and implementations (`OpencodeHarness`, `StubHarness`) |
| `installers` | Install project/home templates and harness assets |
| `list` | List/query project entities |
| `planning_init` | Planning directory initialization |
| `process` | Process execution boundary |
| `ralph` | AI agent loop support |
| `show` | Display and inspection |
| `stats` | Statistics collection |
| `tasks` | Task-focused orchestration use-cases |
| `validate` | Validation of on-disk state, repo integrity |
| `workflow` | Workflow execution and planning |

## Workspace Dependencies

- `ito-common` (Layer 0)
- `ito-config` (Layer 0)
- `ito-domain` (Layer 1) — **required edge**
- `ito-templates` (Layer 1)

## Architectural Constraints

### MUST NOT

- Depend on `ito-cli` or `ito-web` (enforced by `make arch-guardrails`)
- Own CLI argument parsing or output formatting — that belongs in the adapter layer
- Contain presentation logic (colours, terminal output, interactive prompts)
- Carry presentation logic in `CoreError` — adapter layers convert it to `miette` reports

### MUST

- Depend on `ito-domain` (required edge, enforced by guardrails)
- Implement repository traits defined in `ito-domain` (repository adapters live here)
- Keep `#![warn(missing_docs)]` enabled
- Own `CoreError` which wraps `DomainError`

## Common Mistakes to Watch For

1. **Adding CLI-specific formatting** — if you're writing code that decides how output looks, it belongs in `ito-cli`. Core returns structured data; adapters format it.
2. **Depending on `ito-cli`** — if you need to call CLI code from core, the design is inverted. Core should expose functions that the CLI calls.
3. **Bypassing repository abstractions** — don't parse markdown files directly. Use `ChangeRepository`, `ModuleRepository`, or `TaskRepository`.
4. **Adding `dialoguer` or `crossterm`** — interactive prompts are adapter-layer concerns.

## Quality Checks

```bash
make check              # fmt + clippy
make test               # all workspace tests
make arch-guardrails    # verify no forbidden dependencies
```

This is the largest crate. Use the `rust-quality-checker` subagent proactively as you work. Use the `rust-code-reviewer` subagent after completing features — especially to verify that business logic hasn't leaked into presentation concerns or vice versa. Use the `rust-test-engineer` subagent when designing tests for complex orchestration logic.
