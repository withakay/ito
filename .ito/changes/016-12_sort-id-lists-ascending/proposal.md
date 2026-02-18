<!-- ITO:START -->
## Why

ID-bearing lists are not consistently ordered across commands and persisted artifacts, which makes output harder to scan and introduces unnecessary merge churn. We need a single ordering contract so module IDs, change IDs, task IDs, and spec IDs are always presented predictably.

## What Changes

- Define and enforce a consistent ordering policy for all list outputs that include IDs: ascending by canonical ID (lower first, higher last), with deterministic tie-breakers where non-ID primary sorts are retained.
- Update CLI list/show/tasks surfaces so human and JSON output follow the same deterministic ordering rules for modules, changes, specs, and tasks.
- Canonicalize `.ito/workflows/.state/change-allocations.json` writes so module keys are stable and sorted to reduce merge conflicts.
- Keep `change-allocations` as JSON snapshot state for now (not JSONL), and document the rationale and constraints in design.
- **BREAKING**: Change the default ordering behavior for `ito list` change output from recency-first to ID-ascending.

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `cli-list`: standardize default and deterministic ordering rules for ID-bearing list output.
- `cli-tasks`: require deterministic ID-ascending ordering for task and change ID lists in status/ready/show outputs.
- `cli-show`: require ID-ascending ordering for interactive and ambiguous selection lists.
- `change-creation`: require canonical sorted serialization for change allocation state and deterministic ordering in module change checklists.

## Impact

- Affected code: `ito-rs/crates/ito-core/src/list.rs`, `ito-rs/crates/ito-core/src/tasks.rs`, `ito-rs/crates/ito-core/src/create/mod.rs`, `ito-rs/crates/ito-cli/src/commands/tasks.rs`, `ito-rs/crates/ito-cli/src/app/show.rs`, and related tests.
- Affected persisted artifact: `.ito/workflows/.state/change-allocations.json` ordering behavior.
- Affected user experience: list command defaults and ordering consistency across CLI and JSON consumers.
<!-- ITO:END -->
