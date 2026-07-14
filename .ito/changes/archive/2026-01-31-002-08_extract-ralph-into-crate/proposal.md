## Why

Ralph is a distinct subsystem (loop runner + state + prompt construction) that currently lives inside `ito-core`. Extracting it into its own crate lets us iterate on Ralph features with clearer boundaries and fewer unintended interactions with the core library.

## What Changes

- Create a new Rust crate (workspace member) for Ralph.
- Move Ralph code (`runner`, `state`, `prompt`) out of `ito-core` into that new crate.
- Update `ito-cli` to depend on and call the new Ralph crate.
- Move/adjust tests so Ralph behavior remains covered.
- Keep CLI behavior stable (`ito ralph` continues to work the same).

## Capabilities

### New Capabilities

- `ralph-crate`: Ralph logic is packaged as an independent crate with a stable internal API.

### Modified Capabilities

<!-- None (behavior-preserving refactor) -->

## Impact

- Workspace structure: adds a new crate under `ito-rs/crates/`.
- Dependencies: `ito-cli` will add a dependency on the Ralph crate.
- Internal API: code moves from `ito_core::ralph` to a dedicated crate (call sites updated).
- Tests: Ralph tests move out of `ito-core` into the new crate (or are adjusted accordingly).
