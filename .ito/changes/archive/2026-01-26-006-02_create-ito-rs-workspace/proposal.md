## Why

We need a Rust workspace with clear crate boundaries and baseline tooling so subsequent command ports can iterate quickly with high test coverage and consistent formatting/linting.

## What Changes

- Create a Cargo workspace at `ito-rs/`.
- Add the initial crate layout (CLI + focused libraries) and baseline dependencies.
- Wire formatting, clippy, tests, and coverage measurement.

## Capabilities

### New Capabilities

- `rust-workspace`: Cargo workspace and crate scaffolding for the Rust port.

### Modified Capabilities

<!-- None. New workspace. -->

## Impact

- Adds `ito-rs/` workspace and crates.
- Adds developer commands for fmt/clippy/tests/coverage.
