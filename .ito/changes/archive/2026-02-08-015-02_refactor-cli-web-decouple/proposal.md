# Refactor: Decouple `ito-cli` from `ito-web`

## Why

`ito-cli` and `ito-web` are both adapters. A hard dependency between adapters makes layering ambiguous, increases compile/load surface, and blocks running CLI-only builds.

## What

- Remove the hard `ito-cli` -> `ito-web` dependency.
- If CLI functionality needs to invoke web behavior, keep it behind an optional Cargo feature (e.g., `web`) that is enabled by default for end-user builds.
- Ensure `ito-cli` can build without the web adapter (`--no-default-features`).

## Compatibility

- Default builds continue to include existing commands.
- Disabling default features produces a CLI binary without web-related commands.

## Verification

- In `ito-rs/`: `cargo build -p ito-cli --no-default-features`
- In `ito-rs/`: `cargo tree -p ito-cli --no-default-features` (must not include `ito-web`)
