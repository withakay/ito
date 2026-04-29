---
children_hash: 9165e49a171f5c42b4e5d583c3cd2ad2918626c0b6bfc258a64d6cb6a4cbd6ee
compression_ratio: 0.583902263744161
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md, source_guides/_index.md]
covers_token_total: 2783
summary_level: d2
token_count: 1625
type: summary
---
# Ito Knowledge Structure

This d2 summary groups the current knowledge into three operational areas: **template standardization**, **workflow/synchronization**, and **release + documentation governance**. Each child entry below is the drill-down point for implementation details, rules, and exact behavior.

## 1) Template bundle retrofit
**`ito_templates/_index.md`** captures the marker standardization pass for `ito-rs/crates/ito-templates/assets`.

- Core rule: **plain markdown** files were retrofitted with `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Files already pre-marked were intentionally left unchanged.
- Verification confirmed there were no unmarked plain markdown files in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter sample changed.
- Process pattern: `scan assets -> add markers to plain markdown -> leave pre-marked files unchanged -> verify adapter sample status`

**Drill down:** `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`

## 2) Ito workflow: publishing, validation, and audit mirroring
**`ito_workflow/_index.md`** describes how Ito safely synchronizes coordination-backed state into read-only published outputs and audit mirrors.

### Main structure
- **Source of truth vs published output**
  - writable coordination-backed state remains the source of truth
  - `docs/ito` is the generated read-only mirror for plain GitHub/main checkouts
- **Worktree safety and validation**
  - validation distinguishes unsafe main/control checkouts from advisory mismatches elsewhere
  - machine-readable status is emitted for OpenCode pre-tool hooks
  - exact change-id prefix matching avoids substring false positives, including suffix worktrees like `<change>-review`
- **Audit mirror synchronization**
  - `mirror.rs` syncs audit JSONL into an internal branch
  - it uses unique temp worktree/orphan branch names, JSONL deduplication, bounded retention, and conflict retries

### Child entry roles
- **`published_ito_mirror.md`**
  - safe path resolution for `changes.published_mirror.path`
  - default mirror path: `docs/ito`
  - read-only output layout under `README.md`, `changes/active`, `changes/archive`, `specs`
  - symlink skipping, drift detection, and replacement via `ito publish`
- **`worktree_validation_flow.md`**
  - command: `ito worktree validate --change <id> [--json]`
  - hard-fails main/control checkouts
  - advisory mismatch behavior outside main
  - exact prefix matching for change IDs
- **`audit_mirror_concurrency_and_temp_naming.md`**
  - temp worktree naming: `ito-audit-mirror-{pid}-{nanos}-{sequence}`
  - orphan branch naming: `ito-audit-mirror-orphan-{pid}-{nanos}-{sequence}`
  - atomic sequence counter prevents collisions
  - JSONL merge dedupes identical lines, preserves order, and collapses adjacent reconciled events by count
  - retention: 30 days from newest event and max 1000 events
  - retry policy: internal branch append retries once; push retries after non-fast-forward by refetching and merging again
  - runs only inside a Git worktree; missing remote branches fall back to an orphan branch
- **`obsolete_specialist_cleanup.md`**
  - update and force reinstall/init pre-clean legacy `ito-orchestrator-*` specialist assets
  - only planner/researcher/reviewer/worker assets are migrated to `ito-*`
  - coordinator-level assets keep their existing names

### Shared patterns
- Safety first: path validation, worktree checks, conflict detection
- Read-only outputs are derived, not edited directly
- Deterministic reconciliation via drift detection and JSONL deduplication
- Bounded growth via retention limits
- Retry with constraints: conflicts are retried once, then surfaced
- Migration hygiene removes obsolete specialist files without disturbing coordinator assets

## 3) Release workflow, guardrails, and manifesto rendering
**`release_workflow/_index.md`** summarizes the release pipeline plus two supporting rule sets.

### Core release pipeline
- The release flow is split between **release-plz** and **cargo-dist**
  - `release-plz` merges the release PR, publishes crates.io releases, and creates version tags
  - `cargo-dist` consumes tags to build and publish GitHub Releases
- Homebrew updates are pushed to **withakay/homebrew-ito**
- Release notes are polished after publication
- Key automation files:
  - `.github/workflows/release-plz.yml`
  - `.github/workflows/v-release.yml`
  - `.github/workflows/polish-release-notes.yml`
  - `dist-workspace.toml`
  - `release-plz.toml`
- Important constraints:
  - do **not** set `git_only = true` in `release-plz.toml`
  - `publish-homebrew-formula` fails if the generated formula already contains a `service do` block
  - local installation supports the `withakay/ito` tap, including `brew install`, `brew upgrade`, `brew unlink`, and `brew link`

### Build and coverage guardrails
**`build_and_coverage_guardrails.md`**
- `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset
- This prevents mixed Homebrew/rustup failures
- `cargo-llvm-cov` runs after toolchain resolution
- Oversized Rust files are controlled by `ito-rs/tools/max_lines_baseline.txt`
- `cargo-deny` allows the narrowly scoped duplicate `wit-bindgen@0.51` for wasip3

Guardrail flow:
`make check -> coverage target resolves LLVM toolchain vars -> cargo-llvm-cov runs -> max-lines guardrail compares against baseline -> cargo-deny accepts wit-bindgen@0.51 duplicate`

### Manifesto instruction implementation notes
**`manifesto_instruction_implementation_notes.md`**
- `synced_at_generation` is populated only when coordination sync returns **Synchronized**
- **RateLimited** means no sync was observed during generation and must not be treated as fresh success
- `full --operation` requires `--change`
- Embedded operation instructions are scoped to the resolved change state
- Unconfigured operations render as `null`

### Why these matter
- The release workflow covers publishing and packaging
- Build/coverage guardrails protect release confidence and CI reliability
- Manifesto instruction rules define how sync state and operation visibility are rendered

## 4) Source guide workflow
**`source_guides/_index.md`** documents the code atlas / source-guide process used during apply work.

- The workflow is: inspect nearby `source-guide.md` files, refresh missing or stale guides, read them for orientation, verify claims against source, and update affected guides after structural changes.
- Guide coverage spans:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md` files
- `source-guide.json` tracks freshness.
- Guides are orientation aids only; implementation claims must be verified against source.

**Drill down:** `source_guide_workflow.md`
