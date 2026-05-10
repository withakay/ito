## Why

The current JSONL audit stream lives under `.ito/.state/` on the working branch, which creates high-volume churn in normal commits and pollutes repository history with machine-written validation state. We need audit storage to move off user-facing branches: backend mode should write to the backend only, and local mode should persist into an internal Ito repository/branch instead of the branch developers are editing.

## What Changes

- Add audit storage routing so backend mode writes audit events only to the backend/server-side project store.
- Replace tracked working-branch audit JSONL writes in local mode with writes to a dedicated internal Ito branch/repository when available.
- Keep validation, reconciliation, and audit-reading workflows working against the routed audit storage instead of assuming `.ito/.state/audit/events.jsonl` on the current branch.
- Define fallback behavior for environments where an internal branch/repository cannot be used, without writing tracked JSONL files onto normal working branches.
- Update migration, docs, and operational guidance so large audit streams no longer clog ordinary commits.

## Capabilities

### New Capabilities

- `audit-storage-routing`: Route audit persistence and reads to the correct storage backend for local/internal-branch mode vs server-backed mode.

### Modified Capabilities

- `audit-remote-mirroring`: Reframe the internal branch as the primary durable location for local audit history rather than an optional mirror of working-branch JSONL.
- `backend-event-ingest`: Make backend mode server-side audit storage authoritative so local tracked audit files are not required.
- `execution-logs`: Clarify the separation between user-scoped execution telemetry and repository-scoped audit history.

## Impact

- **Affected code**: audit writer/reader/reconcile/validate paths, backend event ingest, git/internal-branch plumbing, and CLI audit commands.
- **Affected systems**: local repository audit persistence, backend project audit storage, validation/reconciliation flows, and worktree/internal branch handling.
- **Operational impact**: normal working branches stop accumulating large audit JSONL history, while backend mode stores audit events only in backend-managed state.
