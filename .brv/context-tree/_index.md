---
children_hash: merged-development-index
compression_ratio: 0.75
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 860
summary_level: d3
token_count: 720
type: summary
---
# development

This domain groups the operational knowledge for ITO template retrofits, worktree validation, and release/build orchestration. Across all three areas, the common pattern is safe automation with explicit machine-readable outcomes while preserving already-correct artifacts.

## ITO Templates — `template_bundle_retrofit.md`
- Standardizes plain markdown assets under `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Only unmarked `.md` files are eligible; already marked files are left unchanged.
- Verification confirmed no unmarked adapter samples in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter markdown was modified.

## ITO Workflow — `worktree_validation_flow.md`
- Uses `ito worktree validate --change <id> [--json]` as a dedicated read-only validation path for change work.
- Produces machine-readable status for OpenCode pre-tool hooks so they can distinguish unsafe states from recoverable mismatches.
- Hard-fail rule: main/control checkouts are failures.
- Advisory rule: non-main mismatches are not fatal; they return guidance and recovery instructions.
- Matching uses exact change-id prefixes to avoid false positives, including suffix worktrees such as `<change>-review`.

## Release Workflow — `release_workflow.md` / `build_and_coverage_guardrails.md`
- The release pipeline is split between `release-plz` and `cargo-dist`, with Homebrew publishing and CI-based release note polishing.
- Build and coverage guardrails document LLVM tool resolution, max-lines baseline enforcement, and the narrowly scoped `wit-bindgen@0.51` cargo-deny exception.
- Important rule: do not set `git_only = true` in `release-plz.toml`; it can miscalculate repo paths during diff/worktree operations.
