## Context

These foundations are reused by all later command ports and parity tests. They must be deterministic and isolate side effects.

## Goals / Non-Goals

**Goals:**

- Implement shared parsing/config/env utilities.
- Match TS behavior for global flags and output controls.
- Provide unit tests + parity tests for foundations.

**Non-Goals:**

- Port high-level commands beyond what tests require.

## Decisions

### Decision: Foundations live in `ito-core`

`ito-cli` only wires clap and delegates to `ito-core`.

### Decision: Rendering is deterministic

JSON keys and ordering match the TypeScript output as observed by parity tests.

## Risks / Trade-offs

- Hidden behavior in TS (env vars, cwd resolution) -> mitigate by parity tests across fixture repos.
