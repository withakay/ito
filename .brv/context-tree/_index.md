---
children_hash: 9f70a7cfd497e8957e3d050597cf1d18aaa9cbd3cabd90cb66f05f58aede794d
compression_ratio: 0.8621837549933422
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1502
summary_level: d3
token_count: 1295
type: summary
---
# d3 Structural Summary

## Development area overview
The `development` domain groups three operational concerns: **template asset retrofit**, **Ito workflow synchronization/validation**, and **release workflow/verification**. Across all three, the recurring pattern is controlled transformation of generated or curated state, with explicit validation rules and drift-safe regeneration.

## `ito_templates/_index.md`
Focuses on marker standardization for `ito-rs/crates/ito-templates/assets`.

- Plain `.md` files receive `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Already pre-marked markdown files are preserved unchanged.
- Verification confirmed no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so adapter samples required no changes.

Structural rule:
- **plain markdown** → add ITO markers
- **already marked markdown** → leave as-is

Process pattern:
- `scan assets -> add markers to plain markdown -> preserve pre-marked files -> verify adapter sample status`

Drill down:
- `template_bundle_retrofit.md`
- `template_bundle_retrofit.abstract.md`
- `template_bundle_retrofit.overview.md`

## `ito_workflow/_index.md`
Describes how Ito publishes, validates, and synchronizes coordination-backed state into the read-only `docs/ito` mirror.

### Core relationship
- Coordination-backed Ito state is the writable source of truth.
- `docs/ito` is a generated read-only mirror for plain GitHub/main checkouts.
- Drift is handled by regeneration and replacement, not manual edits.

### Child entries
- **`published_ito_mirror.md`**
  - Safe path resolution for `changes.published_mirror.path`
  - Default mirror path: `docs/ito`
  - Read-only layout under `README.md`, `changes/active`, `changes/archive`, and `specs`
  - Symlink skipping during generation
  - Drift detection compares generated output to the existing mirror
  - Replacement is driven by `ito publish`

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
- Safety-first validation: path checks, worktree checks, conflict detection
- Read-only output generation
- Deterministic reconciliation
- Bounded growth via retention limits
- Retry behavior is constrained and conflict-aware

Drill down:
- `published_ito_mirror.md`
- `worktree_validation_flow.md`
- `audit_mirror_concurrency_and_temp_naming.md`

## `release_workflow/_index.md`
Covers release automation, build verification, and manifesto instruction rendering.

### Core pipeline
Release flow is split between **release-plz** and **cargo-dist**:
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

### Child entries
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
- Build and dependency guardrails protect release confidence.
- Sync-status semantics are strict: only observed synchronization counts as fresh success.

Drill down:
- `release_workflow.md`
- `build_and_coverage_guardrails.md`
- `manifesto_instruction_implementation_notes.md`