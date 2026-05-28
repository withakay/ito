---
children_hash: bf4f13cd4e911926200a311729419add744eee6d445966e9bb4f7410f2d6b16f
compression_ratio: 0.2113789528252981
condensation_order: 1
covers: [audit_mirror_concurrency_and_temp_naming.md, context.md, coordination_branch_git_behavior.md, coordination_symlink_repair_and_sync.md, ddd_discovery_workflow.md, ito_config_gotcha.md, ito_orchestration_consolidation.md, obsolete_specialist_cleanup.md, pre_push_adversarial_code_review.md, published_ito_mirror.md, worktree_validation_flow.md]
covers_token_total: 7716
summary_level: d1
token_count: 1631
type: summary
---
# ito_workflow

## Overview
This domain documents how Ito manages coordination-backed state, mirrors, validation, and workflow safety around Git worktrees and generated docs. The core theme is a read-only published mirror backed by writable coordination state, with strict safeguards for symlinks, branch bootstrapping, worktree validation, and audit mirroring.

## Structural Map

### Mirror and published output
- **published_ito_mirror.md**: Defines the read-only `docs/ito` published mirror.
  - Key facts: configurable via `changes.published_mirror.path` (default `docs/ito`), safe path resolution rejects empty/absolute/parent-traversal paths, renderer skips symlinks, and publish CLI detects drift then replaces the mirror from coordination-backed state.
  - Drill down for: mirror layout, drift detection, and source-of-truth separation.
- **audit_mirror_concurrency_and_temp_naming.md**: Covers audit mirror sync internals.
  - Key facts: temporary worktree and orphan branch names use `pid + timestamp + atomic counter`, JSONL merge dedupes identical lines and collapses adjacent reconciled events, retention is capped by age and count, and push/ref-update conflicts retry once.
  - Drill down for: concurrency protections, Git conflict handling, and log truncation behavior.

### Coordination branches and worktree safety
- **coordination_branch_git_behavior.md**: Documents coordination branch bootstrapping and reservation behavior.
  - Key facts: missing remote branches are initialized from an empty tree root commit, not caller HEAD; `git commit-tree` initialization must omit `-p`; remote push refspec is `<commit-hash>:refs/heads/<branch>`; SHA-256 empty-tree hashes are supported with SHA-1 fallback.
  - Drill down for: branch initialization rules, error classification, and tests.
- **coordination_symlink_repair_and_sync.md**: Covers symlink repair and sync orchestration for coordination worktrees.
  - Key facts: missing `.ito/` links, correct symlinks whose targets are missing, and empty generated `.ito/` directories are treated as repairable; wrong targets and non-empty duplicate directories fail explicitly; sync wires symlinks before health checks and treats missing remote configuration as non-fatal after local repair.
  - Drill down for: `.ito/` wiring, health checks, and sync state persistence.
- **ito_config_gotcha.md**: Clarifies global vs repo-local config behavior.
  - Key facts: `ito config` reads/writes global `~/.config/ito/config.json`; normal worktrees should use `changes.coordination_branch.enabled=true`, `name=ito/internal/changes`, `storage=worktree`; the coordination worktree itself must use `storage=embedded` to avoid self-symlink validation failures.
  - Drill down for: config scope boundaries and storage mode requirements.

### Worktree validation and branch hygiene
- **worktree_validation_flow.md**: Defines read-only worktree validation.
  - Key facts: `ito worktree validate --change <id> [--json]` emits machine-readable status; main/control checkouts are hard failures; mismatches outside main are advisory; matching uses exact change-id prefixes to avoid false positives such as `<change>-review`.
  - Drill down for: pre-tool hook behavior and validation policy.
- **obsolete_specialist_cleanup.md**: Describes installer cleanup for renamed orchestrator assets.
  - Key facts: update and forceful init/reinstall paths pre-clean obsolete specialist assets; broken symlinks are removed via `symlink_metadata`; legacy `ito-orchestrator-*` specialist assets are removed while coordinator assets remain.
  - Drill down for: cleanup triggers, retained assets, and installer path behavior.

### Workflow and orchestration guidance
- **ddd_discovery_workflow.md**: Captures the DDD discovery workflow and its gates.
  - Key facts: integrates `strategic_ddd_for_coding_agents` as non-normative reference material; includes discovery depth gate, capability boundary checks, model ownership, named-or-provisional context relationships, consistency requirements, optional queries, and boundary-smell probes; rigorous domain-grill is gated and auto-recommended only for high-impact ambiguity or explicit opt-in.
  - Drill down for: consensus discovery rules and boundary probes.
- **ito_orchestration_consolidation.md**: Records orchestration consolidation into change `028-02_centralize-instruction-source-of-truth`.
  - Key facts: consolidates overlapping orchestration and multi-agent skills/prompts, introduces agent-surface-taxonomy, and designates `ito agent instruction orchestrate` as the authoritative source.
  - Drill down for: surface taxonomy and source-of-truth decision.
- **pre_push_adversarial_code_review.md**: Adds a pre-push quality gate.
  - Key facts: before pushing a change branch or opening a PR, run an adversarial code review; block P0/P1 issues; minor low-risk findings may be deferred at agent discretion.
  - Drill down for: severity gate and review flow.

## Cross-cutting relationships
- **Published mirror and audit mirror** both depend on coordination-backed state but serve different outputs: `docs/ito` is read-only public output, while audit mirror synchronization handles internal branch logging and retention.
- **Coordination branch initialization**, **symlink repair**, and **worktree validation** jointly enforce safe local state before sync/publish actions proceed.
- **Config handling** is split between global user settings and repo-local effective configuration, with the coordination worktree explicitly exempted from normal worktree storage rules.
- **DDD discovery** and **orchestration consolidation** both preserve Ito capability boundaries, but the former focuses on domain-discovery behavior while the latter centralizes agent instruction ownership.

## Drill-down guide
Use the child entries for detail on:
- mirror safety and publishing: **published_ito_mirror.md**
- audit branch concurrency and retention: **audit_mirror_concurrency_and_temp_naming.md**
- coordination bootstrap and ref behavior: **coordination_branch_git_behavior.md**
- `.ito/` symlink repair and sync: **coordination_symlink_repair_and_sync.md**
- config scope and storage mode: **ito_config_gotcha.md**
- read-only worktree validation: **worktree_validation_flow.md**
- installer cleanup of renamed assets: **obsolete_specialist_cleanup.md**
- domain discovery gates: **ddd_discovery_workflow.md**
- orchestration source-of-truth: **ito_orchestration_consolidation.md**
- pre-push adversarial review: **pre_push_adversarial_code_review.md**