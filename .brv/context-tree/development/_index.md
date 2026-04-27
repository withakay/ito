---
children_hash: c7f89b92ea0278526ffa8a7235f5ec33a661971eaaaadf5d453e208937298198
compression_ratio: 0.5981067125645438
condensation_order: 2
covers: [ito_templates/_index.md, release_workflow/_index.md]
covers_token_total: 1162
summary_level: d2
token_count: 695
type: summary
---
# d2 Structural Summary

## development

The development domain is organized around two operational concerns: template asset normalization and release/build reliability. The child entries show a clear pattern of enforcing invariants during automation rather than rewriting already-correct artifacts.

### ito_templates
**Drill down:** `ito_templates/_index.md` → `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`

- The template bundle retrofit standardizes markdown assets under `ito-rs/crates/ito-templates/assets`.
- Core rule: plain `.md` files get `<!-- ITO:START -->` / `<!-- ITO:END -->` markers; already pre-marked files remain unchanged.
- Verification treated `ito-rs/crates/ito-templates/assets/adapters` as a separate check and found no unmarked plain markdown there, so no adapter sample was modified.
- Structural pattern: `scan assets -> add markers to plain markdown -> leave pre-marked files unchanged -> verify adapter sample status`.
- Main concern is compliance preservation without altering already valid files.

### release_workflow
**Drill down:** `release_workflow/_index.md` → `release_workflow.md`, `build_and_coverage_guardrails.md`

- This topic covers the Ito release pipeline plus build and coverage guardrails.
- The release chain is:
  - `release-plz` merges the release PR, publishes crates.io releases, and creates `vX.Y.Z` tags
  - `cargo-dist` consumes tags to build artifacts and publish GitHub Releases
  - Homebrew formula updates go to `withakay/homebrew-ito`
  - Release notes are polished after release steps complete
- Coordinating files include `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, and `release-plz.toml`
- Important rules:
  - `release-plz.toml` must not set `git_only = true`
  - `publish-homebrew-formula` fails if the generated formula already contains a `service do` block
- Dependencies include GitHub Actions, `release-plz`, `cargo-dist`, crates.io token, Homebrew tap token, and optionally Claude Code OAuth for release-note polishing
- Build/coverage guardrails add:
  - `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset
  - `ito-rs/tools/max_lines_baseline.txt` defines existing oversized Rust files so failures only flag regressions
  - `cargo-deny` allows `wit-bindgen@0.51` as a narrowly scoped wasip3 transitive duplicate
- Guardrail flow: `make check -> LLVM toolchain resolution -> cargo-llvm-cov -> max-lines baseline check -> cargo-deny exception`
- Overall purpose: keep release automation, coverage, and dependency checks reliable without introducing broad exceptions or brittle toolchain assumptions.