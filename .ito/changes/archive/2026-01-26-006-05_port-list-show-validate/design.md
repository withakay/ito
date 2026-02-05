## Context

These commands define the baseline UX and JSON contracts for users and for the porting workflow itself. They also touch schema validation behavior, which must match the TS implementation.

## Goals / Non-Goals

**Goals:**

- Implement list/show/validate in Rust.
- Match TS output text and `--json` shapes.
- Add parity tests across multiple fixture repos.

**Non-Goals:**

- Implement installers (`init/update`) or interactive commands.

## Decisions

### Decision: Rendering functions are pure and snapshot-tested

Separate data loading from rendering so snapshots remain stable.

### Decision: Validator logic is shared

Where possible, centralize validation logic in `ito-core`.

## Testing Strategy

- Unit tests: core models and validation
- Integration tests: CLI flag plumbing
- Parity tests: compare TS vs Rust stdout/stderr/exit codes/JSON
