## Why

After foundations and the view/validate commands, the next layer is the artifact workflow: creating modules/changes and generating instructions/templates. These commands are central to the Ito user workflow and are required to manage the Rust port itself over time.

## What Changes

- Port commands:
  - `ito status`
  - `ito instructions` / `ito agent instruction` equivalents
  - `ito templates`
  - `ito create module`
  - `ito create change`
- Preserve legacy aliases and deprecation warnings where applicable.
- Add parity tests for outputs and filesystem writes.

## Capabilities

### New Capabilities

- `rust-artifact-workflow`: Rust implementations of artifact workflow commands.

### Modified Capabilities

<!-- None. New Rust implementation. -->

## Impact

**Affected areas:**

- `ito-rs/crates/ito-cli/`, `ito-rs/crates/ito-core/`, `ito-rs/crates/ito-templates/`

**Behavioral impact:**

- None until Rust becomes default

**Risks:**

- Instruction content drift; mitigated by snapshot + file-write parity tests.
