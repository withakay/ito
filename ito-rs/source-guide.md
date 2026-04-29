# Source Guide: ito-rs

## Responsibility
`ito-rs` contains the Rust implementation of Ito. The top-level workspace `Cargo.toml` is at the repo root; crate code lives under `ito-rs/crates/`.

## Entry Points
- `../Cargo.toml`: workspace definition.
- `crates/ito-cli/src/main.rs`: primary user-facing binary.
- `crates/ito-web/src/main.rs`: standalone web development binary.
- `crates/source-guide.md`: crate responsibility map.

## Design
The workspace follows a layered split: domain types and traits, core application logic, and adapter crates. Shared helpers are isolated so business behavior does not leak into utility crates.

## Flow
1. Adapter crates receive CLI/HTTP/web requests.
2. `ito-config` resolves runtime context.
3. `ito-core` executes use-cases against `ito-domain` repository traits.
4. Templates, logging, and test support are consumed as supporting crates.

## Integration
- `Makefile` targets run workspace-wide checks from the repo root.
- Release automation uses workspace version metadata and cargo-dist/release-plz config outside this folder.

## Gotchas
- Some crates enable `#![warn(missing_docs)]`; public additions usually need documentation.
- Tests live both inside crate `src` modules and under crate `tests/` folders.

## Tests
- Broad: `make check`.
- Targeted: `cargo test -p ito-core`, `cargo test -p ito-cli --test <name>`, etc.
