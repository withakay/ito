---
children_hash: 5ef7ed8726e18b6877338e2bb3679bd34d678365c5465d3f90956b9969d384cc
compression_ratio: 0.4515738498789346
condensation_order: 1
covers: [build_and_coverage_guardrails.md, manifesto_instruction_implementation_notes.md, release_workflow.md]
covers_token_total: 1652
summary_level: d1
token_count: 746
type: summary
---
# Development Release Workflow and Guardrails

This area captures the Ito release pipeline plus build/verification guardrails. The release path is split between versioning/publishing, artifact release creation, and post-release packaging, while the build side adds coverage resilience, line-limit enforcement, and dependency exception handling. See **release_workflow.md** for the end-to-end release process and **build_and_coverage_guardrails.md** for validation and CI guardrails.

## Release workflow
See **release_workflow.md**

- Release-plz handles release PRs, publishes crates.io releases, and creates version tags.
- cargo-dist consumes version tags to build and publish GitHub Releases.
- Homebrew formula updates are pushed to `withakay/homebrew-ito`.
- Supporting workflow files:
  - `.github/workflows/release-plz.yml`
  - `.github/workflows/v-release.yml`
  - `.github/workflows/polish-release-notes.yml`
  - `dist-workspace.toml`
  - `release-plz.toml`

### Key relationships and rules
- Pipeline order: merge release PR → release-plz publishes crates and tags `vX.Y.Z` → cargo-dist builds/releases → Homebrew update → release notes polished.
- `release-plz.toml` must not set `git_only = true`; it can miscalculate repository paths in diff/worktree flows.
- `publish-homebrew-formula` fails if the generated formula already includes a service `do` block.
- Local Homebrew usage is documented via `withakay/ito` install, upgrade, unlink/link, and version verification commands.

## Build and coverage guardrails
See **build_and_coverage_guardrails.md**

- The `Makefile` test-coverage target now resolves `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset.
- `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so the line-limit guardrail fails only on regressions or new violations.
- `cargo-deny` explicitly allows `wit-bindgen@0.51` as a wasip3 transitive duplicate.

### Key relationships and rules
- Coverage execution is designed to work in mixed Homebrew/rustup environments.
- Guardrail enforcement depends on the baseline file for distinguishing legacy oversize files from new growth.
- The cargo-deny exception is narrowly scoped to the exact duplicate version `^wit-bindgen@0.51$`.

## Manifesto instruction implementation notes
See **manifesto_instruction_implementation_notes.md**

- `synced_at_generation` is only populated when coordination sync returns `Synchronized`.
- `RateLimited` means no sync was observed during generation and must not be treated as fresh success.
- Full `--operation` requires `--change`.
- Embedded operation instructions are scoped to the resolved change state.
- Unconfigured operations render as `null`.

### Key relationships and rules
- Sync reporting is tied directly to generation-time coordination results.
- Operation visibility depends on whether a change is resolved.
- Memory instruction rendering exposes configured operations only; missing configuration stays null.