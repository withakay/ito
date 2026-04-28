---
children_hash: 674f8ea33efbe5a607b857d5f0b24cc4b69680bf796f6b38511d1257287dbc5e
compression_ratio: 0.4017017465293327
condensation_order: 1
covers: [audit_mirror_concurrency_and_temp_naming.md, context.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 2233
summary_level: d1
token_count: 897
type: summary
---
# ito_workflow

## Overview
This topic covers how Ito publishes, validates, and safely synchronizes coordination-backed state into a read-only `docs/ito` mirror. The three child entries form a related set: `published_ito_mirror.md` defines the mirror generation and path-safety model, `worktree_validation_flow.md` defines read-only worktree validation behavior, and `audit_mirror_concurrency_and_temp_naming.md` defines the audit mirror’s concurrency, merge, and retry protections.

## Key Structure and Relationships
- **Source of truth vs. published output**
  - Coordination-backed Ito state remains the writable source of truth.
  - `docs/ito` is generated as a read-only mirror for consumption in plain GitHub/main checkouts.
  - `published_ito_mirror.md` documents this relationship and the drift-replacement flow.
- **Worktree safety and validation**
  - `worktree_validation_flow.md` splits unsafe main/control checkout cases from advisory mismatches elsewhere.
  - Validation emits machine-readable status for OpenCode pre-tool hooks.
  - Change matching uses exact change-id prefixes to avoid false positives, including suffix worktrees like `<change>-review`.
- **Audit mirror synchronization**
  - `audit_mirror_concurrency_and_temp_naming.md` documents `mirror.rs` behavior for syncing audit JSONL into an internal branch.
  - It uses unique temp worktree and orphan branch names, JSONL deduplication, bounded retention, and conflict retries.

## Child Entry Drill-Down

### `published_ito_mirror.md`
Focuses on the published mirror implementation:
- Safe path resolution for `changes.published_mirror.path`
- Default mirror path: `docs/ito`
- Read-only output layout under `README.md`, `changes/active`, `changes/archive`, and `specs`
- Symlink skipping during generation
- Drift detection by comparing generated output to the existing mirror
- Replacement behavior driven by the `ito publish` CLI

### `worktree_validation_flow.md`
Focuses on read-only validation for change worktrees:
- Command: `ito worktree validate --change <id> [--json]`
- Hard-fails main/control checkouts
- Returns advisory mismatch guidance outside main
- Produces machine-readable status for hooks
- Uses exact change-id prefix matching to avoid substring-based false positives

### `audit_mirror_concurrency_and_temp_naming.md`
Focuses on audit mirror concurrency and Git conflict handling:
- Temp worktree naming pattern: `ito-audit-mirror-{pid}-{nanos}-{sequence}`
- Orphan branch naming pattern: `ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}`
- Atomic sequence counter prevents collisions when multiple processes share the same timestamp
- JSONL merge dedupes identical lines, preserves order, and collapses adjacent reconciled events by count
- Retention policy truncates logs to 30 days from newest event and caps at 1000 events
- Retry policy:
  - internal branch appends retry once on conflict
  - pushes retry after non-fast-forward by refetching and merging again
- Only runs inside a Git worktree; missing remote branches fall back to an orphan branch

## Shared Design Patterns
- **Safety first**: path validation, worktree checks, and conflict detection prevent destructive writes.
- **Read-only output generation**: published artifacts are derived, not edited directly.
- **Deterministic reconciliation**: drift detection and JSONL deduplication preserve stable outputs.
- **Bounded growth**: retention limits prevent audit mirror accumulation from growing without limit.
- **Retry with constraints**: conflicts are retried once, then surfaced as failures or retryable results.