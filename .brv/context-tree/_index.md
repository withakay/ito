---
children_hash: merged-development-index
compression_ratio: 0.78
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1100
summary_level: d3
token_count: 880
type: summary
---
# Development Overview

This domain groups operational knowledge for Ito template retrofits, worktree validation, manifesto instruction behavior, and release/build orchestration. Across all areas, the common pattern is safe automation with explicit machine-readable outcomes while preserving already-correct artifacts.

## ITO Templates - `template_bundle_retrofit.md`
- Standardizes plain markdown assets under `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Only unmarked `.md` files are eligible; already marked files are left unchanged.
- Verification confirmed no unmarked adapter samples in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter markdown was modified.

## ITO Workflow - `worktree_validation_flow.md`
- Uses `ito worktree validate --change <id> [--json]` as a dedicated read-only validation path for change work.
- Produces machine-readable status for OpenCode pre-tool hooks so they can distinguish unsafe states from recoverable mismatches.
- Hard-fail rule: main/control checkouts are failures.
- Advisory rule: non-main mismatches are not fatal; they return guidance and recovery instructions.
- Matching uses exact change-id prefixes to avoid false positives, including suffix worktrees such as `<change>-review`.

## Release Workflow - `release_workflow.md` / `build_and_coverage_guardrails.md`
- The release pipeline is split between `release-plz` and `cargo-dist`, with Homebrew publishing and CI-based release note polishing.
- Build and coverage guardrails document LLVM tool resolution, max-lines baseline enforcement, and the narrowly scoped `wit-bindgen@0.51` cargo-deny exception.
- Important rule: do not set `git_only = true` in `release-plz.toml`; it can miscalculate repo paths during diff/worktree operations.

## Manifesto instruction implementation notes
- `synced_at_generation` is set only when coordination sync returns `Synchronized`.
- `RateLimited` means no sync was observed during generation and must not be treated as fresh success.
- Full `--operation` requires `--change`.
- Embedded operation instructions are scoped to the resolved change state.
- Unconfigured operations render as `null`.
