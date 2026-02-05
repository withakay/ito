## Why

Consistent ID parsing, ito directory discovery, config/env handling, and output controls (JSON/color/interactive) must match TypeScript exactly to avoid downstream drift across all commands.

## What Changes

- Implement foundational behaviors in Rust:
  - ID parsing and normalization
  - ito dir discovery (default `.ito` and overrides)
  - config parsing and environment variable behavior
  - global output controls (`--json`, `--no-color`, `NO_COLOR`, `--no-interactive`)
- Add unit tests and parity tests vs TS.

## Capabilities

### New Capabilities

- `rust-foundations`: ID/config/env foundations for the Rust port.

### Modified Capabilities

<!-- None. New Rust implementation layer. -->

## Impact

- Adds foundational libraries in `ito-rs/crates/ito-core/`.
- Unlocks subsequent command ports.
