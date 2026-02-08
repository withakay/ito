# ito-domain — Layer 1 (Domain)

Domain models and repository ports for Ito. This crate defines the stable "shape" of Ito data and the interfaces for accessing it. **Domain purity is the single most important architectural constraint in the workspace.**

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Define domain models (`Change`, `Module`, `Task`, `Audit`, `Workflow`, `Planning`, `Schemas`) and repository traits (ports) that higher layers implement. Consumers should use repository APIs, never direct file I/O.

## Modules

| Module | Responsibility |
|---|---|
| `changes` | Change model, computed status (`Draft`/`Ready`/`InProgress`/`Paused`/`Complete`), `ChangeRepository` trait |
| `modules` | Module model, `ModuleRepository` trait, dependency graph |
| `tasks` | Task model, parsing, computation, `TaskRepository` trait |
| `audit` | Audit event types and pure functions |
| `discovery` | Project discovery and filesystem traversal |
| `errors` | `DomainError` (Io, NotFound, AmbiguousTarget) |
| `planning` | Planning primitives and execution plan construction |
| `schemas` | Serde schema types for workflows, plans, and execution state |
| `workflow` | Workflow models and execution helpers |

## Workspace Dependencies

- `ito-common` only

## Architectural Constraints

**These constraints are enforced by `arch_guardrails.py` with baseline counts. Violations will fail CI.**

### MUST NOT

- Depend on `ito-core`, `ito-cli`, or `ito-web`
- Use `miette::` anywhere — error reporting belongs in adapter layers
- Use `std::fs` in production code — use the `FileSystem` trait from `ito-common` for all file operations
- Use `std::process::Command` — the domain must not spawn processes
- Contain presentation logic (formatting, colours, terminal output)
- Perform I/O directly — all I/O must go through trait abstractions

### MUST

- Remain deterministic and side-effect-free in production code paths
- Keep `#![warn(missing_docs)]` enabled
- Own all error types for the domain layer (`DomainError`)
- Define repository traits as ports — implementations live in `ito-core`

## Common Mistakes to Watch For

1. **Adding `std::fs` calls** — always use the `FileSystem` trait instead. The guardrails enforce a baseline count; any increase will fail CI.
2. **Using `miette` for errors** — use `thiserror` to derive `DomainError`. Only adapter layers (CLI, web) convert to `miette`.
3. **Adding a dependency on `ito-core`** — if you need core functionality here, the design is wrong. Domain defines the interfaces; core implements them.
4. **Putting repository implementations here** — only traits (ports) belong in domain. Filesystem-backed implementations go in `ito-core`.

## Quality Checks

```bash
make check              # fmt + clippy
make test               # all workspace tests
make arch-guardrails    # CRITICAL — verifies domain purity bans
```

Use the `rust-quality-checker` subagent to verify style compliance. Use the `rust-code-reviewer` subagent after any change — domain purity violations are the highest-severity architectural issue in this project.
