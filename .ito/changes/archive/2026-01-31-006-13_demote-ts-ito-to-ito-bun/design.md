## Context

This repository currently contains a TypeScript/Bun implementation rooted at `src/` and an in-progress Rust port under `ito-rs/`. The coexistence of both implementations has created ambiguity around:

- Which implementation is supported and should be installed as `ito`
- Which codebase and docs are authoritative
- How to avoid PATH/global-cache conflicts between multiple `ito` implementations

The requested change demotes the TypeScript/Bun implementation by moving it into a dedicated `ito-bun/` folder, marking it deprecated, and treating `ito-rs` as the supported default moving forward.

## Goals / Non-Goals

**Goals:**

- Make `ito-rs` the supported implementation and the default `ito` command.
- Move the TypeScript implementation out of the root layout into `ito-bun/` and update all path references accordingly.
- Update docs/agent instructions to reflect the new support policy and layout.
- Ensure installation and caching behavior does not allow the legacy TypeScript `ito` to shadow the Rust `ito`.

**Non-Goals:**

- Removing the TypeScript codebase entirely.
- Completing feature parity work between TypeScript and Rust beyond what is required to make `ito-rs` the supported default.
- Reworking the Rust workspace structure under `ito-rs/`.

## Decisions

### Decision: Repository layout becomes multi-implementation

Adopt an explicit split:

- `ito-rs/`: supported implementation
- `ito-bun/`: deprecated legacy implementation (migrated from the current root `src/` tree)

Rationale: Keeps the legacy code accessible while making the supported implementation unambiguous.

Alternatives considered:

- Delete the TypeScript code: too disruptive; removes a working fallback.
- Keep TypeScript at root and add Rust elsewhere: preserves current ambiguity and path conflicts.

### Decision: Command naming prioritizes Rust

Treat `ito` as the Rust CLI. The legacy TypeScript CLI (if still runnable) uses a distinct name and must be labeled deprecated.

Rationale: Aligns with the support direction, avoids shadowing conflicts, and reduces user confusion.

Alternatives considered:

- Keep both claiming `ito` via PATH precedence: leads to hard-to-debug behavior differences.

### Decision: Makefile and developer workflows default to Rust

Update `Makefile` targets so the default developer path builds/tests `ito-rs` first, while keeping legacy TS targets explicitly named (e.g., `bun-*` or `ito-bun-*`).

Rationale: Establishes the supported workflow by default while keeping explicit escape hatches.

### Decision: Installers must de-conflict global cache

Update install flows to install `ito-rs` as `ito`, and proactively remove/disable any cached legacy TypeScript `ito` that could shadow Rust.

Rationale: A deprecated implementation should not be able to silently take precedence.

## Risks / Trade-offs

- \[Path breakage\] Moving `src/` will break imports/scripts → Mitigation: update all path references and CI scripts in the same change; add a minimal smoke build/test for both implementations.
- \[Tooling confusion\] Users may still discover TS docs via search → Mitigation: add explicit deprecation banners and cross-links to `ito-rs`.
- \[Cache ambiguity\] "Global cache" location differs by environment → Mitigation: document exact cache paths supported for cleanup and make cleanup idempotent.
- \[Parity expectations\] Existing specs/tests may assume TS is canonical → Mitigation: update specs to treat Rust outputs as canonical and remove TS byte-for-byte parity requirements.

## Migration Plan

1. Create `ito-bun/` and move the TypeScript codebase from root `src/` (and any coupled config) under it.
1. Update build/test configs and scripts to point to `ito-bun/` (and keep Rust configs under `ito-rs/`).
1. Update documentation and agent instructions to state `ito-rs` is supported and the TS version is deprecated.
1. Update `Makefile` to favor Rust by default and expose explicit legacy targets.
1. Update install logic so `ito-rs` is installed as `ito`, and implement cleanup of legacy cached TypeScript `ito` so it cannot shadow Rust.
1. Add/adjust tests that validate `ito --help/--version` identify the Rust implementation and that installer output is validated without depending on TS byte-for-byte parity.

Rollback strategy: revert the layout move and Makefile/installer changes; keep `ito-bun/` as a non-default folder until the migration is re-attempted.

## Open Questions

- What is the authoritative "global cache" location(s) for the TypeScript `ito` in this project (Ito cache vs OpenCode cache vs package manager cache)?
- Should the legacy implementation expose a stable CLI name (e.g., `ito-bun`) or remain developer-only (invoked via Bun scripts)?
- Do we need a compatibility shim to preserve any TS-only behaviors that users rely on, or is the deprecation notice sufficient?
