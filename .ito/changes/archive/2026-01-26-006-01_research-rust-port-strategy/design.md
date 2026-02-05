## Context

This change is documentation and strategy only. The Rust implementation must match the TypeScript CLI surface identically, including installed prompt contents and marker-managed file edits.

## Goals / Non-Goals

**Goals:**

- Produce research artifacts required by the Rust port workflow.
- Establish a concrete parity testing strategy and command matrix.

**Non-Goals:**

- Implement any Rust code.

## Decisions

### Decision: TypeScript CLI is the behavior oracle

All Rust behavior is validated by running the existing TypeScript CLI and comparing outputs and side effects.

### Decision: Parity harness precedes command ports

Before porting commands, build a harness capable of comparing stdout/stderr/exit codes and filesystem writes.

## Risks / Trade-offs

- Research drift as the TS CLI evolves -> mitigate by re-running parity matrix generation and keeping docs updated.
