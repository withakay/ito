# ito-common — Layer 0 (Foundation)

Shared utilities used across Ito crates. This is a **leaf crate** — it has zero workspace dependencies and zero domain knowledge.

For workspace-wide guidance see [`ito-rs/AGENTS.md`](../../AGENTS.md). For architectural context see [`.ito/architecture.md`](../../../.ito/architecture.md).

## Purpose

Foundational building blocks reused across the workspace: filesystem abstraction, ID parsing/validation, path builders, fuzzy matching, and I/O wrappers.

## Modules

| Module | Responsibility |
|---|---|
| `fs` | `FileSystem` trait abstraction for testable I/O |
| `id` | Parsing and validation of Ito identifiers (change, module, spec IDs) |
| `io` | Convenience wrappers around common file I/O operations |
| `match_` | Simple fuzzy matching utilities |
| `paths` | Canonical `.ito/` path builders (`changes_dir`, `specs_dir`, etc.) |

## Architectural Constraints

**This crate is the most restricted in the workspace. Violations here cascade everywhere.**

### MUST NOT

- Depend on **any** other workspace crate (leaf crate — enforced by `make arch-guardrails`)
- Contain domain-specific behaviour or domain models (no `Change`, `Module`, `Task` types)
- Contain business logic or policy decisions
- Depend on `ito-domain`, `ito-core`, `ito-cli`, or `ito-web`

### MUST

- Remain small and boring — pure utility code only
- Keep `#![warn(missing_docs)]` enabled
- Provide the `FileSystem` trait that `ito-domain` depends on for testability

## When Adding Code Here

Ask: "Is this utility genuinely reusable across multiple crates, with no domain knowledge required?" If yes, it belongs here. If it needs to know about Changes, Modules, Tasks, or Specs, it belongs in `ito-domain` or `ito-core`.

## Quality Checks

After modifying this crate, run:

```bash
make check              # fmt + clippy
make test               # all workspace tests (changes here can break anything)
make arch-guardrails    # verify no forbidden dependencies crept in
```

Use the `rust-quality-checker` subagent to verify style compliance and the `rust-code-reviewer` subagent to catch architectural drift.
