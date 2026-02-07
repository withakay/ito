# Move Ralph Command Into Commands

## Why

The `ito-cli` crate currently implements `ito ralph` in `ito-cli/src/app/ralph.rs`, while most CLI subcommands live under `ito-cli/src/commands/`.

This change makes the source layout more consistent and predictable, improving discoverability and reducing "where does this command live?" friction.

## What

- Move the `ito ralph` command handler from `ito-rs/crates/ito-cli/src/app/ralph.rs` to `ito-rs/crates/ito-cli/src/commands/ralph.rs`.
- Update module declarations, imports, and call sites to match the new location.
- Keep behavior the same (no CLI flags or output changes).

## Impact

- Refactor-only; expected to be low risk.
- No user-visible behavior changes.

## Verification

- `make check`
- `make test`
