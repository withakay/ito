---
children_hash: 0caf8195d6f4b500d1136a6a96d297271952452446ad82e825524f7d59ef1b65
compression_ratio: 0.6979591836734694
condensation_order: 2
covers: [ito_templates/_index.md, release_workflow/_index.md]
covers_token_total: 1225
summary_level: d2
token_count: 855
type: summary
---
# Development Knowledge Overview

This d2 summary covers the release pipeline, build/coverage guardrails, manifesto instruction implementation notes, and template marker retrofit. The main pattern across the area is controlled transformation: preserve already-correct artifacts, enforce guardrails with explicit baselines, and make release behavior depend on well-defined workflow states.

## Template bundle retrofit
See **ito_templates/_index.md** and drill into **template_bundle_retrofit.md** for details.

- Applies to `ito-rs/crates/ito-templates/assets`.
- Core rule: **plain markdown** files receive `<!-- ITO:START -->` / `<!-- ITO:END -->` markers; **already marked** files are left unchanged.
- Adapter assets are verified separately; no unmarked plain markdown was found in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter sample changed.
- Structural process: `scan assets -> retrofit plain markdown -> preserve pre-marked files -> verify adapter status`.

## Release workflow
See **release_workflow.md** under **release_workflow/_index.md** for the end-to-end pipeline.

- Release-plz manages release PRs, publishes crates.io releases, and creates version tags.
- cargo-dist consumes version tags to produce GitHub Releases.
- Homebrew updates are pushed to `withakay/homebrew-ito`.
- Supporting files: `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, `release-plz.toml`.
- Workflow order: merge release PR -> release-plz publishes and tags `vX.Y.Z` -> cargo-dist releases -> Homebrew update -> release notes polishing.
- Important rules:
  - `release-plz.toml` must not set `git_only = true`; it can miscalculate repo paths in diff/worktree flows.
  - `publish-homebrew-formula` fails if the generated formula includes a service `do` block.
- Local Homebrew operations are documented via install/upgrade/unlink-link/version verification steps.

## Build and coverage guardrails
See **build_and_coverage_guardrails.md**.

- The `Makefile` coverage target resolves `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset.
- `ito-rs/tools/max_lines_baseline.txt` is the baseline for existing oversized Rust files; the line-limit guardrail should fail only on regressions or new violations.
- `cargo-deny` allows the exact duplicate `wit-bindgen@0.51` as a wasip3 transitive duplicate.
- The guardrail pattern is baseline-driven: distinguish legacy exceptions from new breakage.

## Manifesto instruction implementation notes
See **manifesto_instruction_implementation_notes.md**.

- `synced_at_generation` is set only when coordination sync returns `Synchronized`.
- `RateLimited` means no sync was observed during generation and must not be treated as fresh success.
- Full `--operation` requires `--change`.
- Embedded operation instructions are scoped to the resolved change state.
- Unconfigured operations render as `null`.

## Cross-cutting structure
- Release behavior is stateful and tool-driven: release-plz for versioning, cargo-dist for artifact publication, Homebrew for distribution updates.
- Validation is conservative: preserve existing correct files, baseline known violations, and only fail on new regressions.
- Several rules are path- and config-sensitive, especially `release-plz.toml`, `dist-workspace.toml`, `Makefile`, and `ito-rs/tools/max_lines_baseline.txt`.