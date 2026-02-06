# Refactor: Task repository ports (domain interface + core FS impl)

## Why

- Task counting is used widely (notably by `ito list`) and has a dedicated spec (`task-repository`).
- We want to ensure task parsing/counting remains the single source of truth while removing concrete filesystem access from `ito-domain`.

## What

- Define the `TaskRepository` boundary in `ito-domain` as a port/interface.
- Provide a filesystem-backed implementation in `ito-core`.
- Ensure CLI continues to use the repository boundary for task counting (no ad-hoc parsing).

## Scope

- Task counting (`get_task_counts`) and task parsing behavior.
- No changes to list/show output formats beyond what is required to preserve correctness.

## Depends on

- 015-01_refactor-arch-guardrails
- 015-04_refactor-tracer-bullet-ito-list (recommended, because it exercises task counting)

## Verification

- In `ito-rs/`: `cargo test --workspace`
- `make arch-guardrails`
