---
children_hash: 98c15b3f14d93ecff49dc6eabae89c46f36d8e03ba42c720b6d0446b178f917d
compression_ratio: 0.39964912280701753
condensation_order: 1
covers: [build_and_coverage_guardrails.md, manifesto_instruction_implementation_notes.md, release_plz_guardrails.md, release_workflow.md]
covers_token_total: 2850
summary_level: d1
token_count: 1139
type: summary
---
## Release Workflow Guardrails

This set of entries defines the release pipeline for Ito and the guardrails that keep publishing, coverage, and manifest rendering consistent. The core workflow is centered on `release_workflow.md`, with supporting constraints in `build_and_coverage_guardrails.md`, `release_plz_guardrails.md`, and `manifesto_instruction_implementation_notes.md`.

### Main release pipeline
`release_workflow.md` describes the end-to-end release sequence:

- merge a release PR
- `release-plz` publishes crates and creates version tags `vX.Y.Z`
- `cargo-dist` builds artifacts and creates GitHub Releases
- Homebrew formulas are updated in `withakay/homebrew-ito`
- release notes are polished afterward

It also establishes the split between:
- `release-plz` for versioning/publishing
- `cargo-dist` for artifact builds, GitHub Releases, and Homebrew publishing

Key config files:
- `.github/workflows/release-plz.yml`
- `.github/workflows/v-release.yml`
- `.github/workflows/polish-release-notes.yml`
- `dist-workspace.toml`
- `release-plz.toml`

Important rule from `release_workflow.md`:
- Do not set `git_only = true` in `release-plz.toml`, because it can miscalculate repository paths during diff/worktree operations.

### Build and coverage guardrails
`build_and_coverage_guardrails.md` documents build verification fixes around `make check` and coverage execution:

- `Makefile` coverage target now derives `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset
- `ito-rs/tools/max_lines_baseline.txt` tracks existing oversized Rust files so only regressions or new violations fail
- `cargo-deny` allows `wit-bindgen@0.51` as a wasip3 transitive duplicate

Workflow shape:
- `make check` → coverage target resolves LLVM toolchain vars → `cargo-llvm-cov` runs → max-lines check compares against baseline → `cargo-deny` accepts the duplicate

Key pattern:
- `^wit-bindgen@0.51$`

### Release-plz coordination and gitignore rules
`release_plz_guardrails.md` captures repo-root release-plz behavior and coordination-branch hygiene:

- `.ito` coordination paths must remain gitignored
- already tracked ignored files should be removed from Git tracking with `git rm --cached`
- `release-plz.toml` stays at the repository root so repo discovery works in temp clones
- the GitHub Actions workflow runs release-plz on `main` with separate `release` and `release-pr` jobs

Important operational settings:
- `allow_dirty = false`
- `publish_allow_dirty = false`
- workspace changelog updates enabled
- workspace dependency updates enabled
- changelog config uses `cliff.toml`
- `ito-cli` is the only package with git tags enabled

Protected paths pattern:
- `^.ito/(changes|specs|modules|workflows|audit)$`

Rules emphasized in this entry:
- keep `.ito/changes`, `.ito/specs`, `.ito/modules`, `.ito/workflows`, and `.ito/audit` gitignored
- if tracked ignored files appear under `.ito/changes`, untrack them with `git rm --cached`
- do not unignore `.ito/changes`
- do not set `git_only = true`

### Manifesto instruction rendering and sync status
`manifesto_instruction_implementation_notes.md` defines how manifesto generation reports sync and operation visibility:

- `synced_at_generation` is only populated when coordination sync returns `Synchronized`
- `RateLimited` means no sync was observed during generation and must not be reported as fresh success
- full `--operation` requires `--change`
- embedded operation instructions are scoped to the resolved change state
- unconfigured operations render as `null`

Facts captured here:
- `synced_at_generation` only for `Synchronized`
- `RateLimited` is not fresh success
- `full --operation` requires `--change`
- memory instruction rendering exposes configured operations, but unconfigured ones stay `null`

### Cross-entry relationships
- `release_workflow.md` is the parent release-process overview.
- `build_and_coverage_guardrails.md` and `release_plz_guardrails.md` refine the workflow’s execution and repo-state constraints.
- `manifesto_instruction_implementation_notes.md` covers how instruction rendering and sync reporting behave during the broader coordination/release flow.

### Drill-down map
- `release_workflow.md` — overall release pipeline and artifact flow
- `build_and_coverage_guardrails.md` — coverage, max-lines, and cargo-deny guardrails
- `release_plz_guardrails.md` — release-plz config, `.ito` gitignore policy, dirty-state handling
- `manifesto_instruction_implementation_notes.md` — sync status semantics and instruction rendering rules