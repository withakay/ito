## Context

These commands create and manipulate the `.ito/` structure and emit human-facing instructions. Output must match TS, including phrasing, formatting, and error messages.

## Goals / Non-Goals

**Goals:**

- Implement workflow commands and match TS behavior.
- Ensure filesystem writes match TS (names, numbering, templates).
- Add parity tests for both output and writes.

**Non-Goals:**

- Implement `plan/tasks/workflow/state` (handled in the next change).

## Decisions

### Decision: Templates are centralized

Emit instruction templates via `ito-templates` so installer/template rendering logic is reused.

### Decision: Numbering logic mirrors TypeScript

Module IDs and change numbering must match TS behavior and collision handling.

## Testing Strategy

- Unit tests: change/module naming and numbering
- Integration tests: `create` writes correct files
- Parity tests: stdout + filesystem writes compare to TS
