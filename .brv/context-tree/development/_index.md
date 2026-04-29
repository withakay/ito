---
children_hash: df7d87cc6721d2dd728126344a9185470fadd1bff3b30e001cf43247b9765ab3
compression_ratio: 0.7361290322580645
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 3100
summary_level: d2
token_count: 2282
type: summary
---
# ito context tree: structural overview

This level groups the main workflow and release knowledge for Ito, plus the source-guide orientation system used during apply work. The entries emphasize safety, reproducibility, and maintenance hygiene across publishing, validation, and release operations.

## 1) Template bundle retrofit
Reference: **ito_templates/_index.md** → drill down to `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`

- The template bundle retrofit standardizes markdown marker usage across `ito-rs/crates/ito-templates/assets`.
- Core rule: retrofitted plain markdown files get `<!-- ITO:START -->` / `<!-- ITO:END -->`; files already marked stay unchanged.
- Verification found no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so adapter samples were left untouched.
- The process pattern is:
  - scan assets
  - add markers to plain markdown
  - leave pre-marked files unchanged
  - verify adapter sample status

## 2) Ito workflow safety and synchronization
Reference: **ito_workflow/_index.md** → drill down to `published_ito_mirror.md`, `audit_mirror_concurrency_and_temp_naming.md`, `worktree_validation_flow.md`, `obsolete_specialist_cleanup.md`, `context.md`

- This cluster covers how Ito publishes, validates, and maintains coordination-backed workflow assets.
- Common themes:
  - safe mirror generation
  - drift detection
  - concurrency resilience
  - migration hygiene

### Published mirror generation
- `published_ito_mirror.md` defines a read-only `docs/ito` mirror generated from coordination-backed state.
- Mirror path is configurable via `changes.published_mirror.path`, defaulting to `docs/ito`.
- Path resolution is strict: rejects empty, absolute, parent-traversal, and project-root-only paths.
- Renderer skips symlinks and writes a deterministic layout under:
  - `README.md`
  - `changes/active`
  - `changes/archive`
  - `specs`
- `ito publish` compares generated output to the existing mirror, detects drift, and replaces the mirror from the source of truth.

### Audit mirror concurrency and temp naming
- `audit_mirror_concurrency_and_temp_naming.md` documents safe concurrent audit mirror synchronization.
- Temp worktrees and orphan branches use unique names built from:
  - PID
  - `SystemTime` nanos
  - atomic sequence counter
- Naming pattern:
  - `ito-audit-mirror-{pid}-{nanos}-{sequence}`
  - `ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}`
- Flow:
  - detect git worktree
  - create temp worktree
  - fetch/checkout branch or orphan
  - merge JSONL
  - stage/commit
  - push/update ref
  - retry on conflict
- JSONL merge behavior dedupes identical lines, preserves order, and collapses adjacent reconciled events by incrementing count.
- Retention is bounded by age and count:
  - 30 days from newest event
  - 1000 events
- Conflict handling retries once for push/ref conflicts.

### Worktree validation flow
- `worktree_validation_flow.md` makes `ito worktree validate --change <id> [--json]` emit machine-readable status for pre-tool hooks.
- Validation distinguishes:
  - hard failures for main/control checkouts
  - advisory mismatches for non-main cases, with recovery guidance
- Matching uses exact change-id prefixes to avoid false positives like `<change>-review`.

### Obsolete specialist cleanup
- `obsolete_specialist_cleanup.md` covers installer cleanup for obsolete `ito-orchestrator` specialist assets during update and force reinstall/init paths.
- Cleanup runs as a harness-level pre-pass before writing new assets.
- Broken legacy symlinks are removed with `symlink_metadata`.
- Removed legacy paths include:
  - `ito-orchestrator-planner`
  - `ito-orchestrator-researcher`
  - `ito-orchestrator-reviewer`
  - `ito-orchestrator-worker`
- Coordinator assets are preserved, including:
  - `ito-orchestrator.md`
  - `ito-orchestrator-workflow`
- Plain init intentionally leaves user files untouched.

### Shared structural rules
- `context.md` sets the domain scope: project-relative mirror resolution, read-only mirror generation, drift detection, and coordination-backed source of truth.
- The subtopics are closely related:
  - `published_ito_mirror.md` and `audit_mirror_concurrency_and_temp_naming.md` focus on mirror generation and state sync.
  - `worktree_validation_flow.md` protects change-related operations through read-only validation.
  - `obsolete_specialist_cleanup.md` handles migration safety after orchestrator asset renames.

## 3) Release workflow, guardrails, and rendering rules
Reference: **release_workflow/_index.md** → drill down to `release_workflow.md`, `release_plz_guardrails.md`, `build_and_coverage_guardrails.md`, `manifesto_instruction_implementation_notes.md`

- This cluster defines the end-to-end release pipeline and the guardrails that keep release, coverage, and rendered output consistent.
- Main relationship:
  - `release_workflow.md` = release pipeline
  - `release_plz_guardrails.md` = release-plz constraints and coordination paths
  - `build_and_coverage_guardrails.md` = verification resilience
  - `manifesto_instruction_implementation_notes.md` = sync/rendering semantics for generated output

### Core release pipeline
- `release_workflow.md` defines:
  - `release-plz` merges release PRs, publishes crates.io releases, and creates `vX.Y.Z` tags
  - `cargo-dist` consumes version tags to build artifacts and create GitHub Releases
  - Homebrew formula updates are pushed to `withakay/homebrew-ito`
- Key workflow/config files:
  - `.github/workflows/release-plz.yml`
  - `.github/workflows/v-release.yml`
  - `.github/workflows/polish-release-notes.yml`
  - `dist-workspace.toml`
  - `release-plz.toml`
- Important rule: do not set `git_only = true` in `release-plz.toml`, because it can miscalculate repository paths during diff/worktree operations.

### release-plz guardrails
- `release_plz_guardrails.md` requires running `release-plz` from the repo root with dirty publishing disabled:
  - `allow_dirty = false`
  - `publish_allow_dirty = false`
  - workspace changelog updates enabled
  - workspace dependency updates enabled
  - `cliff.toml` used as changelog config
  - git tags enabled only for `ito-cli`
- Coordination-branch paths remain gitignored:
  - `.ito/changes`
  - `.ito/specs`
  - `.ito/modules`
  - `.ito/workflows`
  - `.ito/audit`
- If ignored files become tracked, the fix is `git rm --cached` while keeping local files.
- GitHub Actions release flow uses:
  - GitHub App token
  - `fetch-depth: 0`
  - build-essential
  - mise toolchain setup
  - Rust cache
  - `release-plz/action@v0.5`
  - `CARGO_REGISTRY_TOKEN`

### Build and coverage guardrails
- `build_and_coverage_guardrails.md` hardens verification for mixed Homebrew/rustup environments.
- Main fixes:
  - `Makefile` coverage target derives `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset
  - `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so the guardrail only fails on regressions or new violations
  - `wit-bindgen@0.51` is allowed as a cargo-deny duplicate because it is a wasip3 transitive dependency
- The documented flow is:
  - `make check`
  - coverage target resolves LLVM toolchain vars
  - `cargo-llvm-cov`
  - max-lines baseline check
  - `cargo-deny` duplicate allowance for `wit-bindgen@0.51`

### Manifesto instruction implementation notes
- `manifesto_instruction_implementation_notes.md` defines how generation output should reflect coordination sync status.
- Key constraints:
  - `synced_at_generation` is set only when sync returns `Synchronized`
  - `RateLimited` means no sync was observed and must not be reported as fresh success
  - `full --operation` requires `--change`
  - embedded operation instructions are scoped to resolved change state
  - unconfigured operations render as `null`

## 4) Source guide workflow
Reference: **source_guides/_index.md** → drill down to `source_guide_workflow.md`

- Ito’s source-guide system is a code map / code atlas workflow used during apply work.
- Structural model:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md`
- `source-guide.json` tracks freshness.
- Operational rules:
  - inspect nearby guides before implementing an apply change
  - refresh missing or stale guides
  - read guides for orientation, but verify important claims against source
  - update affected guides after structural changes

## Cross-cutting patterns across the tree
- **Safety first**: strict path validation, read-only mirrors, and guarded worktree validation.
- **Concurrency resilience**: atomic counters plus time-based naming prevent temp resource collisions.
- **Drift and control management**: mirrors and validation both rely on explicit reconciliation and machine-readable status.
- **Migration hygiene**: obsolete specialist assets are cleaned up without disturbing coordinator assets or user files.
- **Release stability**: coverage, line-limit, and cargo-deny guardrails keep the release pipeline reliable across toolchain variations.