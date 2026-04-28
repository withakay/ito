---
children_hash: merged-development-index-published-mirror-manifesto
compression_ratio: 0.78
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1900
summary_level: d3
token_count: 1500
type: summary
---
# Development Overview

This domain groups operational knowledge for Ito template retrofits, workflow publication and validation, release/build governance, and manifesto instruction behavior. Across all areas, the common pattern is safe automation with explicit machine-readable outcomes while preserving already-correct artifacts and keeping writable coordination state authoritative.

## ITO Templates - `template_bundle_retrofit.md`
- Standardizes plain markdown assets under `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Only unmarked `.md` files are eligible; already marked files are left unchanged.
- Verification confirmed no unmarked adapter samples in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter markdown was modified.

## ITO Workflow - `published_ito_mirror.md` / `worktree_validation_flow.md`
- Covers two complementary concerns: publishing a read-only mirror of coordination-backed state and validating worktrees safely during change work.
- Authoritative writable state remains in coordination storage; `docs/ito` is generated as a consumable read-only mirror for plain GitHub/main checkouts.
- Mirror path handling is safety-first: project-relative, configurable, and strictly validated before generation.
- The published mirror is deterministic and excludes symlinks, making it safe for consumption without exposing writable state.
- `ito worktree validate --change <id> [--json]` emits machine-readable status for hooks.
- Main/control checkouts are hard failures; non-main mismatches are advisory and include recovery guidance.
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
