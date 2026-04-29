---
children_hash: 40c4439d9a38c7645aceb442fba084e5ffafbd4174e84f746edf7cd5105a9872
compression_ratio: 0.35574614065180105
condensation_order: 1
covers: [audit_mirror_concurrency_and_temp_naming.md, context.md, obsolete_specialist_cleanup.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 2915
summary_level: d1
token_count: 1037
type: summary
---
# ito_workflow

Covers how Ito publishes, validates, and maintains its coordination-backed workflow assets, with a strong emphasis on safety, drift control, and concurrent-update resilience.

## Core themes

- **Published mirror generation** — `published_ito_mirror.md`
  - Ito generates a **read-only `docs/ito` mirror** from coordination-backed state.
  - The mirror path is configurable via `changes.published_mirror.path` and defaults to `docs/ito`.
  - Path resolution is intentionally strict: it rejects empty paths, absolute paths, parent traversal, and project-root-only paths.
  - The renderer skips symlinks and emits a deterministic output layout under `README.md`, `changes/active`, `changes/archive`, and `specs`.
  - The `ito publish` CLI compares generated output against the existing mirror, detects drift, and replaces the mirror from the coordination source of truth.

- **Audit mirror concurrency and temp naming** — `audit_mirror_concurrency_and_temp_naming.md`
  - Audit mirror sync uses **unique temp worktree and orphan branch names** to avoid collisions under parallel writes.
  - Naming pattern includes **PID + SystemTime timestamp (`nanos`) + atomic sequence counter**:
    - `ito-audit-mirror-{pid}-{nanos}-{sequence}`
    - `ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}`
  - The mirror flow is: detect git worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push/update ref → retry on conflict.
  - JSONL merge behavior dedupes identical lines, preserves order, and collapses adjacent reconciled events by incrementing count.
  - Retention is bounded by **age (30 days from newest event)** and **count (1000 events)**.
  - Conflict handling retries once for push/ref conflicts, with best-effort behavior restricted to Git worktrees only.

- **Worktree validation flow** — `worktree_validation_flow.md`
  - `ito worktree validate --change <id> [--json]` now emits **machine-readable status** for pre-tool hooks.
  - Validation distinguishes:
    - **Hard failures** for main/control checkouts
    - **Advisory mismatches** for non-main cases, with recovery guidance
  - Matching is done on **exact change-id prefixes**, preventing false positives such as suffix worktrees like `<change>-review`.

- **Obsolete specialist cleanup** — `obsolete_specialist_cleanup.md`
  - Installer flows now pre-clean obsolete **ito-orchestrator specialist assets** during **update** and **force reinstall/init** paths.
  - Cleanup is performed as a **harness-level pre-pass** before writing new assets.
  - Broken legacy symlinks are removed using `symlink_metadata`.
  - Removed legacy paths include `ito-orchestrator-planner`, `ito-orchestrator-researcher`, `ito-orchestrator-reviewer`, and `ito-orchestrator-worker` markdown/SKILL files.
  - **Coordinator assets are preserved**, including `ito-orchestrator.md` and `ito-orchestrator-workflow`.
  - Plain init intentionally leaves untouched user files in place.

## Shared structure and relationships

- `context.md` defines the domain-level scope for `ito_workflow`:
  - safe project-relative mirror resolution
  - read-only mirror generation
  - drift detection
  - coordination-backed source of truth
- The subtopics are tightly linked:
  - `published_ito_mirror.md` and `audit_mirror_concurrency_and_temp_naming.md` both center on safe mirror generation and state synchronization.
  - `worktree_validation_flow.md` complements the mirror workflow by guarding change-related operations through read-only validation.
  - `obsolete_specialist_cleanup.md` addresses migration safety during installer/init flows after orchestrator asset renames.

## Cross-cutting patterns

- **Safety first**: strict path validation, read-only published output, and guarded validation of worktrees.
- **Concurrency resilience**: atomic counters plus time-based naming prevent temp resource collisions.
- **Drift/control management**: mirror publishing and validation both rely on explicit reconciliation and machine-readable status.
- **Migration hygiene**: installer cleanup removes obsolete specialist assets while preserving coordinator-level assets.