## Why

Teams already have meaningful project history in local `.ito/changes/` and `.ito/changes/archive/`, but backend mode currently assumes that state already exists remotely. Without a safe import path, switching to the remote backend risks missing active work context and losing archived traceability.

## What Changes

- Extend `ito init` to let users choose backend mode or local mode during setup.
- When backend mode is selected and local changes exist, require an explicit import decision (`Yes`/`No`) before enabling backend mode.
- If the user declines import, fail backend-mode setup with actionable guidance instead of allowing a partial cutover.
- Add deterministic non-interactive init flags for scripted setup (`--backend`/`--local` and explicit import-policy flags).
- Import both active and archived changes into backend storage with lifecycle fidelity (archived stays archived/immutable).
- After successful, validated import, remove local change artifacts so backend mode has a single source of truth and avoids dual-state confusion.

## Capabilities

### New Capabilities

- (none)

### Modified Capabilities

- `cli-init`: Add backend/local setup selection and mandatory import gating when local changes exist.
- `backend-change-sync`: Add backend import behavior used during backend-mode initialization.
- `change-repository`: Ensure backend-backed reads expose imported active and archived changes as canonical state.

## Impact

- **Affected workflows**: backend cutover is initiated during `ito init` with an import decision gate.
- **Affected code**: `ito-cli` init/backend command handling, `ito-core` backend sync orchestration, backend artifact/lifecycle APIs.
- **Data safety**: archived history remains recoverable after migration because archived changes are imported with immutable status.
- **Operations**: migration becomes retry-safe and deterministic, and local duplicate state is cleaned up after verified import.
