## Why

Teams can now run the backend as the system of record, but there is still no supported way to migrate existing local `.ito/changes/` history into backend storage on demand. Without a direct import command, backend adoption is fragile, repeated cutovers are manual, and archived history can be left behind.

## What Changes

- Add `ito backend import` to import active and archived local changes into backend-managed state.
- Add `--dry-run` so users can preview the import scope before making backend changes.
- Make import idempotent and resumable so reruns safely skip or refresh already-imported state instead of duplicating it.
- Preserve imported proposal, design, tasks, spec deltas, and archived lifecycle state so backend reads match local history after import.
- Report a migration summary with imported, skipped, and failed counts plus actionable next steps.

## Capabilities

### New Capabilities

- `backend-import`: Direct local-to-backend import workflow for active and archived Ito changes.

### Modified Capabilities

- `backend-change-sync`: Add import behavior, dry-run preview, and resumable semantics alongside backend export.
- `change-repository`: Ensure backend-backed reads expose imported active and archived changes after migration.

## Impact

- **Affected code**: `ito-cli` backend subcommands, `ito-core` import orchestration and repository adapters, backend storage APIs, and import/integration tests.
- **Affected systems**: backend project store, local change discovery, archived history handling, and backend-mode migration workflows.
- **Operational impact**: backend adoption becomes scriptable and retry-safe without requiring `ito init` as the only migration entrypoint.
