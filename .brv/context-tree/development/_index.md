---
children_hash: e742710341bd2c7195a23047df504e87aa4df5fc6c23d96feea82f633705c133
compression_ratio: 0.5795038633590891
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md]
covers_token_total: 2459
summary_level: d2
token_count: 1425
type: summary
---
# d2 Structural Summary

## Top-level structure
This level groups three related operational areas: **template asset retrofit**, **Ito workflow synchronization/validation**, and **release workflow/verification**. Together they describe how the project standardizes markdown assets, safely publishes and validates coordination-backed state, and enforces release guardrails.

---

## `ito_templates/_index.md`
### Theme
Structural knowledge for marker standardization across `ito-rs/crates/ito-templates/assets`.

### Core facts
- Plain `.md` files were retrofitted with `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Files that were already pre-marked were left unchanged.
- Verification found no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter sample changed.

### Structural rule
- **plain markdown** → add ITO markers
- **already marked markdown** → preserve as-is

### Process pattern
`scan assets -> add markers to plain markdown -> leave pre-marked files unchanged -> verify adapter sample status`

### Drill-down
- `template_bundle_retrofit.md` for the primary retrofit and verification details
- `template_bundle_retrofit.abstract.md` for the compressed structural view
- `template_bundle_retrofit.overview.md` for the marker retrofit approach

---

## `ito_workflow/_index.md`
### Theme
How Ito publishes, validates, and safely synchronizes coordination-backed state into a read-only `docs/ito` mirror.

### Core relationship model
- Coordination-backed Ito state is the writable source of truth.
- `docs/ito` is a generated read-only mirror for plain GitHub/main checkouts.
- Drift is resolved by regeneration and replacement, not manual edits.

### Child entry roles
- **`published_ito_mirror.md`**
  - Safe path resolution for `changes.published_mirror.path`
  - Default mirror path: `docs/ito`
  - Read-only output layout under `README.md`, `changes/active`, `changes/archive`, and `specs`
  - Symlink skipping during generation
  - Drift detection by comparing generated output to existing mirror
  - Replacement driven by the `ito publish` CLI
- **`worktree_validation_flow.md`**
  - Command: `ito worktree validate --change <id> [--json]`
  - Hard-fails main/control checkouts
  - Provides advisory mismatch guidance outside main
  - Emits machine-readable status for hooks
  - Uses exact change-id prefix matching to avoid substring false positives, including suffix worktrees like `<change>-review`
- **`audit_mirror_concurrency_and_temp_naming.md`**
  - `mirror.rs` syncs audit JSONL into an internal branch
  - Uses unique temp worktree and orphan branch names
  - Deduplicates JSONL, applies bounded retention, and retries on conflicts

### Shared design patterns
- Safety first: path validation, worktree checks, conflict detection
- Read-only output generation
- Deterministic reconciliation
- Bounded growth through retention limits
- Retry with constraints: conflict handling is limited and surfaced when unresolved

### Drill-down
- `published_ito_mirror.md` for mirror generation and drift replacement
- `worktree_validation_flow.md` for validation semantics and hook output
- `audit_mirror_concurrency_and_temp_naming.md` for concurrency, merge, and retry behavior

---

## `release_workflow/_index.md`
### Theme
Release automation, build verification, and manifesto instruction rendering rules.

### Core pipeline
Release is split across **release-plz** and **cargo-dist**:
- `release-plz` merges the release PR, publishes crates.io releases, and creates version tags.
- `cargo-dist` consumes tags to build and publish GitHub Releases.
- Homebrew formula updates are pushed to `withakay/homebrew-ito`.
- Release notes are polished after publication.

### Key automation files
- `.github/workflows/release-plz.yml`
- `.github/workflows/v-release.yml`
- `.github/workflows/polish-release-notes.yml`
- `dist-workspace.toml`
- `release-plz.toml`

### Important constraints
- Do **not** set `git_only = true` in `release-plz.toml`; it can miscalculate repository paths during diff/worktree operations.
- The `publish-homebrew-formula` job fails if the generated formula already contains a `service do` block.
- Local installation supports the `withakay/ito` tap, including `brew install`, `brew upgrade`, `brew unlink`, and `brew link`.

### Child entry roles
- **`release_workflow.md`**
  - End-to-end release automation and publishing pipeline
- **`build_and_coverage_guardrails.md`**
  - `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset
  - Prevents mixed Homebrew/rustup coverage failures
  - Runs `cargo-llvm-cov`
  - Enforces the max-lines baseline in `ito-rs/tools/max_lines_baseline.txt`
  - Accepts only the narrowly scoped `wit-bindgen@0.51` duplicate in cargo-deny
- **`manifesto_instruction_implementation_notes.md`**
  - `synced_at_generation` is only set when sync returns `Synchronized`
  - `RateLimited` means no sync was observed and must not be treated as fresh success
  - `full --operation` requires `--change`
  - Embedded operation instructions are scoped to the resolved change state
  - Unconfigured operations render as `null`

### Shared design patterns
- Release correctness is separated from verification and rendering rules.
- Build/test guardrails protect release confidence.
- Sync status semantics are strict: only observed synchronization counts as fresh success.

### Drill-down
- `release_workflow.md` for release-plz, cargo-dist, GitHub Releases, and Homebrew publishing
- `build_and_coverage_guardrails.md` for coverage and dependency guardrails
- `manifesto_instruction_implementation_notes.md` for sync-state and instruction rendering semantics