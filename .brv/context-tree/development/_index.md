---
children_hash: d12828c79b660047e62942c3e450ce6c87e49aa248984b22a7f9821cc647923c
compression_ratio: 0.6601752677702045
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 3081
summary_level: d2
token_count: 2034
type: summary
---
# d2 Structural Summary

## Development: Ito workflows and release guardrails

The curated entries describe three closely related operational areas: template marker retrofits, workflow safety/mirror management, and release pipeline guardrails. Together they emphasize strict structure, safe automation, and preserving already-correct state rather than rewriting it.

### Template bundle retrofit
- **`ito_templates/_index.md`** summarizes a marker standardization pass over `ito-rs/crates/ito-templates/assets`.
- Core rule: **plain `.md` files** receive `<!-- ITO:START -->` / `<!-- ITO:END -->` markers; **already marked files are left unchanged**.
- Verification confirmed there were **no unmarked plain markdown files in `ito-rs/crates/ito-templates/assets/adapters`**, so no adapter sample was modified.
- Drill-down:
  - `template_bundle_retrofit.md` — primary retrofit summary and verification facts
  - `template_bundle_retrofit.abstract.md` — abstracted structural view
  - `template_bundle_retrofit.overview.md` — marker retrofit approach

### Ito workflow
- **`ito_workflow/_index.md`** covers how Ito publishes, validates, and maintains coordination-backed workflow assets.
- Shared themes across the child entries:
  - **Safety first**: strict path validation, read-only published output, guarded validation.
  - **Concurrency resilience**: PID + timestamp + atomic sequence naming avoids temp-worktree collisions.
  - **Drift/control management**: publish/validate flows compare expected vs current state.
  - **Migration hygiene**: obsolete specialist assets are removed while coordinator assets are preserved.

#### Child entry structure
- **`published_ito_mirror.md`**
  - Generates a **read-only `docs/ito` mirror** from coordination-backed state.
  - Mirror path is configurable via `changes.published_mirror.path`, defaulting to `docs/ito`.
  - Path resolution is strict: rejects empty paths, absolute paths, parent traversal, and project-root-only paths.
  - Renderer skips symlinks and emits a deterministic layout under `README.md`, `changes/active`, `changes/archive`, and `specs`.
  - `ito publish` compares generated output against the existing mirror, detects drift, and replaces the mirror from the coordination source of truth.

- **`audit_mirror_concurrency_and_temp_naming.md`**
  - Audit mirror sync uses **unique temp worktree and orphan branch names** to prevent collisions.
  - Naming pattern:
    - `ito-audit-mirror-{pid}-{nanos}-{sequence}`
    - `ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}`
  - Flow: detect git worktree → create temp worktree → fetch/checkout branch or orphan → merge JSONL → stage/commit → push/update ref → retry on conflict.
  - JSONL merge dedupes identical lines, preserves order, and collapses adjacent reconciled events by incrementing count.
  - Retention is bounded by **30 days from newest event** and **1000 events**.
  - Conflict handling retries once for push/ref conflicts, with best-effort behavior limited to Git worktrees.

- **`worktree_validation_flow.md`**
  - `ito worktree validate --change <id> [--json]` now emits **machine-readable status** for pre-tool hooks.
  - Validation distinguishes **hard failures** for main/control checkouts from **advisory mismatches** for non-main cases.
  - Matching uses **exact change-id prefixes**, preventing false positives such as `<change>-review`.

- **`obsolete_specialist_cleanup.md`**
  - Installer flows pre-clean obsolete **ito-orchestrator specialist assets** during **update** and **force reinstall/init** paths.
  - Cleanup is a **harness-level pre-pass** before writing new assets.
  - Broken legacy symlinks are removed using `symlink_metadata`.
  - Removed legacy paths include:
    - `ito-orchestrator-planner`
    - `ito-orchestrator-researcher`
    - `ito-orchestrator-reviewer`
    - `ito-orchestrator-worker`
  - Coordinator assets are preserved, including:
    - `ito-orchestrator.md`
    - `ito-orchestrator-workflow`
  - Plain init leaves untouched user files in place.

### Release workflow guardrails
- **`release_workflow/_index.md`** defines the release pipeline and the guardrails that keep publishing, coverage, and manifest rendering consistent.
- The release system is split between:
  - **`release-plz`** for versioning and publishing
  - **`cargo-dist`** for artifact builds, GitHub Releases, and Homebrew publishing

#### Child entry structure
- **`release_workflow.md`**
  - End-to-end release sequence:
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
  - Important rule: **do not set `git_only = true` in `release-plz.toml`** because it can miscalculate repository paths during diff/worktree operations.

- **`build_and_coverage_guardrails.md`**
  - `make check` now resolves `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset.
  - `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so only regressions/new violations fail.
  - `cargo-deny` allows `wit-bindgen@0.51` as a tolerated duplicate because it is a wasip3 transitive dependency.
  - Workflow shape:
    - `make check` → coverage target resolves LLVM vars → `cargo-llvm-cov` → max-lines baseline check → `cargo-deny` duplicate allowance.

- **`release_plz_guardrails.md`**
  - `.ito` coordination paths must remain **gitignored**.
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

- **`manifesto_instruction_implementation_notes.md`**
  - `synced_at_generation` is only populated when coordination sync returns `Synchronized`.
  - `RateLimited` means no sync was observed during generation and must not be reported as fresh success.
  - Full `--operation` requires `--change`.
  - Embedded operation instructions are scoped to the resolved change state.
  - Unconfigured operations render as `null`.

### Source guide workflow
- **`source_guides/_index.md`** summarizes Ito’s code-map/code-atlas workflow used during apply work.
- Structural model:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md`
  - `source-guide.json` tracks freshness
- Operational rules:
  - inspect nearby guides before implementing apply changes
  - refresh missing or stale guides when needed
  - use guides for orientation, but verify claims against source
  - update affected guides after structural changes
- Drill-down:
  - `source_guide_workflow.md` — full workflow, hierarchy, freshness tracking, and verification rules

## Cross-entry relationships
- `ito_workflow` centers on safe mirror generation, validation, and migration cleanup.
- `release_workflow` adds release, coverage, and repository-state guardrails around publishing.
- `source_guides` supports implementation work by keeping code maps current and source-verified.
- `template_bundle_retrofit` is the most localized structural pass, focused on consistent marker application in template assets.