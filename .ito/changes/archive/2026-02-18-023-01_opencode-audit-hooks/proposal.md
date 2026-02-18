<!-- ITO:START -->
## Why

Ito's audit log is only reliable if drift is detected early and often. Today, OpenCode sessions can mutate `.ito/` state (tasks, specs, archive prep) without any deterministic pre-tool auditing, so drift accumulates and reconciliation happens late (or not at all).

## What Changes

- Install an OpenCode plugin that runs Ito audit checks on a pre-tool-use hook and injects audit status/warnings into the session context.
- Keep the plugin thin: delegate all logic to the Ito CLI (`ito audit validate`, `ito audit reconcile`, `ito audit reconcile --fix`) and avoid embedding policy.
- Add deterministic testing around init/update installing and updating the OpenCode plugin files.

## Capabilities

### New Capabilities

- `harness-audit-hooks`: Deterministic, pre-tool audit validation and drift detection for supported harnesses.

### Modified Capabilities

- `tool-adapters`: Add OpenCode-specific hook wiring for audit validation/reconciliation.
- `rust-installers`: Ensure OpenCode adapter assets are installed/updated consistently.

## Impact

- Templates/adapters under `.opencode/` (plugin code + any minimal config) installed by `ito init` / refreshed by `ito update`.
- Rust installers logic and tests (template merge/overwrite behavior and update semantics).
<!-- ITO:END -->
