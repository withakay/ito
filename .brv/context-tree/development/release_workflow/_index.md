---
children_hash: a050e2ad5a92bdf695a8d8de77ecb84a35d10d0454e3097f41084cc78b4b7266
compression_ratio: 0.5707736389684813
condensation_order: 1
covers: [build_and_coverage_guardrails.md, manifesto_instruction_implementation_notes.md, release_workflow.md]
covers_token_total: 1745
summary_level: d1
token_count: 996
type: summary
---
# Release Workflow

This level summarizes the release and verification knowledge captured in three child entries:

- **Release Workflow** — end-to-end Ito release automation and publishing pipeline
- **Build and Coverage Guardrails** — verification guardrails for coverage, file-size regression control, and cargo-deny exceptions
- **Manifesto Instruction Implementation Notes** — rendering and sync-status rules for manifesto instruction output

## Core release pipeline

The release process is split across **release-plz** and **cargo-dist**:

- **release-plz** merges the release PR, publishes crates.io releases, and creates version tags
- **cargo-dist** consumes tags to build and publish GitHub Releases
- Homebrew formula updates are pushed to **withakay/homebrew-ito**
- Release notes are polished after release publication

Key automation files referenced by **Release Workflow**:
- `.github/workflows/release-plz.yml`
- `.github/workflows/v-release.yml`
- `.github/workflows/polish-release-notes.yml`
- `dist-workspace.toml`
- `release-plz.toml`

### Important rules and constraints
- Do **not** set `git_only = true` in `release-plz.toml`; it can miscalculate repository paths during diff/worktree operations
- The `publish-homebrew-formula` job fails if the generated formula already contains a `service do` block
- Local installation supports the `withakay/ito` tap, including `brew install`, `brew upgrade`, `brew unlink`, and `brew link` flows

## Build and verification guardrails

**Build and Coverage Guardrails** documents the release-side verification path:

- `make check` now resolves `LLVM_COV` and `LLVM_PROFDATA` from the active **rustup** toolchain when unset
- This avoids failures caused by mixed **Homebrew/rustup** environments
- `cargo-llvm-cov` runs after toolchain resolution
- Oversized Rust files are governed by `ito-rs/tools/max_lines_baseline.txt`
- `cargo-deny` permits `wit-bindgen@0.51` as a specific **wasip3** transitive duplicate

### Guardrail flow
`make check -> coverage target resolves LLVM toolchain vars -> cargo-llvm-cov runs -> max-lines guardrail compares against baseline -> cargo-deny accepts wit-bindgen@0.51 duplicate`

### Key preservation points
- The line-limit policy is baseline-driven: pre-existing oversized files are tolerated, regressions are not
- The cargo-deny exception is narrowly scoped to `^wit-bindgen@0.51$`

## Manifesto instruction rendering and sync behavior

**Manifesto Instruction Implementation Notes** defines how sync and operation instructions are represented:

- `synced_at_generation` is only populated when coordination sync returns **Synchronized**
- **RateLimited** means no sync was observed during generation and must **not** be treated as fresh success
- `full --operation` requires `--change`
- Embedded operation instructions are scoped to the resolved change state
- Unconfigured operations render as `null`

### Summary of behavior
- Successful sync only: `synced_at_generation`
- No observed sync: `RateLimited`
- Full operation visibility: only when paired with a resolved change
- Missing configuration: render `null`

## Relationships between entries

- **Release Workflow** is the parent operational pipeline
- **Build and Coverage Guardrails** supports release confidence and CI reliability
- **Manifesto Instruction Implementation Notes** constrains how release/coordination state is rendered and reported
- Both child entries relate back to release automation correctness and observability, but cover different layers:
  - build/test verification
  - instruction rendering and sync semantics

## Drill-down references

- **Release Workflow** for release-plz, cargo-dist, GitHub Releases, and Homebrew publishing
- **Build and Coverage Guardrails** for `make check`, LLVM env resolution, max-lines baseline, and cargo-deny exception handling
- **Manifesto Instruction Implementation Notes** for `Synchronized` / `RateLimited` semantics, `full --operation`, and null rendering rules