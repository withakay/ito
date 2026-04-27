---
children_hash: 4d9cb32f1723d26a8de9bbcdf2446d600e16283345991a1410419a0d52f2b041
compression_ratio: 0.8642297650130548
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 766
summary_level: d3
token_count: 662
type: summary
---
# development

The development domain centers on two operational themes: template asset normalization and release/build reliability. Across both topics, the pattern is to enforce invariants during automation while preserving already-correct artifacts.

## ito_templates
**Drill down:** `ito_templates/_index.md` → `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`

- Standardizes markdown assets under `ito-rs/crates/ito-templates/assets`.
- Core rule: plain `.md` files receive `<!-- ITO:START -->` / `<!-- ITO:END -->`; pre-marked files remain unchanged.
- Verification treats `ito-rs/crates/ito-templates/assets/adapters` as a separate check and found no unmarked plain markdown there, so no adapter sample was modified.
- Structural flow: `scan assets -> add markers to plain markdown -> preserve pre-marked files -> verify adapter status`.
- Main concern is compliance preservation without altering already valid files.

## release_workflow
**Drill down:** `release_workflow/_index.md` → `release_workflow.md`, `build_and_coverage_guardrails.md`

- Covers the Ito release pipeline plus build and coverage guardrails.
- Release chain:
  - `release-plz` merges the release PR, publishes crates.io releases, and creates `vX.Y.Z` tags
  - `cargo-dist` consumes tags to build artifacts and publish GitHub Releases
  - Homebrew formula updates go to `withakay/homebrew-ito`
  - Release notes are polished after release steps complete
- Coordinating files include `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, and `release-plz.toml`
- Important rules:
  - `release-plz.toml` must not set `git_only = true`
  - `publish-homebrew-formula` fails if the generated formula already contains a `service do` block
- Dependencies include GitHub Actions, `release-plz`, `cargo-dist`, crates.io token, Homebrew tap token, and optionally Claude Code OAuth for release-note polishing.
- Build/coverage guardrails add:
  - `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset
  - `ito-rs/tools/max_lines_baseline.txt` defines existing oversized Rust files so failures only flag regressions
  - `cargo-deny` allows `wit-bindgen@0.51` as a narrowly scoped wasip3 transitive duplicate
- Guardrail flow: `make check -> LLVM toolchain resolution -> cargo-llvm-cov -> max-lines baseline check -> cargo-deny exception`.
- Overall purpose is to keep release automation, coverage, and dependency checks reliable without broad exceptions or brittle toolchain assumptions.