---
children_hash: 5a5843ac0d0f507516cab060cd7cf4eec9f5c65677121ed925c5dcffb5d1ffba
compression_ratio: 0.23414918414918415
condensation_order: 1
covers: [audit_mirror_concurrency_and_temp_naming.md, context.md, coordination_branch_bootstrap.md, coordination_symlink_repair_and_sync.md, ddd_discovery_workflow.md, ito_config_gotcha.md, ito_orchestration_consolidation.md, obsolete_specialist_cleanup.md, pre_push_adversarial_code_review.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 8580
summary_level: d1
token_count: 2009
type: summary
---
# development/ito_workflow — Structural Overview

This topic covers Ito’s workflow around coordination-backed state, publishing mirrors, validating worktrees, and maintaining safe Git/worktree behavior. The main structural theme is **coordination state as source of truth**, with read-only published output and guarded sync/bootstrap paths.

## Core workflow architecture

- **context.md** sets the top-level framing:
  - `changes.published_mirror.path` drives where the published mirror is generated.
  - The published mirror is **read-only** and generated from coordination-backed state.
  - Drift detection and safe project-relative path resolution are central to mirror handling.
- The workflow is organized around keeping the writable coordination state separate from consumer-facing outputs.

## Mirror publication and synchronization

### published_ito_mirror.md
- Defines the published mirror as a generated `docs/ito` tree by default.
- Key safety rules:
  - Reject empty paths, absolute paths, parent traversal, and root-only paths.
  - Skip symlinks during generation.
  - Compare generated output against the existing mirror and replace only on drift.
- The `ito publish` CLI is the main reconciliation mechanism.
- Relationship: coordination-backed state remains the writable source of truth; `docs/ito` is the read-only published view.

### audit_mirror_concurrency_and_temp_naming.md
- Describes audit mirror sync internals and concurrency protections.
- Important patterns:
  - Unique temp worktree names and orphan branch names use `pid + timestamp + atomic counter`.
  - JSONL merge dedupes identical lines, preserves order, and collapses adjacent reconciled events.
  - Retention truncates by age and max count.
- Conflict handling:
  - Retry once on push/ref-update conflicts.
  - Uses detached worktrees and falls back to orphan branches when needed.
- Best read-down entry for implementation details of mirror.rs behavior.

## Worktree validation and safety gates

### worktree_validation_flow.md
- Introduces a dedicated read-only validation flow for change worktrees.
- Key rules:
  - `ito worktree validate --change <id> [--json]`
  - Main/control checkouts are hard failures.
  - Non-main mismatches are advisory and include recovery guidance.
  - Matching uses exact change-id prefixes to avoid false positives.
- This flows into machine-readable status output for hooks and pre-tool gating.

### pre_push_adversarial_code_review.md
- Adds a pre-push quality gate before pushing a change branch or opening a PR.
- The workflow is:
  - review diff adversarially
  - block P0/P1 issues
  - optionally address low-risk findings
  - then push/open PR
- This is positioned as a review-noise reduction and defect-catch step before publication.

## Git coordination branch lifecycle

### coordination_branch_bootstrap.md
- Documents bootstrap behavior for missing coordination/origin branches.
- Critical rules:
  - Initialize from a clean empty-tree **root commit**, not from caller HEAD.
  - Do not include parent refs (`-p`) during initialization.
  - Reject empty `git commit-tree` / `mktree` stdout before hashing.
  - Push the created commit as `<oid>:refs/heads/<branch>`.
- Object-format handling is explicit:
  - SHA-256 supported
  - SHA-1 fallback when object-format detection is absent or non-sha256
- The branch setup result distinguishes existing remote branches from newly created ones.
- Consolidates multiple overlapping bootstrap docs into one authoritative source.

### coordination_symlink_repair_and_sync.md
- Defines coordination worktree provisioning, symlink repair, sync ordering, and failure modes.
- Structural split:
  - `coordination.rs`: symlink creation/repair/teardown and health classification
  - `coordination_worktree.rs`: provisioning, auto-commit, sync state persistence, fetch/fast-forward/push orchestration
- Repair policy:
  - Missing links, correct symlinks with missing targets, and empty generated `.ito/` dirs are safe.
  - Wrong symlink targets and non-empty duplicate dirs fail explicitly.
- Sync behavior:
  - wire symlinks before health checks
  - fetch first, then fast-forward, then auto-commit/push
  - missing or unconfigured remote can degrade to `RateLimited` after local repair
- Uses `lexical_normalize` for path comparisons and `CoordinationStorage` to distinguish embedded vs worktree mode.

### ito_config_gotcha.md
- Clarifies the split between global CLI config and repo-local effective config.
- Key paths:
  - global user config: `~/.config/ito/config.json`
  - repo-local config: `.ito/config.json`
- Worktree/storage rules:
  - normal worktrees: `changes.coordination_branch.enabled=true`, `name=ito/internal/changes`, `storage=worktree`
  - coordination worktree: same branch settings but `storage=embedded`
- The coordination worktree at `~/.local/share/ito/withakay/ito` must not validate itself as a self-symlinked worktree.

### coordination_branch_bootstrap.md and coordination_symlink_repair_and_sync.md relation
- Together they define the branch/worktree initialization path:
  - bootstrap missing branch from empty tree
  - provision or repair symlinks
  - validate and sync safely
  - avoid ambiguous or self-referential states

## Workflow consolidation and documentation governance

### ddd_discovery_workflow.md
- Captures the DDD discovery workflow as a curated, gated discovery process.
- Important concepts:
  - discovery depth gate
  - capability boundary checks
  - model ownership over data/code location
  - named-or-provisional context relationships
  - consistency requirements, optional queries, boundary-smell probes
- The “rigorous domain-grill” is **gated**, not unconditional:
  - auto-recommended only for high-impact ambiguity or explicit opt-in
- Treats `strategic_ddd_for_coding_agents.md` as non-normative reference material.

### ito_orchestration_consolidation.md
- Records consolidation of orchestration work into existing change `028-02_centralize-instruction-source-of-truth`.
- Introduces an `agent-surface-taxonomy`:
  - direct entrypoint agents vs delegated role sub-agents
- The authoritative orchestration source is the `ito agent instruction orchestrate` path.
- Prevents duplicated orchestration logic across overlapping skills/prompts.

### obsolete_specialist_cleanup.md
- Documents installer cleanup for obsolete `ito-orchestrator` specialist assets after rename migration.
- Cleanup applies on:
  - update flows
  - forceful reinstall/init paths
- Legacy files and broken symlinks are removed before new assets are written.
- Empty legacy dirs are pruned, but coordinator assets remain excluded and preserved.

## High-level relationships and patterns

- **Source of truth**: coordination-backed state
- **Published consumer view**: read-only `docs/ito`
- **Safety gates**:
  - worktree validation hard-fails unsafe main/control states
  - pre-push adversarial review blocks major issues
  - bootstrap rejects empty stdout and HEAD-based initialization
  - symlink repair refuses ambiguous or wrong-target states
- **Stability patterns**:
  - unique temp naming with pid/timestamp/counter
  - exact prefix matching for change IDs
  - object-format-aware hashing with SHA-256 support
  - explicit conflict retries and bounded mirror retention

## Drill-down map

- Mirror publication and safe read-only output: `published_ito_mirror.md`
- Audit mirror concurrency and retention: `audit_mirror_concurrency_and_temp_naming.md`
- Worktree validation and hook status: `worktree_validation_flow.md`
- Pre-push review gate: `pre_push_adversarial_code_review.md`
- Coordination branch bootstrap rules: `coordination_branch_bootstrap.md`
- Symlink repair and sync orchestration: `coordination_symlink_repair_and_sync.md`
- Config path and storage mode gotcha: `ito_config_gotcha.md`
- DDD discovery gating and boundaries: `ddd_discovery_workflow.md`
- Orchestration consolidation and ownership: `ito_orchestration_consolidation.md`
- Legacy asset cleanup during install/init: `obsolete_specialist_cleanup.md`