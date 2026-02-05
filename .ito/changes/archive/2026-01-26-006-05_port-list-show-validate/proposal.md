## Why

`ito list`, `ito show`, and `ito validate` are core, mostly-non-mutating commands that exercise ID parsing, specs loading, change/module discovery, and JSON output shapes. Porting them early provides high confidence and unlocks parity tests across multiple fixture repositories.

## What Changes

- Implement Rust versions of:
  - `ito list` (including `--modules`, filtering, `--json`)
  - `ito show` (rendering change/module/spec details; `--json`)
  - `ito validate` (including `--strict`, warning behavior, `--json`)
- Add parity tests vs TypeScript across fixture repos.

## Capabilities

### New Capabilities

- `rust-view-and-validate`: Rust implementations of list/show/validate with identical CLI behavior.

### Modified Capabilities

<!-- None. New Rust implementation. -->

## Impact

**Affected areas:**

- `ito-rs/crates/ito-cli/`, `ito-rs/crates/ito-core/`

**Behavioral impact:**

- None until Rust becomes default

**Risks:**

- JSON shape drift; mitigated by snapshot-based parity tests.
