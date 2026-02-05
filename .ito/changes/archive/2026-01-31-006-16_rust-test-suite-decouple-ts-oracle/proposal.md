## Why

The TypeScript/Bun implementation is deprecated. The Rust test suite currently includes parity tests that shell out to the legacy implementation ("TS oracle"), which creates unnecessary coupling and maintenance burden.

Keeping these tests around:

- makes `cargo test` depend on node/bun being installed (or forces gating complexity)
- introduces flakiness and frequent mismatches
- slows down the Rust development loop

Now that Rust is the supported implementation, we should remove TS-oracle parity tests entirely.

## What Changes

- Remove TS-oracle parity tests from the Rust test suite.
- Remove TS-oracle execution helpers from `ito-test-support` (or keep them only if used elsewhere).
- Ensure `cargo test --workspace` is node/bun-free without needing any feature flags.
- Replace any remaining parity coverage with Rust-native tests (snapshots/fixtures) as needed.

## Capabilities

### New Capabilities

- `rust-remove-ts-oracle-tests`

### Modified Capabilities

(none)

## Impact

- Default Rust CI/dev loops get faster and more reliable.
- Cross-implementation comparisons are no longer part of the Rust test suite.
