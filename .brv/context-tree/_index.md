---
children_hash: a3ef10e79c22e6739060d399c5705375dadeb4fb32ca24288e4c1594bd89a12b
compression_ratio: 0.7798816568047338
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 845
summary_level: d3
token_count: 659
type: summary
---
# Development

This domain groups the operational knowledge for ITO template retrofits, worktree validation, and release orchestration. The three child entries describe separate but related automation rulesets: safe markdown conversion, read-only worktree checks, and a split release pipeline. The common pattern is controlled automation with explicit machine-readable outcomes and preservation of compliant existing files.

## ITO Templates — `template_bundle_retrofit.md`
- Standardizes plain markdown assets under `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` / `<!-- ITO:END -->` markers.
- Only unmarked `.md` files are eligible; already marked files are left unchanged.
- Verification confirmed no unmarked adapter samples in `ito-rs/crates/ito-templates/assets/adapters`, so no adapter markdown was modified.
- Structural flow: scan assets → retrofit plain markdown → preserve compliant files → verify adapter status.

## ITO Workflow — `worktree_validation_flow.md`
- Uses `ito worktree validate --change <id> [--json]` as a dedicated read-only validation path for change work.
- Produces machine-readable status for OpenCode pre-tool hooks so they can distinguish unsafe states from recoverable mismatches.
- Hard-fail rule: main/control checkouts are failures.
- Advisory rule: non-main mismatches are not fatal; they return guidance and recovery instructions.
- Matching uses exact change-id prefixes to avoid false positives, including suffix worktrees such as `<change>-review`.

## Release Workflow — `release_workflow.md`
- Release pipeline is split between **release-plz** and **cargo-dist**:
  - **release-plz** handles versioning, crates.io publishing, and tagging.
  - **cargo-dist** builds artifacts from version tags and publishes GitHub Releases.
- Also includes Homebrew tap updates via `withakay/homebrew-ito` and CI-based release-note polishing.
- Key workflow/config files: `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, `release-plz.toml`.
- Integrations and dependencies include GitHub Actions, release-plz, cargo-dist, crates.io token, Homebrew tap token, and optional Claude Code OAuth for note polishing.
- Important rules:
  - Do not set `git_only = true` in `release-plz.toml`; it can miscalculate repo paths during diff/worktree operations.
  - `publish-homebrew-formula` fails if the generated formula already contains a `service do` block.
- Outputs include GitHub Releases, cross-platform installer artifacts, Homebrew formula publishing, and local installation via `withakay/ito`.