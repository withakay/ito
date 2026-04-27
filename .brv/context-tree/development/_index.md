---
children_hash: merged-template-worktree-release
compression_ratio: 0.64
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md]
covers_token_total: 1361
summary_level: d2
token_count: 860
type: summary
---
## Development / ITO Knowledge Overview

This d2 summary compresses the three main development knowledge areas: template retrofit, worktree validation, and release/build reliability. Each entry documents a distinct operational rule set, with shared emphasis on safe automation, machine-readable status, and preserving compliant existing files.

### 1) ITO Templates — `template_bundle_retrofit.md`
- The template bundle retrofit standardizes plain markdown assets under `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` and `<!-- ITO:END -->` markers.
- Only unmarked plain `.md` files are eligible; pre-marked files are preserved unchanged.
- Verification of `ito-rs/crates/ito-templates/assets/adapters` found no unmarked adapter samples, so no adapter markdown was modified.

### 2) ITO Workflow — `worktree_validation_flow.md`
- Worktree validation now uses a dedicated read-only path for change work via `ito worktree validate --change <id> [--json]`.
- The command emits machine-readable status for OpenCode pre-tool hooks, allowing them to distinguish unsafe states from recoverable mismatches.
- **Hard-fail rule:** main/control checkouts are treated as failures.
- **Advisory rule:** non-main mismatches are not fatal and return guidance/recovery instructions.
- Matching uses exact change-id prefixes to avoid false positives, including suffix worktrees such as `<change>-review`.

### 3) Release Workflow — `release_workflow.md` and `build_and_coverage_guardrails.md`
- The release pipeline is split between `release-plz` and `cargo-dist`, with Homebrew publishing and release-note polishing.
- Release/build guardrails cover LLVM toolchain resolution for coverage, max-lines baseline enforcement, and a scoped cargo-deny exception for `wit-bindgen@0.51`.
- The overall purpose is to keep release automation, coverage, and dependency checks reliable without broad exceptions or brittle toolchain assumptions.
