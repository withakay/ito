## Purpose

Ensure Rust tests do not require the legacy TypeScript/Bun implementation at all.

## ADDED Requirements

### Requirement: TS oracle parity tests are removed

Tests that invoke the TS oracle SHALL be removed from the Rust test suite.

#### Scenario: Default test run does not require node/bun

- **WHEN** running `cargo test --workspace`
- **THEN** tests do not attempt to execute node/bun

#### Scenario: No TS oracle feature exists

- **WHEN** inspecting `ito-cli` Cargo features
- **THEN** there is no feature flag intended to enable TS-oracle parity tests

### Requirement: Reusable test helpers live in test support

Shared test helpers for filesystem tree comparisons and normalization SHALL live in `ito-test-support`.

#### Scenario: Tree diff helper reuse

- **GIVEN** multiple tests need to compare directory trees
- **WHEN** implementing the comparison
- **THEN** the logic is implemented once in `ito-test-support` and reused
