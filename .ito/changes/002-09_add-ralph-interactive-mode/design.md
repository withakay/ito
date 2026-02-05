## Context

The Rust implementation of Ralph already has an `interactive` flag, but `ito-core` currently returns a placeholder error (`Interactive selection is not yet implemented in Rust`) when `--change` is not provided or when a module contains multiple changes.

We want interactive target selection to live in the CLI UX layer and keep `ito-core` largely free of terminal UI dependencies.

## Goals / Non-Goals

**Goals:**

- Provide a fast interactive picker for selecting one or more changes when `--change` is omitted.
- Support batch execution by running Ralph sequentially across selected changes.
- Preserve existing non-interactive behavior (`--no-interactive` requires an explicit target).
- Provide clear behavior on cancellation.

**Non-Goals:**

- Parallel execution across changes.
- Reordering/priority controls beyond the stable selection list order.
- Changing Ralph iteration commit message format.

## Decisions

- Decision: Implement selection in `ito-cli` using `dialoguer`.
  - Rationale: `ito-cli` already depends on `dialoguer` and uses it for other interactive commands; `ito-core` can remain UI-agnostic.
  - Alternative: add terminal UI to `ito-core`. Rejected to avoid coupling core logic to interactive terminal concerns.

- Decision: Represent selection as a list of change IDs and call `ito_core::ralph::run_ralph` once per change.
  - Rationale: Reuses existing loop/state behavior without inventing a new multi-target runner.
  - Alternative: introduce a new multi-change runner in `ito-core`. Rejected for initial scope.

- Decision: Present changes in a stable order (sorted by change id) and execute in that presented order.
  - Rationale: deterministic behavior and easy to communicate in UX and specs.

## Risks / Trade-offs

- Interactive prompts in CI/headless environments could be confusing.
  - Mitigation: `--no-interactive` remains available and is the documented approach for automation.

- Batch execution across multiple changes may produce interleaved git commits that are harder to attribute.
  - Mitigation: keep execution sequential and print clear per-change headers before each run.

## Migration Plan

- No data migration.
- Update CLI help text and snapshots/tests.

## Open Questions

- Whether to default the interactive picker to only show "active" changes (e.g., not complete) vs listing all non-archived changes.
