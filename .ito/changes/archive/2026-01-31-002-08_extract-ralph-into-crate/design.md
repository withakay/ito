## Context

Ralph currently ships as a module inside `ito-core` (`ito-core/src/ralph/*`) and is invoked by `ito-cli` to implement `ito ralph`. This couples the Ralph loop implementation to core concerns and makes future Ralph-specific work riskier and noisier.

## Goals / Non-Goals

**Goals:**

- Extract Ralph into a dedicated crate (workspace member).
- Preserve CLI behavior and on-disk state layout.
- Keep clear dependency direction (avoid cyclic crate dependencies).

**Non-Goals:**

- Feature work on Ralph itself (this change is refactor-only).
- Changing the Ralph state file format or path.

## Decisions

### Decision: New crate `ito-ralph`

Create `ito-rs/crates/ito-ralph/` (crate name `ito-ralph`, Rust path `ito_ralph`) containing:

- `runner` (the main loop)
- `state` (context + state read/write)
- `prompt` (prompt composition)

### Decision: Dependency direction

`ito-ralph` depends on `ito-core` for shared utilities already used today (e.g. `io`, `paths`, `validate` helpers). `ito-core` does not depend on `ito-ralph`.

This avoids cyclic dependencies and keeps core independent.

### Decision: Preserve CLI and state behavior

- `ito ralph` command remains in `ito-cli`.
- `.ito/.state/ralph/<change-id>/` remains the state directory layout.

## Risks / Trade-offs

- Internal API churn: references to `ito_core::ralph` must be updated.
- Dependency hygiene: `ito-ralph` must not pull in CLI-only concerns.
- Workspace complexity: more crates requires disciplined boundaries.

## Migration Plan

1. Create `ito-ralph` crate and move the Ralph source files.
1. Update imports and public surface (`RalphOptions`, `run_ralph`, state helpers).
1. Update `ito-cli` to use the new crate.
1. Move Ralph tests into `ito-ralph` and ensure `make test` passes.

## Open Questions

- Should we keep a temporary compatibility shim (e.g. a deprecated re-export) for `ito_core::ralph`, or treat it as internal and update all call sites immediately?
