## Context

The TS implementation supports multiple harnesses and writes loop state and history. Rust must implement the same contract and keep tests deterministic by using stub harnesses.

## Goals / Non-Goals

**Goals:**

- Implement `ralph`/`loop` commands and core runner.
- Match prompt assembly, completion promise detection, and state layout.
- Provide harness abstraction and stub harnesses for tests.

**Non-Goals:**

- Actually call networked model APIs in tests.

## Decisions

### Decision: Harness invocation behind a trait

`ito-harness` exposes a trait-based runner; `ito-cli` selects harness implementation. Tests can inject stubs.

### Decision: Deterministic state

Any timestamps/paths stored on disk are normalized or made deterministic in tests.

## Testing Strategy

- Unit tests: promise detection and prompt assembly
- Integration tests: CLI behavior
- Parity tests: compare TS vs Rust with a stub harness
