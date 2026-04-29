---
children_hash: beeff4f394143d26f07040f206b9aba24b043df3f8217732baf9db0fdc97ba5b
compression_ratio: 0.9168618266978923
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1708
summary_level: d3
token_count: 1566
type: summary
---
# Development Knowledge Overview

The `development` domain groups four operational areas: template standardization, Ito workflow/state synchronization, release governance, and source-guide maintenance. The child entries below are the drill-down points for exact behavior and implementation details.

## Template standardization
**`ito_templates/_index.md`** covers the marker retrofit for `ito-rs/crates/ito-templates/assets`.

- Plain markdown files were updated to include `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Files already marked were left unchanged.
- Verification found no unmarked plain markdown files in `ito-rs/crates/ito-templates/assets/adapters`, so the adapter sample was unchanged.
- Core process: `scan assets -> add markers to plain markdown -> preserve pre-marked files -> verify adapter status`

Drill down: `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`

## Ito workflow: publishing, validation, and audit mirroring
**`ito_workflow/_index.md`** describes how coordination-backed state is safely published and mirrored into read-only outputs.

### Main structure
- **Source of truth vs published output**
  - writable coordination-backed state remains authoritative
  - `docs/ito` is the generated read-only mirror for plain GitHub/main checkouts
- **Validation and safety**
  - main/control checkouts fail hard
  - mismatches outside main are advisory
  - machine-readable status supports OpenCode pre-tool hooks
  - change IDs use exact prefix matching to avoid substring false positives, including suffix worktrees like `<change>-review`
- **Audit mirror synchronization**
  - `mirror.rs` syncs audit JSONL into an internal branch
  - uses unique temp worktree/orphan branch names, JSONL deduplication, bounded retention, and conflict retries

### Child entry roles
- **`published_ito_mirror.md`**
  - safe path resolution for `changes.published_mirror.path`
  - default mirror path: `docs/ito`
  - read-only layout under `README.md`, `changes/active`, `changes/archive`, `specs`
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
  - retry policy: one internal branch append retry; push retries after non-fast-forward via refetch and merge
  - runs only inside a Git worktree; missing remote branches fall back to an orphan branch

### Shared patterns
- Safety-first validation, conflict detection, and path checks
- Read-only outputs are derived, not edited directly
- Deterministic reconciliation via drift detection and JSONL deduplication
- Bounded growth through retention limits
- Conflicts are retried once, then surfaced

## Release workflow, guardrails, and manifesto rendering
**`release_workflow/_index.md`** summarizes the release pipeline and supporting rules.

### Core release pipeline
- Release flow is split between **release-plz** and **cargo-dist**
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
- This avoids mixed Homebrew/rustup failures
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
- Release workflow handles publishing and packaging
- Guardrails protect CI reliability and release confidence
- Manifesto rendering rules define how sync state and operation visibility are presented

## Source guide workflow
**`source_guides/_index.md`** documents the code atlas / source-guide process used during apply work.

- Workflow: inspect nearby `source-guide.md` files, refresh missing or stale guides, read them for orientation, verify claims against source, then update affected guides after structural changes.
- Coverage includes:
  - root `source-guide.md`
  - `ito-rs/source-guide.md`
  - `ito-rs/crates/source-guide.md`
  - per-crate `source-guide.md` files
- `source-guide.json` tracks freshness.
- Source guides are orientation aids only; implementation claims must be verified against source.

Drill down: `source_guide_workflow.md`