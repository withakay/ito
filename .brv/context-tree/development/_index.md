---
children_hash: 1dfded01f1cdcdb430570fd688a7aaf6283675d3009d644ce1064ecab7192aae
compression_ratio: 0.6117677961649592
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 3807
summary_level: d2
token_count: 2329
type: summary
---
# Structural Summary: Ito Knowledge Index

## Overall shape
This level groups three major knowledge areas:
- **`ito_templates`** — template asset retrofit and marker standardization
- **`ito_workflow`** — coordination, mirroring, worktree safety, and workflow governance
- **`release_workflow`** — release pipeline, packaging guardrails, and repo-state constraints
- **`source_guides`** — source-guide atlas workflow used for apply-time orientation

## `ito_templates`
**Primary entry:** `ito_templates/_index.md`

- The template bundle retrofit standardized markdown markers across `ito-rs/crates/ito-templates/assets`.
- Core decision: apply `<!-- ITO:START -->` / `<!-- ITO:END -->` to plain markdown files, while leaving already pre-marked files unchanged.
- Verification showed no unmarked plain markdown under `ito-rs/crates/ito-templates/assets/adapters`, so adapter samples were not modified.
- Process pattern:
  - scan assets
  - add markers to plain markdown
  - preserve pre-marked files
  - verify adapter sample status

**Drill down:** `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`

## `ito_workflow`
**Primary entry:** `ito_workflow/_index.md`

This domain describes the operational safety model around coordination-backed state, published mirrors, and Git worktrees.

### Mirror and published output
- **`published_ito_mirror.md`** defines the read-only public mirror at `docs/ito`.
- Key rules:
  - configurable via `changes.published_mirror.path` with default `docs/ito`
  - safe path resolution rejects empty, absolute, and parent-traversal paths
  - renderer skips symlinks
  - publish CLI detects drift and replaces the mirror from coordination-backed state

- **`audit_mirror_concurrency_and_temp_naming.md`** covers internal audit mirror sync behavior.
  - temp worktree/orphan branch names use `pid + timestamp + atomic counter`
  - JSONL merge dedupes identical lines and collapses adjacent reconciled events
  - retention is bounded by age and count
  - push/ref-update conflicts retry once

### Coordination branches and sync safety
- **`coordination_branch_git_behavior.md`**
  - missing remote coordination branches initialize from an empty-tree root commit, not caller HEAD
  - `git commit-tree` initialization must omit `-p`
  - remote push refspec uses `<commit-hash>:refs/heads/<branch>`
  - SHA-256 empty-tree hashes are supported with SHA-1 fallback

- **`coordination_symlink_repair_and_sync.md`**
  - missing `.ito/` links, correct symlinks with missing targets, and empty generated `.ito/` directories are repairable
  - wrong targets and non-empty duplicate directories fail explicitly
  - sync wires symlinks before health checks
  - missing remote configuration is non-fatal after local repair

- **`ito_config_gotcha.md`**
  - `ito config` reads/writes global `~/.config/ito/config.json`
  - normal worktrees should use `changes.coordination_branch.enabled=true`, `name=ito/internal/changes`, `storage=worktree`
  - the coordination worktree itself must use `storage=embedded` to avoid self-symlink validation failures

### Validation and cleanup
- **`worktree_validation_flow.md`**
  - `ito worktree validate --change <id> [--json]` emits machine-readable status
  - main/control checkouts are hard failures
  - mismatches outside main are advisory
  - exact change-id prefix matching avoids false positives like `<change>-review`

- **`obsolete_specialist_cleanup.md`**
  - installer update and forceful init/reinstall paths pre-clean obsolete specialist assets
  - broken symlinks are removed via `symlink_metadata`
  - legacy `ito-orchestrator-*` specialist assets are removed while coordinator assets remain

### Workflow and orchestration governance
- **`ddd_discovery_workflow.md`**
  - integrates `strategic_ddd_for_coding_agents` as non-normative reference material
  - includes discovery depth gate, capability boundary checks, model ownership, named-or-provisional context relationships, consistency requirements, optional queries, and boundary-smell probes
  - rigorous domain-grill is gated and only auto-recommended for high-impact ambiguity or explicit opt-in

- **`ito_orchestration_consolidation.md`**
  - records orchestration consolidation into change `028-02_centralize-instruction-source-of-truth`
  - consolidates overlapping orchestration and multi-agent skills/prompts
  - introduces `agent-surface-taxonomy`
  - designates `ito agent instruction orchestrate` as the authoritative source

- **`pre_push_adversarial_code_review.md`**
  - adds a pre-push quality gate
  - before pushing a change branch or opening a PR, run an adversarial code review
  - block P0/P1 issues; minor low-risk findings may be deferred at agent discretion

### Cross-cutting relationships
- The **published mirror** and **audit mirror** both depend on coordination-backed state, but serve different outputs: public read-only docs vs internal audit logging/retention.
- **Coordination branch initialization**, **symlink repair**, and **worktree validation** jointly enforce safe local state before sync/publish actions.
- **Config handling** splits global settings from repo-local effective configuration, with the coordination worktree explicitly exempted from normal worktree storage rules.
- **DDD discovery** and **orchestration consolidation** both preserve capability boundaries, but one focuses on discovery behavior while the other centralizes instruction ownership.

**Drill down:**  
`published_ito_mirror.md`, `audit_mirror_concurrency_and_temp_naming.md`, `coordination_branch_git_behavior.md`, `coordination_symlink_repair_and_sync.md`, `ito_config_gotcha.md`, `worktree_validation_flow.md`, `obsolete_specialist_cleanup.md`, `ddd_discovery_workflow.md`, `ito_orchestration_consolidation.md`, `pre_push_adversarial_code_review.md`

## `release_workflow`
**Primary entry:** `release_workflow/_index.md`

This domain defines the release pipeline and the constraints that keep publishing, coverage, and manifest rendering consistent.

### Main release pipeline
- `release_workflow.md` establishes the end-to-end sequence:
  - merge release PR
  - `release-plz` publishes crates and creates version tags `vX.Y.Z`
  - `cargo-dist` builds artifacts and creates GitHub Releases
  - Homebrew formulas are updated in `withakay/homebrew-ito`
  - release notes are polished afterward

- Clear split of responsibilities:
  - `release-plz` handles versioning/publishing
  - `cargo-dist` handles artifact builds, GitHub Releases, and Homebrew publishing

### Build and coverage guardrails
- `build_and_coverage_guardrails.md`
  - `Makefile` coverage target derives `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset
  - `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so only regressions or new violations fail
  - `cargo-deny` allows `wit-bindgen@0.51` as a wasip3 transitive duplicate
  - workflow shape: `make check` → LLVM vars resolved → `cargo-llvm-cov` runs → max-lines baseline check → `cargo-deny` passes duplicate

### release-plz and repo hygiene
- `release_plz_guardrails.md`
  - `.ito` coordination paths must remain gitignored
  - tracked ignored files should be removed with `git rm --cached`
  - `release-plz.toml` stays at repository root for repo discovery in temp clones
  - GitHub Actions runs release-plz on `main` with separate `release` and `release-pr` jobs
  - important settings include `allow_dirty = false`, `publish_allow_dirty = false`, workspace changelog/dependency updates, `cliff.toml`, and `ito-cli` as the only package with git tags enabled
  - protected paths pattern: `^.ito/(changes|specs|modules|workflows|audit)$`
  - do not unignore `.ito/changes`
  - do not set `git_only = true`

### Manifest rendering and sync semantics
- `manifesto_instruction_implementation_notes.md`
  - `synced_at_generation` is only populated when coordination sync returns `Synchronized`
  - `RateLimited` is not fresh success
  - full `--operation` requires `--change`
  - embedded operation instructions are scoped to the resolved change state
  - unconfigured operations render as `null`

### Cross-entry relationships
- `release_workflow.md` is the parent overview.
- `build_and_coverage_guardrails.md` and `release_plz_guardrails.md` constrain execution and repo state.
- `manifesto_instruction_implementation_notes.md` explains sync reporting and instruction rendering in the broader release flow.

**Drill down:**  
`release_workflow.md`, `build_and_coverage_guardrails.md`, `release_plz_guardrails.md`, `manifesto_instruction_implementation_notes.md`

## `source_guides`
**Primary entry:** `source_guides/_index.md`

- `source_guide_workflow.md` defines source-guide files as a code map / atlas used during Ito apply work.
- Guide hierarchy:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md`
- Freshness is tracked in `source-guide.json`.
- Workflow:
  - check nearby guides
  - refresh or create stale/missing guides
  - read for orientation
  - verify important claims against source
  - update guides after structural changes
- Core rule: guides support navigation and context, but are not the source of truth.

**Drill down:** `source_guide_workflow.md`