---
children_hash: 352ea1afae81a98505fc92e8c98be59f190be6d494e16814f502f8e64e273747
compression_ratio: 0.8876916701201824
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 2413
summary_level: d3
token_count: 2142
type: summary
---
# Structural Summary: Ito Knowledge Index

This level organizes the `development` domain into four major areas: template standardization, workflow safety/governance, release pipeline controls, and source-guide orientation. The entries form a layered operating model around `ito-rs` where documentation, coordination state, and release state are tightly constrained.

## `ito_templates`
**Drill down:** `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`

- The template bundle retrofit standardized markdown markers across `ito-rs/crates/ito-templates/assets`.
- Main decision: add `<!-- ITO:START -->` / `<!-- ITO:END -->` only to plain markdown files; leave already marked files unchanged.
- Verification showed no unmarked plain markdown under `ito-rs/crates/ito-templates/assets/adapters`, so adapter samples were not modified.
- Process pattern: scan assets → add markers to plain markdown → preserve pre-marked files → verify adapter sample status.

## `ito_workflow`
**Drill down:** `published_ito_mirror.md`, `audit_mirror_concurrency_and_temp_naming.md`, `coordination_branch_git_behavior.md`, `coordination_symlink_repair_and_sync.md`, `ito_config_gotcha.md`, `worktree_validation_flow.md`, `obsolete_specialist_cleanup.md`, `ddd_discovery_workflow.md`, `ito_orchestration_consolidation.md`, `pre_push_adversarial_code_review.md`

This domain defines the operational safety model for coordination-backed state, published mirrors, worktree validation, and orchestration governance.

### Published mirror and audit mirror
- `published_ito_mirror.md` defines the public, read-only mirror at `docs/ito`.
  - Configurable via `changes.published_mirror.path` with default `docs/ito`
  - Safe path resolution rejects empty, absolute, and parent-traversal paths
  - Renderer skips symlinks
  - Publish CLI detects drift and replaces the mirror from coordination-backed state
- `audit_mirror_concurrency_and_temp_naming.md` covers audit sync internals.
  - Temp worktree/orphan branch names use `pid + timestamp + atomic counter`
  - JSONL merge deduplicates identical lines and collapses adjacent reconciled events
  - Retention is bounded by age and count
  - Push/ref-update conflicts retry once

### Coordination branch and sync safety
- `coordination_branch_git_behavior.md`
  - Missing remote coordination branches initialize from an empty-tree root commit, not caller HEAD
  - `git commit-tree` initialization must omit `-p`
  - Push refspec uses `<commit-hash>:refs/heads/<branch>`
  - SHA-256 empty-tree hashes are supported with SHA-1 fallback
- `coordination_symlink_repair_and_sync.md`
  - Missing `.ito/` links, correct symlinks with missing targets, and empty generated `.ito/` directories are repairable
  - Wrong targets and non-empty duplicate directories fail explicitly
  - Sync wires symlinks before health checks
  - Missing remote configuration is non-fatal after local repair
- `ito_config_gotcha.md`
  - `ito config` reads/writes global `~/.config/ito/config.json`
  - Normal worktrees should use `changes.coordination_branch.enabled=true`, `name=ito/internal/changes`, `storage=worktree`
  - The coordination worktree itself must use `storage=embedded` to avoid self-symlink validation failures

### Validation and cleanup
- `worktree_validation_flow.md`
  - `ito worktree validate --change <id> [--json]` emits machine-readable status
  - Main/control checkouts are hard failures
  - Mismatches outside main are advisory
  - Exact change-id prefix matching avoids false positives like `<change>-review`
- `obsolete_specialist_cleanup.md`
  - Installer update and forceful init/reinstall paths pre-clean obsolete specialist assets
  - Broken symlinks are removed via `symlink_metadata`
  - Legacy `ito-orchestrator-*` specialist assets are removed while coordinator assets remain

### Workflow and orchestration governance
- `ddd_discovery_workflow.md`
  - Integrates `strategic_ddd_for_coding_agents` as non-normative reference material
  - Includes discovery depth gate, capability boundary checks, model ownership, named-or-provisional context relationships, consistency requirements, optional queries, and boundary-smell probes
  - Rigorous domain-grill is gated and only auto-recommended for high-impact ambiguity or explicit opt-in
- `ito_orchestration_consolidation.md`
  - Records orchestration consolidation into change `028-02_centralize-instruction-source-of-truth`
  - Consolidates overlapping orchestration and multi-agent skills/prompts
  - Introduces `agent-surface-taxonomy`
  - Designates `ito agent instruction orchestrate` as the authoritative source
- `pre_push_adversarial_code_review.md`
  - Adds a pre-push quality gate
  - Before pushing a change branch or opening a PR, run an adversarial code review
  - Block P0/P1 issues; minor low-risk findings may be deferred at agent discretion

### Cross-entry relationships
- The published mirror and audit mirror both depend on coordination-backed state but serve different roles: public read-only docs vs internal audit logging/retention.
- Coordination branch initialization, symlink repair, and worktree validation jointly enforce safe local state before sync/publish actions.
- Config handling splits global settings from repo-local effective configuration, with the coordination worktree explicitly exempted from normal worktree storage rules.
- DDD discovery and orchestration consolidation both preserve capability boundaries, but one governs discovery behavior while the other centralizes instruction ownership.

## `release_workflow`
**Drill down:** `release_workflow.md`, `build_and_coverage_guardrails.md`, `release_plz_guardrails.md`, `manifesto_instruction_implementation_notes.md`

This domain defines the release pipeline and the constraints that keep publishing, coverage, and manifest rendering consistent.

### Main release pipeline
- `release_workflow.md` establishes the end-to-end sequence:
  - merge release PR
  - `release-plz` publishes crates and creates version tags `vX.Y.Z`
  - `cargo-dist` builds artifacts and creates GitHub Releases
  - Homebrew formulas are updated in `withakay/homebrew-ito`
  - release notes are polished afterward
- Responsibility split:
  - `release-plz` handles versioning/publishing
  - `cargo-dist` handles artifact builds, GitHub Releases, and Homebrew publishing

### Build and coverage guardrails
- `build_and_coverage_guardrails.md`
  - `Makefile` coverage target derives `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset
  - `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so only regressions or new violations fail
  - `cargo-deny` allows `wit-bindgen@0.51` as a wasip3 transitive duplicate
  - Workflow shape: `make check` → LLVM vars resolved → `cargo-llvm-cov` runs → max-lines baseline check → `cargo-deny` passes duplicate

### release-plz and repo hygiene
- `release_plz_guardrails.md`
  - `.ito` coordination paths must remain gitignored
  - Tracked ignored files should be removed with `git rm --cached`
  - `release-plz.toml` stays at repository root for repo discovery in temp clones
  - GitHub Actions runs release-plz on `main` with separate `release` and `release-pr` jobs
  - Key settings include `allow_dirty = false`, `publish_allow_dirty = false`, workspace changelog/dependency updates, `cliff.toml`, and `ito-cli` as the only package with git tags enabled
  - Protected paths pattern: `^.ito/(changes|specs|modules|workflows|audit)$`
  - Do not unignore `.ito/changes`
  - Do not set `git_only = true`

### Manifest rendering and sync semantics
- `manifesto_instruction_implementation_notes.md`
  - `synced_at_generation` is only populated when coordination sync returns `Synchronized`
  - `RateLimited` is not fresh success
  - Full `--operation` requires `--change`
  - Embedded operation instructions are scoped to the resolved change state
  - Unconfigured operations render as `null`

## `source_guides`
**Drill down:** `source_guide_workflow.md`

- Source-guide files act as a code map / atlas used during Ito apply work.
- Guide hierarchy:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md`
- Freshness is tracked in `source-guide.json`.
- Workflow: check nearby guides → refresh or create stale/missing guides → read for orientation → verify important claims against source → update guides after structural changes.
- Core rule: guides support navigation and context, but are not the source of truth.