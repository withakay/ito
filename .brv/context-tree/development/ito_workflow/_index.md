---
children_hash: 1706ab678c68984a7f4cd8762e1856b14160bdd59f60621a217f117d97c3d32e
compression_ratio: 0.34060670569451834
condensation_order: 1
covers: [audit_mirror_concurrency_and_temp_naming.md, context.md, coordination_symlink_repair_and_sync.md, obsolete_specialist_cleanup.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 3758
summary_level: d1
token_count: 1280
type: summary
---
# ito_workflow

This topic covers how Ito keeps coordination-backed state synchronized, published, validated, and cleaned up across worktrees and mirror outputs. The child entries form a pipeline around safe publication, synchronization repair, validation, and installer cleanup.

## Core architecture

- **`context.md`** establishes the top-level scope: Ito publishes a **read-only mirror** of coordination-backed state into `docs/ito` and keeps it synchronized safely.
- The main source of truth remains **coordination-backed writable state**; published output is generated for read-only consumption in plain GitHub/main checkouts.
- Key concepts across the topic:
  - `published_mirror.path` configuration
  - safe project-relative path resolution
  - drift detection
  - coordination-backed source of truth
  - read-only mirror generation

## Publication and mirror management

### `published_ito_mirror.md`
Defines the published mirror implementation:
- Mirror path defaults to **`docs/ito`** via `changes.published_mirror.path`.
- Path resolution rejects unsafe inputs: empty paths, absolute paths, parent traversal, and project-root-only paths.
- The renderer outputs a deterministic read-only tree, including:
  - `README.md`
  - `changes/active`
  - `changes/archive`
  - `specs`
- The publish CLI loads cascading config, compares generated output with the existing mirror, detects drift, and replaces the mirror from coordination-backed state.
- Symlinks are skipped during generation.

### `audit_mirror_concurrency_and_temp_naming.md`
Covers the audit mirror sync path and its concurrency protections:
- Uses unique temp worktree and orphan branch names built from:
  - process ID
  - `SystemTime` timestamp
  - atomic counter
- JSONL merge behavior:
  - dedupes identical lines
  - preserves order
  - collapses adjacent reconciled events by incrementing count
- Retention policy bounds logs by:
  - age: **30 days**
  - count: **1000 events**
- Conflict handling retries once for:
  - non-fast-forward push conflicts
  - ref update conflicts
- Sync flow: detect git worktree -> create temp worktree -> fetch/checkout branch or orphan -> merge JSONL -> stage/commit -> push or update ref -> retry on conflict

## Worktree synchronization and repair

### `coordination_symlink_repair_and_sync.md`
Describes coordination worktree init/sync behavior:
- Repairs missing `.ito/` links and correct symlinks whose targets were removed.
- Treats empty generated `.ito/` directories as safe to replace.
- Fails explicitly on:
  - wrong symlink targets
  - non-empty duplicate `.ito/` directories
- Symlinks are wired **before** health checks during sync.
- Missing origin/remote configuration is treated as non-fatal `RateLimited` after local repair.
- Coordination worktree responsibilities:
  - symlink creation/repair/teardown
  - worktree provisioning
  - auto-commit
  - sync-state persistence
  - fetch/fast-forward orchestration
- The sync flow is:
  provision/init -> resolve worktree path -> create or reuse worktree -> wire `.ito` symlinks -> update `.gitignore` -> health check -> fetch -> fast-forward -> rate-limit check -> auto-commit -> push -> persist sync state

## Validation rules

### `worktree_validation_flow.md`
Defines dedicated read-only validation for change work:
- `ito worktree validate --change <id> [--json]` now emits machine-readable status.
- Main/control checkouts are **hard failures**.
- Mismatches outside main are **advisory** and include recovery guidance.
- Matching uses exact change-id prefixes, including suffix worktrees like `<change>-review`, avoiding false positives.
- This flow is designed for OpenCode pre-tool hooks to block only unsafe scenarios.

## Installer and template cleanup

### `obsolete_specialist_cleanup.md`
Documents cleanup of renamed orchestrator specialist assets during install and init:
- Cleanup runs on:
  - update flows
  - forceful reinstall/init paths
- The installer performs a pre-pass to remove legacy assets before writing new ones.
- Broken legacy symlinks are removed using `symlink_metadata`.
- Legacy assets renamed from `ito-orchestrator-*` to `ito-*`.
- Coordinator assets such as `ito-orchestrator.md` and `ito-orchestrator-workflow` are intentionally preserved.
- Plain init leaves untouched user files in place.

## Relationships and drill-down map

- **Mirror publication and read-only output**
  - `context.md`
  - `published_ito_mirror.md`
- **Sync reliability and concurrency**
  - `coordination_symlink_repair_and_sync.md`
  - `audit_mirror_concurrency_and_temp_naming.md`
- **Read-only validation safeguards**
  - `worktree_validation_flow.md`
- **Installer migration cleanup**
  - `obsolete_specialist_cleanup.md`

## Shared patterns

- Prefer safe, deterministic generation over implicit mutation.
- Treat coordination state as authoritative; published artifacts are derived mirrors.
- Fail loudly on ambiguous or wrong-target filesystem state.
- Use retry-on-conflict only for clearly retryable Git races.
- Preserve machine-readable status and exact-prefix matching where hooks and automation depend on correctness.