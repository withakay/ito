## Why

When backend mode is enabled, the backend becomes the system of record for change artifacts and lifecycle state. Archiving must therefore (1) mark the change archived on the backend and (2) materialize an immutable copy of the archived change into the git repo so project history remains recoverable even if the backend is lost.

## What Changes

- Add backend-aware `ito archive` behavior that marks a change archived on the backend.
- Ensure `ito archive` in backend mode pulls the final backend artifact bundle into the local repo and archives it into `.ito/changes/archive/...`.
- Ensure archived changes become immutable in backend mode workflows (no further writes/leases).
- Add guidance/output so users are prompted to commit the archived change and updated specs.

## Capabilities

### New Capabilities

- `backend-archive-sync`: Backend-aware archive orchestration that preserves an immutable archived copy in git.

### Modified Capabilities

- `cli-archive`: Add backend-mode archive behavior that syncs and marks archived on backend.

## Impact

- **Affected workflows**: `ito archive` now has a backend-aware path when backend mode is enabled.
- **Recovery**: Archived changes and resulting spec updates are guaranteed to exist in the repo, enabling recovery if backend storage is unavailable.
- **Backend state**: Archived status becomes a first-class backend lifecycle signal.
