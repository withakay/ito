---
children_hash: 09193f01156a8128f70773e64c122a174af30a860ba2dbff96f232f38b389b76
compression_ratio: 0.259581881533101
condensation_order: 1
covers: [audit_mirror_concurrency_and_temp_naming.md, context.md, ito_orchestration_consolidation.md, obsolete_specialist_cleanup.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 3444
summary_level: d1
token_count: 894
type: summary
---
# ito_workflow

Covers how Ito publishes and validates coordination-backed state in read-only workspace/mirror flows, with safety, conflict handling, and installer cleanup around workflow artifacts.

## Core areas

- **Published mirror generation** — see **Published Ito Mirror**
  - `published_mirror.path` config, default `docs/ito`
  - Safe path resolution rejects empty, absolute, parent-traversal, and project-root-only paths
  - Generates a read-only `docs/ito` tree from coordination-backed source of truth
  - Skips symlinks during rendering
  - `ito publish` compares existing output with regenerated content and replaces on drift
  - Mirror layout includes `README.md`, `changes/active`, `changes/archive`, and `specs`

- **Worktree validation** — see **Worktree Validation Flow**
  - `ito worktree validate --change <id> [--json]`
  - Machine-readable status supports OpenCode pre-tool hooks
  - Hard-fails on main/control checkouts
  - Non-main mismatches are advisory and include recovery guidance
  - Uses exact change-id prefixes to avoid false positives, including suffix worktrees like `<change>-review`

- **Audit mirror synchronization** — see **Audit Mirror Concurrency and Temp Naming**
  - Sync flow: detect git worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push or update ref → retry on conflict
  - Temp worktree and orphan branch names include PID + `SystemTime` nanos + atomic counter to prevent collisions
  - JSONL merge dedupes identical lines, preserves order, and collapses adjacent equivalent reconciled events
  - Retention keeps events within 30 days of newest event and caps logs at 1000 events
  - Conflict handling retries once on push/ref conflicts; only runs inside a Git worktree

- **Orchestration consolidation** — see **Ito Orchestration Consolidation**
  - Orchestration proposal folded into existing change `028-02_centralize-instruction-source-of-truth`
  - Introduces `agent-surface-taxonomy` to separate direct entrypoint agents from delegated sub-agents
  - Makes `ito agent instruction orchestrate` the authoritative source for overlapping orchestration and multi-agent skills/prompts
  - Distinguishes entrypoints like `ito-general` and `ito-orchestrator` from delegated roles such as planner, researcher, worker, reviewer, and test-runner

- **Installer cleanup for obsolete specialist assets** — see **Obsolete Specialist Cleanup**
  - Update and forceful init/reinstall paths pre-clean renamed `ito-orchestrator-*` specialist assets
  - Removes files, broken symlinks, and prunes empty legacy directories before writing new assets
  - Cleanup uses `symlink_metadata` so broken symlinks are removed correctly
  - Plain init preserves untouched user files
  - Preserves coordinator assets such as `ito-orchestrator.md` and `ito-orchestrator-workflow`

## Key relationships

- **Published mirror** and **audit mirror** both serialize coordination-backed state into read-only or synchronized Git-facing artifacts.
- **Worktree validation** protects change work while **audit mirror** protects concurrent branch/ref writes.
- **Installer cleanup** is tied to the `ito-orchestrator` rename migration and avoids leaving obsolete specialist surfaces behind.
- **Orchestration consolidation** reduces duplication by centralizing authoritative instruction sources under the orchestrate path.

## Drill-down entries

- `Published Ito Mirror`
- `Worktree Validation Flow`
- `Audit Mirror Concurrency and Temp Naming`
- `Ito Orchestration Consolidation`
- `Obsolete Specialist Cleanup`