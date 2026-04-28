---
children_hash: 434ffdea30a6790b0ea0a32a7376461f8f4829c829ae5a571c0cc05641ba0c0b
compression_ratio: 0.9199614271938283
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 1037
summary_level: d3
token_count: 954
type: summary
---
# Development Knowledge Overview

This level summarizes the operational structure of the Ito repo across three areas: template normalization, repo/worktree safety, and release/build guardrails. The child entries each define a distinct process boundary, with explicit rules, verification steps, and file-level coordination points.

## ito_templates/_index.md — Template bundle retrofit
- Standardizes `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` / `<!-- ITO:END -->` markers to plain markdown files.
- Core rule: **plain `.md` files are retrofitted; already pre-marked files stay unchanged**.
- Verification found no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter sample was modified.
- Process pattern: `scan assets -> add markers to plain markdown -> preserve pre-marked files -> verify adapter status`.
- Drill down: `template_bundle_retrofit.md`, `template_bundle_retrofit.abstract.md`, `template_bundle_retrofit.overview.md`.

## ito_workflow/_index.md — Repo refresh and worktree validation
- Groups two safety workflows:
  - **repo-level refresh auditing**
  - **worktree validation behavior**
- `repo_level_ito_refresh_audit.md`:
  - Scope covers managed Ito harness assets under `ito-rs/crates/ito-templates/assets/skills` and `ito-rs/crates/ito-templates/assets/commands`.
  - Default project command: `ito-project-setup`.
  - Flow: refresh harness assets -> audit for `ito-*` orphans -> skip user-owned entries -> rerun `ito init --update --tools all` -> confirm unchanged git diff hash.
  - Outcome: no `ito-*` orphan skills/commands remained; refresh was idempotent.
  - Boundary: user-owned files like `.claude/skills/byterover*` and `.opencode/commands/compare-workflow-tool.md` must not be touched.
- `worktree_validation_flow.md`:
  - `ito worktree validate --change <id> [--json]` is read-only and emits machine-readable status.
  - Main/control checkouts hard-fail; non-main mismatches return advisory guidance.
  - Matching uses exact change-id prefixes, including suffix worktrees like `<change>-review`, to avoid false positives.
  - OpenCode pre-tool hooks depend on this machine-readable status.
- Drill down: `repo_level_ito_refresh_audit.md`, `worktree_validation_flow.md`.

## release_workflow/_index.md — Release pipeline and guardrails
- Covers the release chain plus build/coverage safeguards.
- `release_workflow.md`:
  - `release-plz` merges the release PR, publishes crates.io releases, and creates `vX.Y.Z` tags.
  - `cargo-dist` uses version tags to build artifacts and create GitHub Releases.
  - Homebrew formula updates go to `withakay/homebrew-ito`.
  - Release notes are polished after release steps complete.
  - Coordinating files: `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, `release-plz.toml`.
  - Important rules: `release-plz.toml` must **not** set `git_only = true`; `publish-homebrew-formula` fails if the generated formula already contains a `service do` block.
  - Depends on GitHub Actions, `release-plz`, `cargo-dist`, crates.io token, Homebrew tap token, and optionally Claude Code OAuth for release-note polishing.
- `build_and_coverage_guardrails.md`:
  - `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset.
  - `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so the max-lines check fails only on regressions/new violations.
  - `cargo-deny` allows `wit-bindgen@0.51` as a narrowly scoped wasip3 transitive duplicate.
  - Build flow: `make check` -> LLVM vars resolved -> `cargo-llvm-cov` -> max-lines baseline check -> `cargo-deny` duplicate allowance.
- Drill down: `release_workflow.md`, `build_and_coverage_guardrails.md`.