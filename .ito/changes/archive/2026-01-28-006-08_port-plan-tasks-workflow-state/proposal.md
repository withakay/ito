## Why

Ito's planning and workflow commands (`plan`, `tasks`, `workflow`, `state`) define how changes are executed and tracked. They are also used by automated loops (including Ralph) and must be compatible with existing on-disk formats (YAML/JSON) to avoid breaking user workflows.

## What Changes

- Port commands:
  - `ito plan`
  - `ito tasks`
  - `ito workflow`
  - `ito state`
- Preserve YAML/state compatibility with existing TS outputs and on-disk formats.
- Add parity tests that validate both output and on-disk state.

## Capabilities

### New Capabilities

- `rust-planning-and-state`: Rust implementations of plan/tasks/workflow/state.

### Modified Capabilities

<!-- None. New Rust implementation. -->

## Impact

**Affected areas:**

- `ito-rs/crates/ito-workflow/`, `ito-rs/crates/ito-cli/`

**Behavioral impact:**

- None until Rust becomes default

**Risks:**

- Format incompatibilities; mitigated by golden fixtures and parity tests.
