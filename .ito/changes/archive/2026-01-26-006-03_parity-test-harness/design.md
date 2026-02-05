## Context

Parity tests will be the main gate for the Rust port. The harness must be deterministic, runnable in CI, and able to test interactive flows.

## Goals / Non-Goals

**Goals:**

- Provide reusable helpers to run TS and Rust CLIs.
- Provide snapshot-friendly parity assertions.
- Provide PTY helpers for interactive flows.

**Non-Goals:**

- Implement command ports beyond a minimal smoke set for harness validation.

## Decisions

### Decision: Oracle execution uses the existing repo TypeScript CLI

Use the checked-in `ito` implementation as the oracle, invoked via bun/node as appropriate.

### Decision: Parity assertions are explicit

Tests compare: exit code, stdout, stderr. Filesystem diffs are opt-in per test.

## Risks / Trade-offs

- Output instability (timestamps, nondeterministic ordering) -> mitigate by fixing TS outputs if needed or normalizing only when TS already does.
