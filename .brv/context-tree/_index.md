---
children_hash: 39f6c03f84aa6c2efab1db029891b0b6f8b1d71759aa3032436d0f98c365c0dd
compression_ratio: 0.8739365815931941
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 2586
summary_level: d3
token_count: 2260
type: summary
---
# Development — Structural Overview

This level groups the core operational knowledge for Ito across **template standardization**, **workflow/mirror management**, **release guardrails**, and **source-guide usage**. The shared themes are: preserve coordination-backed truth, generate deterministic mirrors/artifacts, validate unsafe states early, and use nearby guides as orientation during apply work.

## 1) `ito_templates/_index.md` — Template bundle retrofit

- Standardized plain markdown assets under `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` / `<!-- ITO:END -->`.
- **Already pre-marked files were preserved**; only unmarked plain markdown was retrofitted.
- Verification confirmed no plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so adapter samples were left untouched.
- Drill down:
  - `template_bundle_retrofit.md`
  - `template_bundle_retrofit.abstract.md`
  - `template_bundle_retrofit.overview.md`

## 2) `ito_workflow/_index.md` — Coordination-backed sync, mirror publication, validation, cleanup

### Core architecture
- `context.md` establishes the main split:
  - **writable source of truth** = coordination-backed state
  - **read-only output** = published mirror in `docs/ito`
- Key concerns include safe project-relative path resolution, drift detection, and mirror generation.

### Publication and mirror management
- `published_ito_mirror.md`
  - Default mirror path is `docs/ito` via `changes.published_mirror.path`.
  - Unsafe paths are rejected: empty, absolute, parent-traversal, and project-root-only.
  - Generated tree is deterministic and read-only, centered on:
    - `README.md`
    - `changes/active`
    - `changes/archive`
    - `specs`
  - Publish flow loads cascading config, compares generated output to the current mirror, detects drift, and replaces the mirror from coordination state.
  - Symlinks are skipped during generation.

- `audit_mirror_concurrency_and_temp_naming.md`
  - Sync uses unique temp worktree/orphan branch names derived from:
    - process ID
    - `SystemTime`
    - atomic counter
  - JSONL merge behavior:
    - dedupe identical lines
    - preserve order
    - collapse adjacent reconciled events by incrementing count
  - Retention policy: **30 days** or **1000 events**
  - Retry once on:
    - non-fast-forward push conflicts
    - ref update conflicts
  - Sync flow: detect worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push/update ref → retry on conflict.

### Worktree synchronization and repair
- `coordination_symlink_repair_and_sync.md`
  - Repairs missing `.ito/` links and broken symlinks whose targets were removed.
  - Empty generated `.ito/` directories may be replaced safely.
  - Explicit failures:
    - wrong symlink targets
    - non-empty duplicate `.ito/` directories
  - Symlinks are wired **before** health checks.
  - Missing origin/remote config becomes non-fatal `RateLimited` after local repair.
  - Responsibilities include symlink creation/repair/teardown, worktree provisioning, auto-commit, sync-state persistence, and fetch/fast-forward orchestration.
  - Sync sequence: provision/init → resolve worktree path → create/reuse worktree → wire `.ito` symlinks → update `.gitignore` → health check → fetch → fast-forward → rate-limit check → auto-commit → push → persist sync state.

### Validation rules
- `worktree_validation_flow.md`
  - `ito worktree validate --change <id> [--json]` emits machine-readable status.
  - Main/control checkouts are **hard failures**.
  - Mismatches outside main are advisory and include recovery guidance.
  - Matching uses exact change-id prefixes, including suffix worktrees like `<change>-review`.
  - Designed for OpenCode pre-tool hooks to block only unsafe scenarios.

### Installer and legacy cleanup
- `obsolete_specialist_cleanup.md`
  - Cleanup runs on update flows and forceful reinstall/init paths.
  - Installer performs a pre-pass to remove legacy assets before writing new ones.
  - Broken legacy symlinks are removed via `symlink_metadata`.
  - Legacy assets renamed from `ito-orchestrator-*` to `ito-*`.
  - Coordinator assets such as `ito-orchestrator.md` and `ito-orchestrator-workflow` are preserved.
  - Plain init leaves user files untouched.

### Cross-entry relationships
- `context.md` and `published_ito_mirror.md` define the source-of-truth vs mirror split.
- `coordination_symlink_repair_and_sync.md` and `audit_mirror_concurrency_and_temp_naming.md` cover sync reliability and concurrency.
- `worktree_validation_flow.md` provides safety checks.
- `obsolete_specialist_cleanup.md` covers migration and cleanup.

## 3) `release_workflow/_index.md` — Release pipeline and guardrails

### Main release pipeline
- `release_workflow.md`
  - Sequence:
    - merge release PR
    - `release-plz` publishes crates and tags `vX.Y.Z`
    - `cargo-dist` builds artifacts and creates GitHub Releases
    - Homebrew formulas update in `withakay/homebrew-ito`
    - release notes are polished afterward
  - Role split:
    - `release-plz` = versioning/publishing
    - `cargo-dist` = artifact builds, GitHub Releases, Homebrew publishing
  - Key config files:
    - `.github/workflows/release-plz.yml`
    - `.github/workflows/v-release.yml`
    - `.github/workflows/polish-release-notes.yml`
    - `dist-workspace.toml`
    - `release-plz.toml`
  - Important rule: do **not** set `git_only = true` in `release-plz.toml`; it can miscalculate repository paths during diff/worktree operations.

### Build and coverage guardrails
- `build_and_coverage_guardrails.md`
  - `Makefile` coverage target derives `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset.
  - `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files; only regressions or new violations fail.
  - `cargo-deny` allows `wit-bindgen@0.51` as a wasip3 transitive duplicate.
  - Workflow shape: `make check` → coverage target resolves LLVM vars → `cargo-llvm-cov` runs → max-lines check compares against baseline → `cargo-deny` accepts the duplicate.
  - Key pattern: `^wit-bindgen@0.51$`

### Release-plz coordination and gitignore rules
- `release_plz_guardrails.md`
  - `.ito` coordination paths must remain gitignored.
  - Already tracked ignored files should be removed with `git rm --cached`.
  - `release-plz.toml` stays at repository root for repo discovery in temp clones.
  - GitHub Actions runs release-plz on `main` with separate `release` and `release-pr` jobs.
  - Important settings:
    - `allow_dirty = false`
    - `publish_allow_dirty = false`
    - workspace changelog updates enabled
    - workspace dependency updates enabled
    - changelog config uses `cliff.toml`
    - `ito-cli` is the only package with git tags enabled
  - Protected paths:
    - `^.ito/(changes|specs|modules|workflows|audit)$`
  - Rules:
    - keep `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, `.ito/audit` gitignored
    - if tracked ignored files appear under `.ito/changes`, untrack them with `git rm --cached`
    - do not unignore `.ito/changes`
    - do not set `git_only = true`

### Manifesto instruction rendering and sync status
- `manifesto_instruction_implementation_notes.md`
  - `synced_at_generation` is only set when coordination sync returns `Synchronized`.
  - `RateLimited` does **not** count as fresh success.
  - full `--operation` requires `--change`.
  - embedded operation instructions are scoped to the resolved change state.
  - unconfigured operations render as `null`.

### Cross-entry relationships
- `release_workflow.md` is the parent overview.
- `build_and_coverage_guardrails.md` and `release_plz_guardrails.md` constrain execution and repo state.
- `manifesto_instruction_implementation_notes.md` defines rendering and sync-report semantics.

## 4) `source_guides/_index.md` — Source guide workflow

- Source guides function as the **code map / code atlas** during apply work.
- Coverage spans:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md`
- `source-guide.json` tracks freshness.
- Operational rules:
  - inspect nearby guides before implementing apply changes
  - refresh missing or stale guides when needed
  - treat guides as orientation aids, not final authority
  - verify claims against source
  - update affected guides after structural edits
- Drill down:
  - `source_guide_workflow.md` for hierarchy, freshness tracking, and verification rules

## Shared structural patterns across the development domain

- Prefer **safe, deterministic generation** over implicit mutation.
- Treat **coordination state as authoritative**; published artifacts are derived mirrors.
- Fail loudly on ambiguous or wrong-target filesystem state.
- Retry only clearly retryable Git races.
- Preserve machine-readable status and exact-prefix matching where automation depends on correctness.
- Use guides and summaries for orientation, but verify behavior in the underlying source.