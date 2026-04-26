# Tasks: 009-04_fix-audit-reconcile-loop-and-prune-audit-mirror

## 1. Audit Reconciliation Semantics

- [x] 1.1 Tombstone stale audit entities when a `reconciled` event has no `to` value.
- [x] 1.2 Recompute drift after `ito audit reconcile --fix` writes compensating events.
- [x] 1.3 Fail `ito audit reconcile --fix` when drift remains after a fix attempt.

## 2. Internal Audit Branch Noise Control

- [x] 2.1 Add optional `count` support to audit events with a default of `1` and omission for single events.
- [x] 2.2 Aggregate adjacent equivalent `reconciled` events in the Git-stored audit log by incrementing `count`.
- [x] 2.3 Retain only audit events within 30 days of the newest event when syncing to Git.
- [x] 2.4 Cap the Git-stored audit log to the newest 1000 events.

## 3. Verification

- [x] 3.1 Add regression tests for stale extra task reconciliation clearing drift.
- [x] 3.2 Add serialization tests for missing/default/aggregated `count` values.
- [x] 3.3 Add merge tests for aggregation, 30-day pruning, and 1000-event truncation.
- [x] 3.4 Run focused audit test suites.
