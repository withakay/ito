# Refactor: Module repository ports (domain interface + core FS impl)

## Why

- Module metadata is used for grouping changes and for module-aware listing.
- The current `module-repository` spec references a crate (`ito-workflow`) that does not exist in the Rust workspace.
- We want to keep `ito-domain` deterministic and free of direct `std::fs` usage by moving concrete filesystem access behind `ito-core`.

## What

- Define a `ModuleRepository` port/interface in `ito-domain`.
- Provide a filesystem-backed implementation in `ito-core`.
- Update any callers that read `.ito/modules/` directly to go through the repository boundary.

## Scope

- Module metadata loading and module listings.
- Does not refactor change repositories or task repositories (handled by 015-05/015-07).

## Depends on

- 015-01_refactor-arch-guardrails

## Verification

- In `ito-rs/`: `cargo test --workspace`
- `make arch-guardrails`
