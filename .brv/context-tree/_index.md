---
children_hash: f43e871fec4fe3ff5cabb8e4418ccea85d6c37b5781728094d0dcda8fba30cab
compression_ratio: 0.8585313174946004
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 926
summary_level: d3
token_count: 795
type: summary
---
# Development Overview

This area centers on controlled transformation: preserve already-correct artifacts, enforce guardrails with explicit baselines, and make release behavior depend on defined workflow states. See the child entries for the detailed implementation and file-level rules.

## Template bundle retrofit
Refer to **ito_templates/_index.md** and **template_bundle_retrofit.md** for the asset rewrite rules.

- Applies to `ito-rs/crates/ito-templates/assets`.
- Plain markdown files get `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Already marked files are left unchanged.
- Adapter assets are checked separately; no unmarked plain markdown was found in `ito-rs/crates/ito-templates/assets/adapters`.
- Core flow: scan assets → retrofit plain markdown → preserve pre-marked files → verify adapter status.

## Release workflow
Refer to **release_workflow/_index.md** and **release_workflow.md** for the end-to-end release pipeline.

- `release-plz` manages release PRs, publishes crates.io releases, and creates version tags.
- `cargo-dist` consumes version tags to produce GitHub Releases.
- Homebrew updates are pushed to `withakay/homebrew-ito`.
- Supporting files include `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, and `release-plz.toml`.
- Workflow order: merge release PR → release-plz publishes and tags `vX.Y.Z` → cargo-dist releases → Homebrew update → release notes polishing.
- Key rules:
  - `release-plz.toml` must not set `git_only = true`; it can miscalculate repo paths in diff/worktree flows.
  - `publish-homebrew-formula` fails if the generated formula includes a service `do` block.

## Build and coverage guardrails
Refer to **build_and_coverage_guardrails.md** for the baseline-driven validation rules.

- The `Makefile` coverage target resolves `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset.
- `ito-rs/tools/max_lines_baseline.txt` is the baseline for existing oversized Rust files.
- The line-limit guardrail should fail only on regressions or new violations.
- `cargo-deny` allows the exact duplicate `wit-bindgen@0.51` as a wasip3 transitive duplicate.

## Manifesto instruction implementation notes
Refer to **manifesto_instruction_implementation_notes.md** for generation-time instruction behavior.

- `synced_at_generation` is set only when coordination sync returns `Synchronized`.
- `RateLimited` means no sync was observed during generation and must not be treated as fresh success.
- Full `--operation` requires `--change`.
- Embedded operation instructions are scoped to the resolved change state.
- Unconfigured operations render as `null`.

## Cross-cutting patterns
- Release behavior is stateful and tool-driven: `release-plz` for versioning, `cargo-dist` for artifact publication, Homebrew for distribution updates.
- Validation is conservative: preserve existing correct files, baseline known violations, and fail only on new regressions.
- Several rules are path- and config-sensitive, especially `release-plz.toml`, `dist-workspace.toml`, `Makefile`, and `ito-rs/tools/max_lines_baseline.txt`.