# Refactor: Change repository ports (domain interface + core FS impl)

## Why

- The repository APIs for changes are fundamental to many commands (list/show/validate), but today the layering is blurry and hard to enforce.
- The current `change-repository` spec references a crate (`ito-workflow`) that does not exist in the Rust workspace.
- We want to make the domain deterministic and testable by removing direct `std::fs` usage from `ito-domain` and pushing concrete I/O behind a core boundary.

## What

- Define a `ChangeRepository` port/interface in `ito-domain`.
- Provide a filesystem-backed implementation in `ito-core` (using the existing filesystem abstraction patterns).
- Update callers to depend on the port/interface rather than reading `.ito/changes/` directly.

## Scope

- Change artifact loading and listing (proposal/design/specs/tasks).
- This change does not refactor module or task repositories (handled by 015-06/015-07).

## Depends on

- 015-01_refactor-arch-guardrails
- 015-04_refactor-tracer-bullet-ito-list (recommended, to lock in behavior)

## Verification

- In `ito-rs/`: `cargo test --workspace`
- `make arch-guardrails`
