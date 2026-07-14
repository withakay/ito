## Context

These commands operate on workflow YAML/state and are sensitive to ordering and schema compatibility. Rust must read and write state compatible with existing `.ito/` data.

## Goals / Non-Goals

**Goals:**

- Implement plan/tasks/workflow/state commands.
- Match TS output and on-disk schema.
- Provide deterministic ordering for snapshots.

**Non-Goals:**

- Introduce a new workflow format.

## Decisions

### Decision: Schema types live in `ito-schemas`

Use `serde` models in `ito-schemas` and share them across crates.

### Decision: Sorting for stable output

When TS behavior yields stable ordering, match it; otherwise define canonical sorting and normalize TS output in parity tests if required.

## Testing Strategy

- Unit tests: schema parsing/serialization
- Integration tests: command behavior
- Parity tests: compare TS vs Rust outputs and state files
