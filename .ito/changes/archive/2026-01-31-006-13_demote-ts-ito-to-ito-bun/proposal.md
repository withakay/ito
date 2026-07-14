## Why

Ito currently has two overlapping implementations (TypeScript/Bun and Rust), which creates ongoing confusion about what is supported, how to install it, and which behavior is canonical. We want to make `ito-rs` the clearly supported default going forward while keeping the TypeScript version available only as a deprecated legacy implementation.

## What Changes

- Move the TypeScript implementation out of the repository root by relocating the current `src/` tree into `ito-bun/` (e.g., `ito-bun/src/`) and update all build/test/config references to match.
- Mark the TypeScript/Bun implementation as deprecated in docs and instructions; explicitly state that `ito-rs` is the supported version and must be favored.
- Update references that assume the root TypeScript layout, including `AGENTS.md` at the repo root and any `ito-bun/`-scoped agent/docs content.
- Update `Makefile` targets to prefer the Rust workflow as the default developer path.
- Update install behavior so `ito-rs` is installed as `ito` (not `ito.rs`).
- Uninstall the TypeScript `ito` from the global cache so it no longer shadows/conflicts with the Rust `ito`.
- **BREAKING**: Any direct references to root `src/` (imports, scripts, paths) will need to be updated to the new `ito-bun/` location.
- **BREAKING**: Default installation expectations shift to Rust; the TypeScript version is no longer the primary installed `ito`.

## Capabilities

### New Capabilities

<!-- None; this change primarily modifies packaging/installer requirements and project layout. -->

### Modified Capabilities

- `rust-packaging-transition`: Update the transition policy so the supported `ito` command maps to `ito-rs`, with the TypeScript/Bun implementation treated as deprecated legacy.
- `rust-installers`: Update installer requirements to install `ito-rs` as `ito` by default and to remove/avoid global-cache conflicts with the legacy TypeScript `ito`.

## Impact

- Repository layout and path references (root `src/` move to `ito-bun/`).
- Documentation and agent guidance (`AGENTS.md`, `.ito/AGENTS.md`, plus any `ito-bun/` docs).
- Developer tooling (`Makefile`, CI scripts, package/workspace configs).
- Installation and caching behavior (default `ito` becomes `ito-rs`; legacy TypeScript version removed from global cache).
