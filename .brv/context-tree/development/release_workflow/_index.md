---
children_hash: d842e3c81cd6824d909f2daf7b1bc04f4c168edf33c0a1ba94ae948cacc82c16
compression_ratio: 0.650749063670412
condensation_order: 1
covers: [build_and_coverage_guardrails.md, release_workflow.md]
covers_token_total: 1068
summary_level: d1
token_count: 695
type: summary
---
# development/release_workflow

This domain covers the Ito release pipeline and its build/verification guardrails. The main workflow is split between versioned publishing, artifact distribution, and release-quality checks; see **release_workflow.md** for the end-to-end release process and **build_and_coverage_guardrails.md** for CI/build safeguards.

## Core release pipeline

**release_workflow.md** documents the main release chain:

- `release-plz` merges the release PR, publishes crates.io releases, and creates `vX.Y.Z` tags
- `cargo-dist` consumes version tags to build artifacts and create GitHub Releases
- Homebrew formula updates are pushed to `withakay/homebrew-ito`
- Release notes are polished after the release steps complete

Key files coordinating this automation:

- `.github/workflows/release-plz.yml`
- `.github/workflows/v-release.yml`
- `.github/workflows/polish-release-notes.yml`
- `dist-workspace.toml`
- `release-plz.toml`

### Important rules and dependencies

- `release-plz.toml` must **not** set `git_only = true`; this can break repository path calculation during diff/worktree operations
- The `publish-homebrew-formula` job fails if the generated formula already contains a `service do` block
- The workflow depends on GitHub Actions, `release-plz`, `cargo-dist`, a crates.io token, a Homebrew tap token, and optionally Claude Code OAuth for release-note polishing

### Notable outputs

- GitHub Releases
- cross-platform installer artifacts
- Homebrew formula updates
- local installation support via the `withakay/ito` tap

## Build and coverage guardrails

**build_and_coverage_guardrails.md** captures build-system fixes and verification policy:

- `make check` now resolves `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset, making coverage more resilient in mixed Homebrew/rustup environments
- `ito-rs/tools/max_lines_baseline.txt` records existing oversized Rust files so the max-lines guardrail fails only on regressions or new violations
- `cargo-deny` allows `wit-bindgen@0.51` as a specific wasip3 transitive duplicate

### Build/coverage flow

`make check` → coverage target resolves LLVM toolchain vars → `cargo-llvm-cov` runs → max-lines check compares against baseline → `cargo-deny` accepts the `wit-bindgen@0.51` duplicate

### Key relationships

- Coverage behavior depends on `rustup` LLVM tools when `LLVM_COV` and `LLVM_PROFDATA` are not explicitly provided
- Line-limit enforcement depends on `max_lines_baseline.txt`
- The `wit-bindgen@0.51` exception is narrowly scoped to a wasip3 transitive dependency

### Why it matters

These guardrails prevent coverage failures caused by mixed toolchain discovery, keep oversized-file enforcement actionable, and avoid over-broad dependency-deny noise.
