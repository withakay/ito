## Overview

This change removes TS-oracle parity testing from the Rust test suite.

## Design

- Delete parity tests that invoke node/bun or the TS oracle.
- Delete TS-oracle execution helpers from `ito-test-support` if they are no longer used.
- Extract duplicated non-TS test logic (tree collection, normalization, repo resets) into `ito-test-support`.

## What NOT to Change

- Do not change runtime CLI behavior.

## Testing Strategy

- `cargo test --workspace` passes without node/bun.
