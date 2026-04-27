---
children_hash: fc7ff25e318a9781ec286b6c282aa67a7930304f650aba6809f7e75b6dfd2a6c
compression_ratio: 0.6470092670598147
condensation_order: 2
covers: [ito_templates/_index.md, ito_workflow/_index.md, release_workflow/_index.md]
covers_token_total: 1187
summary_level: d2
token_count: 768
type: summary
---
## Development / ITO Knowledge Overview

This level d2 summary compresses the three main knowledge areas under **development**: template retrofit, worktree validation, and release workflow. Each entry documents a distinct operational rule set, with shared emphasis on safe automation, machine-readable status, and preserving existing compliant files.

### 1) ITO Templates — `template_bundle_retrofit.md`
- The template bundle retrofit standardizes plain markdown assets under `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` and `<!-- ITO:END -->` markers.
- Only unmarked plain `.md` files are eligible; pre-marked files are preserved unchanged.
- Verification of `ito-rs/crates/ito-templates/assets/adapters` found no unmarked adapter samples, so no adapter markdown was modified.
- Structural flow: scan assets → add markers to plain markdown → preserve compliant files → verify adapter status.
- Drill down in **template_bundle_retrofit.md** for file scope and retrofit details.

### 2) ITO Workflow — `worktree_validation_flow.md`
- Worktree validation now uses a dedicated read-only path for change work via `ito worktree validate --change <id> [--json]`.
- The command emits machine-readable status for OpenCode pre-tool hooks, allowing them to distinguish unsafe states from recoverable mismatches.
- **Hard-fail rule:** main/control checkouts are treated as failures.
- **Advisory rule:** non-main mismatches are not fatal and return guidance/recovery instructions.
- Matching uses exact change-id prefixes to avoid false positives, including suffix worktrees such as `<change>-review`.
- Structural flow: validate worktree → emit machine-readable status → hard-fail main/control → advise on other mismatches → match exact prefixes.
- Drill down in **worktree_validation_flow.md** for the full validation model.

### 3) Release Workflow — `release_workflow.md`
- The release pipeline is split between **release-plz** and **cargo-dist**:
  - **release-plz** handles versioning, crates.io publishing, and tagging.
  - **cargo-dist** builds artifacts from version tags and publishes GitHub Releases.
- The workflow also includes Homebrew tap updates via `withakay/homebrew-ito` and CI-based release-note polishing.
- Key workflow files:
  - `.github/workflows/release-plz.yml`
  - `.github/workflows/v-release.yml`
  - `.github/workflows/polish-release-notes.yml`
  - `dist-workspace.toml`
  - `release-plz.toml`
- Dependencies/integrations include GitHub Actions, release-plz, cargo-dist, crates.io token, Homebrew tap token, and optional Claude Code OAuth for note polishing.
- Important rules:
  - Do **not** set `git_only = true` in `release-plz.toml`; it can miscalculate repo paths during diff/worktree operations.
  - `publish-homebrew-formula` fails if the generated formula already contains a `service do` block.
- Outputs include GitHub Releases, cross-platform installer artifacts, Homebrew formula publishing, and local installation via `withakay/ito`.
- Drill down in **release_workflow.md** for pipeline and CI coordination details.