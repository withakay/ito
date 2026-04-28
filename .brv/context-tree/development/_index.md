---
children_hash: merged-template-workflow-release-manifesto-published-mirror
compression_ratio: 0.78
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md]
covers_token_total: 1817
summary_level: d2
token_count: 1450
type: summary
---
# d2 Structural Summary: Ito knowledge tree

This d2 summary compresses the main development knowledge areas: template retrofit, Ito workflow publication and validation, release/build reliability, and manifesto instruction implementation. Each entry documents an operational rule set, with shared emphasis on safe automation, machine-readable status, state-aware behavior, and preserving compliant existing files.

## development / ito_templates
See **`template_bundle_retrofit.md`** for the detailed retrofit record.

- This topic documents the marker-standardization retrofit for `ito-rs/crates/ito-templates/assets`.
- The defining rule is binary: plain markdown gets `<!-- ITO:START -->` / `<!-- ITO:END -->`; already marked markdown is left unchanged.
- The retrofit preserved compliance without rewriting files that were already correct.
- Verification confirmed no unmarked plain markdown in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter sample required modification.

### Structural pattern
`scan assets -> retrofit plain .md files with ITO markers -> preserve pre-marked files -> verify adapter sample status`

### Drill-down
- **`template_bundle_retrofit.md`** - primary retrofit summary and verification facts
- **`template_bundle_retrofit.abstract.md`** - abstracted structural view
- **`template_bundle_retrofit.overview.md`** - overview of the marker retrofit approach

---

## development / ito_workflow
See **`published_ito_mirror.md`** and **`worktree_validation_flow.md`** for the two main workflow branches.

- This topic covers two complementary concerns: publishing a read-only mirror of coordination-backed state and validating worktrees safely during change work.
- The authoritative writable state remains in coordination storage; `docs/ito` is generated as a consumable read-only mirror for plain GitHub/main checkouts.
- Path handling is safety-first: mirror paths are project-relative, configurable, and strictly validated before generation.
- The published mirror is deterministic and excludes symlinks, so it is safe for consumption without exposing writable state.
- Worktree validation is a dedicated read-only gate that distinguishes unsafe main/control checkouts from advisory mismatches elsewhere and emits machine-readable status for hooks.

### Relationship between the child entries
- **`published_ito_mirror.md`** is about **publishing** state outward into a safe read-only tree.
- **`worktree_validation_flow.md`** is about **guarding** change work and ensuring tooling reacts correctly to checkout state.
- Together they support a workflow where coordination state is authoritative, published docs are consumable, and validation prevents unsafe operations.

### Drill-down
- **`published_ito_mirror.md`**
- Configures mirror output via `changes.published_mirror.path` with default `docs/ito`.
- Validates paths by rejecting empty, absolute, parent-traversal, and project-root-only inputs.
- Generates a read-only layout under `README.md`, `changes/active`, `changes/archive`, and `specs`.
- `ito publish` loads cascading config, detects drift, and replaces the mirror from coordination-backed state.
- **`worktree_validation_flow.md`**
- `ito worktree validate --change <id> [--json]` provides the dedicated validation flow.
- Main/control checkouts are hard failures.
- Non-main mismatches are advisory and include recovery guidance.
- Matching uses exact change-id prefixes, including suffix worktrees like `<change>-review`, to avoid false positives.

---

## development / release_workflow
See **`release_workflow.md`** and **`build_and_coverage_guardrails.md`** for release operations, plus **`manifesto_instruction_implementation_notes.md`** for instruction rendering behavior.

- This domain covers the Ito release pipeline and its build/verification guardrails.
- The release pipeline is split across versioned publishing, artifact distribution, and release-quality checks.
- Release automation depends on GitHub Actions, `release-plz`, `cargo-dist`, a crates.io token, a Homebrew tap token, and optionally Claude Code OAuth for release-note polishing.

### Core release pipeline
- `release-plz` merges the release PR, publishes crates.io releases, and creates `vX.Y.Z` tags.
- `cargo-dist` consumes version tags to build artifacts and create GitHub Releases.
- Homebrew formula updates are pushed to `withakay/homebrew-ito`.
- Release notes are polished after the release steps complete.

### Important rules and dependencies
- `release-plz.toml` must **not** set `git_only = true`; this can break repository path calculation during diff/worktree operations.
- The `publish-homebrew-formula` job fails if the generated formula already contains a `service do` block.
- The workflow depends on `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, and `release-plz.toml`.

### Build and coverage guardrails
See **`build_and_coverage_guardrails.md`** for build-system fixes and verification policy.

- `make check` resolves `LLVM_COV` and `LLVM_PROFDATA` from the active `rustup` toolchain when unset, improving coverage reliability in mixed Homebrew/rustup environments.
- `ito-rs/tools/max_lines_baseline.txt` records existing oversized Rust files so the max-lines guardrail fails only on regressions or new violations.
- `cargo-deny` allows `wit-bindgen@0.51` as a narrowly scoped wasip3 transitive duplicate.

### Manifesto instruction implementation notes
See **`manifesto_instruction_implementation_notes.md`** for generation-time sync and operation rendering rules.

- `synced_at_generation` is only populated when coordination sync returns `Synchronized`.
- `RateLimited` means no sync was observed during generation and must not be treated as fresh success.
- Full `--operation` requires `--change`.
- Embedded operation instructions are scoped to the resolved change state.
- Unconfigured operations render as `null`.
