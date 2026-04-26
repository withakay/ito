<!-- ITO:START -->
## Why

Audit reconciliation could enter a write loop for stale task audit entries: each `--fix` wrote another equivalent `reconciled` event, but the event did not change materialized state. The internal audit branch also retained every repeated event indefinitely, creating noisy Git history and unnecessary CPU and I/O work.

## What Changes

- Treat a `reconciled` audit event without a `to` value as a tombstone for the materialized audit entity, so stale task entries can be cleared.
- Make `ito audit reconcile --fix` re-check drift after writing fixes and return a failure if drift remains, preventing silent auto-fix loops.
- Add an optional audit event `count` field that defaults to `1` and is only serialized for aggregated durable log entries.
- Aggregate adjacent equivalent `reconciled` events in the internal audit branch by incrementing `count` instead of writing duplicate lines that differ only by timestamp/context.
- Truncate the Git-stored audit log to parseable audit events that are no older than 30 days from the newest event and cap the stored log at the newest 1000 events.

## Impact

- **Affected specs**: `audit-remote-mirroring`, `audit-storage-routing`
- **Affected code**: `ito-domain` audit event/materialization, `ito-core` audit reconcile/mirror storage, `ito-cli` audit reconcile command output/failure behavior
- **Risk**: Low to medium. The append-only event stream remains valid, but the internal Git mirror becomes a bounded retention surface rather than an unbounded full-history store.
<!-- ITO:END -->
