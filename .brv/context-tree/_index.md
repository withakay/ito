---
children_hash: dbe2a6e8f24d6d426404af23720eefe46f5ff7f080c27fceee7c89e1c47ad25c
compression_ratio: 0.8460085025980161
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 2117
summary_level: d3
token_count: 1791
type: summary
---
# Development

The development knowledge base focuses on four operational themes: template normalization, Ito workflow safety, release guardrails, and source-guide maintenance. The entries emphasize deterministic structure, validation before mutation, and preserving correct state instead of rewriting it.

## Template bundle retrofit
- **`template_bundle_retrofit.md`** documents a marker standardization pass over `ito-rs/crates/ito-templates/assets`.
- Core rule: plain `.md` files receive `<!-- ITO:START -->` / `<!-- ITO:END -->` markers; already marked files remain unchanged.
- Verification found no unmarked plain markdown files in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter sample was modified.
- Drill down:
  - `template_bundle_retrofit.abstract.md`
  - `template_bundle_retrofit.overview.md`

## Ito workflow
- **`ito_workflow.md`** covers publishing, validation, and cleanup around coordination-backed workflow assets.
- Shared principles across the child entries:
  - strict path validation
  - read-only published output
  - guarded validation
  - collision-resistant temp naming
  - drift detection between expected and current state
  - cleanup of obsolete specialist assets while preserving coordinator assets

### `published_ito_mirror.md`
- Generates a read-only `docs/ito` mirror from coordination-backed state.
- Mirror path comes from `changes.published_mirror.path`, defaulting to `docs/ito`.
- Path resolution rejects empty paths, absolute paths, parent traversal, and project-root-only paths.
- Renderer skips symlinks and emits a deterministic layout under `README.md`, `changes/active`, `changes/archive`, and `specs`.
- `ito publish` compares generated output against the existing mirror, detects drift, and replaces the mirror from the coordination source of truth.

### `audit_mirror_concurrency_and_temp_naming.md`
- Audit mirror sync uses unique temp worktree and orphan branch names to avoid collisions.
- Naming pattern:
  - `ito-audit-mirror-{pid}-{nanos}-{sequence}`
  - `ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}`
- Flow: detect git worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push/update ref → retry on conflict.
- JSONL merge dedupes identical lines, preserves order, and collapses adjacent reconciled events by incrementing count.
- Retention is bounded by 30 days from newest event and 1000 events.

### `worktree_validation_flow.md`
- `ito worktree validate --change <id> [--json]` emits machine-readable status for pre-tool hooks.
- Validation distinguishes hard failures for main/control checkouts from advisory mismatches for non-main cases.
- Matching uses exact change-id prefixes, preventing false positives such as `<change>-review`.

### `obsolete_specialist_cleanup.md`
- Installer flows pre-clean obsolete `ito-orchestrator` specialist assets during update and force reinstall/init paths.
- Cleanup is a harness-level pre-pass before writing new assets.
- Broken legacy symlinks are removed using `symlink_metadata`.
- Removed legacy paths include:
  - `ito-orchestrator-planner`
  - `ito-orchestrator-researcher`
  - `ito-orchestrator-reviewer`
  - `ito-orchestrator-worker`
- Coordinator assets are preserved, including:
  - `ito-orchestrator.md`
  - `ito-orchestrator-workflow`

## Release workflow guardrails
- **`release_workflow.md`** defines the release pipeline and its safety constraints.
- The release system is split between:
  - `release-plz` for versioning and publishing
  - `cargo-dist` for artifact builds, GitHub Releases, and Homebrew publishing
- End-to-end flow:
  - merge a release PR
  - `release-plz` publishes crates and tags `vX.Y.Z`
  - `cargo-dist` builds artifacts and creates GitHub Releases
  - Homebrew formulas update in `withakay/homebrew-ito`
  - release notes are polished afterward
- Key config files:
  - `.github/workflows/release-plz.yml`
  - `.github/workflows/v-release.yml`
  - `.github/workflows/polish-release-notes.yml`
  - `dist-workspace.toml`
  - `release-plz.toml`
- Important rule: do not set `git_only = true` in `release-plz.toml` because it can miscalculate repository paths during diff/worktree operations.

### `build_and_coverage_guardrails.md`
- `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset.
- `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so only regressions and new violations fail.
- `cargo-deny` allows `wit-bindgen@0.51` as a tolerated duplicate because it is a wasip3 transitive dependency.
- Workflow shape:
  - `make check` → coverage target resolves LLVM vars → `cargo-llvm-cov` → max-lines baseline check → `cargo-deny` duplicate allowance

### `release_plz_guardrails.md`
- `.ito` coordination paths must remain gitignored.
- Already tracked ignored files are removed with `git rm --cached`.
- `release-plz.toml` stays at the repository root for repo discovery in temp clones.
- GitHub Actions runs release-plz on `main` with separate `release` and `release-pr` jobs.
- Important settings:
  - `allow_dirty = false`
  - `publish_allow_dirty = false`
  - workspace changelog updates enabled
  - workspace dependency updates enabled
  - changelog config uses `cliff.toml`
  - `ito-cli` is the only package with git tags enabled
- Protected paths pattern:
  - `^.ito/(changes|specs|modules|workflows|audit)$`
- Rules emphasized:
  - keep `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` gitignored
  - if tracked ignored files appear under `.ito/changes`, untrack them with `git rm --cached`
  - do not unignore `.ito/changes`
  - do not set `git_only = true`

### `manifesto_instruction_implementation_notes.md`
- `synced_at_generation` is only populated when coordination sync returns `Synchronized`.
- `RateLimited` means no sync was observed during generation and must not be reported as fresh success.
- Full `--operation` requires `--change`.
- Embedded operation instructions are scoped to the resolved change state.
- Unconfigured operations render as `null`.

## Source guide workflow
- **`source_guide_workflow.md`** describes the code-map/code-atlas workflow used during apply work.
- Structural model:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md`
  - `source-guide.json` for freshness tracking
- Operational rules:
  - inspect nearby guides before implementing apply changes
  - refresh missing or stale guides when needed
  - use guides for orientation, but verify claims against source
  - update affected guides after structural changes
- Drill down: `source_guides/_index.md` for the workflow overview and hierarchy

## Cross-entry relationships
- `ito_workflow` is the core safety layer for mirror generation, validation, and migration cleanup.
- `release_workflow` adds release, coverage, and repository-state guardrails around publishing.
- `source_guides` supports implementation work by keeping code maps current and source-verified.
- `template_bundle_retrofit` is the most localized pass, focused on consistent marker application in template assets.