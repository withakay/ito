<!-- ITO:START -->
## Why

Ito's audit state can drift during agent-driven sessions unless the harness runs deterministic audit checks. Claude Code supports lifecycle hooks (notably PreToolUse) that can run before tools execute, making it a good place to trigger `ito audit validate` / `ito audit reconcile` and surface drift immediately.

## What Changes

- Extend `ito init` / `ito update` to install Claude Code hook configuration that runs Ito audit checks before tool execution.
- Keep the hook shim minimal: shell script reads hook JSON, calls Ito CLI, returns structured hook output for warnings/blocks.
- Add tests to ensure hook config and scripts install/update deterministically and do not clobber user-owned settings.

## Capabilities

### New Capabilities

- `harness-audit-hooks`: Deterministic, pre-tool audit validation and drift detection for supported harnesses.

### Modified Capabilities

- `tool-adapters`: Add Claude Code PreToolUse audit hook wiring.
- `rust-installers`: Ensure Claude adapter assets are installed/updated consistently.

## Impact

- New or updated files under `.claude/` (hook config + hook script) installed by `ito init` and refreshed by `ito update`.
- Rust installer logic/tests to guarantee idempotent behavior across init/update.
<!-- ITO:END -->
