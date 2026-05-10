## Context

We are porting an existing CLI from TypeScript/Bun to Rust. We need a workspace that keeps side effects testable and isolates command wiring from domain logic.

## Goals / Non-Goals

**Goals:**

- Establish `ito-rs/` workspace and crates.
- Ensure fmt/clippy/test/coverage are wired.

**Non-Goals:**

- Port any `ito` commands beyond minimal scaffolding.

## Decisions

### Decision: Thin CLI crate + domain libraries

`ito-cli` owns clap parsing and calls into `ito-core` and helper crates.

### Decision: Side effects behind traits

Filesystem and process execution are abstracted so the core logic is unit-testable.

## Risks / Trade-offs

- Over-splitting crates too early -> mitigate by keeping crates minimal until needed.
