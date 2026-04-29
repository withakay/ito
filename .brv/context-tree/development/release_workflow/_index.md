---
children_hash: ff028bbf98a1bd3f0723ce4b8625649e305eb4a860bd42252fc40e4e43757750
compression_ratio: 0.4179588892895781
condensation_order: 1
covers: [build_and_coverage_guardrails.md, manifesto_instruction_implementation_notes.md, release_plz_guardrails.md, release_workflow.md]
covers_token_total: 2773
summary_level: d1
token_count: 1159
type: summary
---
## Release Workflow Guardrails and Implementation Notes

This level groups the release pipeline, release-plz configuration, and build/verification guardrails for Ito. The entries are tightly connected: **release_workflow.md** describes the end-to-end publishing flow, while **build_and_coverage_guardrails.md**, **release_plz_guardrails.md**, and **manifesto_instruction_implementation_notes.md** capture specific constraints that keep releases, coverage checks, and rendered instructions consistent.

### Core release pipeline
- **release_workflow.md** defines the main release sequence:
  - `release-plz` merges release PRs, publishes crates.io releases, and creates `vX.Y.Z` tags.
  - `cargo-dist` consumes version tags to build artifacts and create GitHub Releases.
  - Homebrew formula updates are pushed to `withakay/homebrew-ito`.
- Key automation files:
  - `.github/workflows/release-plz.yml`
  - `.github/workflows/v-release.yml`
  - `.github/workflows/polish-release-notes.yml`
  - `dist-workspace.toml`
  - `release-plz.toml`
- Important rule from **release_workflow.md**: do not set `git_only = true` in `release-plz.toml`, because it can miscalculate repository paths during diff/worktree operations.

### Release-plz guardrails and coordination paths
- **release_plz_guardrails.md** documents how `release-plz` should run from the repo root with dirty publishing disabled:
  - `allow_dirty = false`
  - `publish_allow_dirty = false`
  - workspace changelog updates enabled
  - workspace dependency updates enabled
  - `cliff.toml` used as changelog config
  - git tags enabled only for `ito-cli`
- Coordination-branch behavior is centered on `.ito/` paths:
  - `.ito/changes`
  - `.ito/specs`
  - `.ito/modules`
  - `.ito/workflows`
  - `.ito/audit`
- These paths must remain gitignored. If tracked ignored files appear, the fix is to remove them from Git tracking with `git rm --cached` while keeping local files on disk.
- The GitHub Actions release workflow uses:
  - a GitHub App token
  - checkout with `fetch-depth: 0`
  - build-essential installation
  - mise toolchain setup
  - Rust cache
  - `release-plz/action@v0.5`
  - `CARGO_REGISTRY_TOKEN` for publishing

### Build and coverage guardrails
- **build_and_coverage_guardrails.md** focuses on making verification resilient in mixed toolchain environments.
- Main fixes:
  - `Makefile` test-coverage target now derives `LLVM_COV` and `LLVM_PROFDATA` from the active rustup toolchain when unset.
  - `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so the guardrail only fails on regressions or new violations.
  - `wit-bindgen@0.51` is allowed as a cargo-deny duplicate because it is a wasip3 transitive dependency.
- The documented flow is:
  - `make check`
  - coverage target resolves LLVM toolchain vars
  - `cargo-llvm-cov` runs
  - max-lines guardrail compares against the baseline
  - `cargo-deny` accepts the `wit-bindgen@0.51` duplicate
- This entry ties directly to the release workflow by ensuring CI verification remains stable across Homebrew/rustup mixes.

### Manifesto instruction implementation notes
- **manifesto_instruction_implementation_notes.md** captures rendering rules for manifesto sync status and operation visibility.
- Key constraints:
  - `synced_at_generation` should only be set when coordination sync returns `Synchronized`.
  - `RateLimited` means no sync was observed during generation and must not be reported as fresh success.
  - `full --operation` requires `--change`.
  - Embedded operation instructions are scoped to the resolved change state.
  - Unconfigured operations render as `null`.
- This entry is about how generation output should reflect sync outcomes, not about release mechanics directly, but it shares the same coordination and workflow discipline as the other release-related files.

### Cross-entry relationships
- **release_workflow.md** is the umbrella release process.
- **release_plz_guardrails.md** narrows the release-plz behavior for repo-root execution, dirty checks, and `.ito` coordination paths.
- **build_and_coverage_guardrails.md** protects the verification side of the release pipeline.
- **manifesto_instruction_implementation_notes.md** defines how sync-related instruction output should be rendered during generation.

### Drill-down references
- Release pipeline and publishing: **release_workflow.md**
- release-plz repo-root and gitignore rules: **release_plz_guardrails.md**
- Coverage, max-lines, and cargo-deny fixes: **build_and_coverage_guardrails.md**
- Manifesto sync and operation rendering rules: **manifesto_instruction_implementation_notes.md**