---
children_hash: cffd12e7a20aeca71397157ceb29095539670893a430654292105a4f24ed9525
compression_ratio: 0.22326092933736597
condensation_order: 1
covers: [audit_mirror_concurrency_and_temp_naming.md, context.md, coordination_branch_git_behavior.md, coordination_symlink_repair_and_sync.md, ddd_discovery_workflow.md, ito_config_gotcha.md, ito_orchestration_consolidation.md, obsolete_specialist_cleanup.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 7274
summary_level: d1
token_count: 1624
type: summary
---
# ito_workflow

## Overview
This domain captures the coordination-backed workflow that powers Ito’s publish, sync, validation, and bootstrap behavior. The core theme is maintaining a writable coordination source of truth while safely exposing read-only mirrors, validating worktrees, and handling Git/symlink/config edge cases without corrupting state.

## Major Themes

### Published mirror and read-only output
- **published_ito_mirror.md** documents the generated `docs/ito` mirror as a read-only view of coordination-backed state.
- The mirror uses safe project-relative path resolution, rejects unsafe paths, skips symlinks, detects drift, and replaces stale output from the coordination source of truth.
- Key config entry: `changes.published_mirror.path`, defaulting to `docs/ito`.

### Coordination branch bootstrap and Git behavior
- **coordination_branch_git_behavior.md** defines how the coordination branch is initialized and reserved.
- Missing remote branches are bootstrapped from an empty tree root commit, not from HEAD.
- Push refspec shape is `^<commit-hash>:refs/heads/<branch>$`; empty `commit-tree` / `mktree` stdout must be rejected to avoid accidental delete refspecs.
- Git errors are classified as: `NonFastForward`, `ProtectedBranch`, `RemoteRejected`, `RemoteMissing`, `RemoteNotConfigured`, and `CommandFailed`.
- SHA-256 empty-tree support exists with SHA-1 fallback.

### Coordination symlink repair and sync
- **coordination_symlink_repair_and_sync.md** covers `.ito/` symlink wiring, repair, health checks, and sync flow.
- Safe states: missing links, correct symlinks whose targets are missing, and empty generated `.ito/` directories.
- Unsafe states: wrong symlink targets and non-empty duplicate `.ito/` directories.
- Sync order matters: wire symlinks before health checks, then fetch, fast-forward, auto-commit, push, and persist sync state.
- Missing origin/remote configuration is treated as non-fatal `RateLimited` after local repair.

### Git mirror concurrency and temp naming
- **audit_mirror_concurrency_and_temp_naming.md** describes the audit mirror’s collision-safe temp naming and bounded log handling.
- Temporary worktrees and orphan branches include PID, timestamp, and atomic counter to prevent collisions during parallel operations.
- JSONL merge behavior dedupes identical entries, preserves order, and aggregates adjacent reconciled events by count.
- Retention is bounded by age and count; push and ref-update conflicts retry once.

### Worktree validation
- **worktree_validation_flow.md** defines the dedicated read-only validation command for change work.
- Main/control checkouts hard-fail; non-main mismatches are advisory and include recovery guidance.
- Status output is machine-readable for OpenCode pre-tool hooks.
- Matching uses exact change-id prefixes to avoid false positives, including suffix worktrees like `<change>-review`.

### Config boundaries and storage mode
- **ito_config_gotcha.md** clarifies the split between global config management and repo-local effective config.
- Global CLI config lives in `~/.config/ito/config.json`.
- Normal repo worktrees must use `changes.coordination_branch.enabled=true`, `name=ito/internal/changes`, `storage=worktree`.
- The coordination worktree at `~/.local/share/ito/withakay/ito` must use `storage=embedded` to avoid self-symlink validation failures.

### Orchestration consolidation
- **ito_orchestration_consolidation.md** records consolidation of orchestration behavior into existing change `028-02_centralize-instruction-source-of-truth`.
- It introduces `agent-surface-taxonomy` to distinguish direct entrypoint agents from delegated sub-agents.
- The authoritative orchestration source is the `ito agent instruction orchestrate` path, preventing duplicated logic across agents and skills.

### DDD discovery workflow
- **ddd_discovery_workflow.md** captures the domain discovery workflow and its gating logic.
- It distinguishes business/domain capability from Ito capability, preserves model ownership over data/code location, and includes named-or-provisional context relationships.
- Rigorous “domain-grill” questioning is not unconditional; it is gated and auto-recommended only for high-impact ambiguity or explicit user opt-in.
- Non-normative reference material includes `strategic_ddd_for_coding_agents.md`, plus lazy capture artifacts like `CONTEXT.md`, `CONTEXT-MAP.md`, and ADRs.

### Installer cleanup for obsolete specialist assets
- **obsolete_specialist_cleanup.md** documents cleanup of renamed `ito-orchestrator-*` specialist assets during update and forceful init/reinstall flows.
- Cleanup is triggered on `InstallMode::Update`, `opts.update`, or `opts.force`; plain init preserves untouched user files.
- Broken symlinks are removed via `symlink_metadata`; empty legacy directories are pruned afterward.
- Coordinator assets such as `ito-orchestrator.md` and `ito-orchestrator-workflow` are intentionally preserved.

## Cross-cutting patterns
- **Coordination-backed source of truth:** writable state remains in coordination storage; published outputs are generated mirrors.
- **Safety-first Git behavior:** bootstrap, push, and reservation flows all guard against accidental HEAD leakage, empty refspecs, and conflicting updates.
- **Explicit failure guidance:** wrong symlink targets, unsafe checkouts, and duplicate state are surfaced with actionable remediation.
- **Bounded sync state:** audit and mirror outputs are kept finite through conflict retry logic and retention truncation.
- **Machine-readable operations:** validation and sync systems increasingly expose structured status for hooks and automation.

## Drill-down map
- **published_ito_mirror.md** — mirror generation, drift detection, safe path handling
- **coordination_branch_git_behavior.md** — branch bootstrap, refspecs, error classification
- **coordination_symlink_repair_and_sync.md** — `.ito/` repair, health checks, sync sequencing
- **audit_mirror_concurrency_and_temp_naming.md** — temp naming, merge dedupe, retry behavior
- **worktree_validation_flow.md** — validation statuses, prefix matching, main/control policy
- **ito_config_gotcha.md** — global vs repo-local config, embedded vs worktree storage
- **ito_orchestration_consolidation.md** — orchestration source-of-truth and agent surface taxonomy
- **ddd_discovery_workflow.md** — gated discovery workflow and boundary probes
- **obsolete_specialist_cleanup.md** — installer cleanup for legacy specialist assets