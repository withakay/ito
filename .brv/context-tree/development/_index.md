---
children_hash: 2e9c1f23bd89c346ba888a36d7de48e0461da9db4211ca33abe9d19ea42f0d7f
compression_ratio: 0.5877862595419847
condensation_order: 2
covers: [ito_templates/_index.md, release_workflow/_index.md]
covers_token_total: 786
summary_level: d2
token_count: 462
type: summary
---
# Development

## ITO Templates
- The template bundle retrofit standardized markdown assets in `ito-rs/crates/ito-templates/assets` by adding `<!-- ITO:START -->` and `<!-- ITO:END -->` markers to plain, unmarked `.md` files.
- Pre-marked files were preserved unchanged; the process explicitly avoided rewriting already compliant files.
- Adapter markdown under `ito-rs/crates/ito-templates/assets/adapters` was checked separately, but no unmarked adapter sample was found, so nothing there was modified.
- Core flow: scan assets → add markers to eligible plain markdown → preserve compliant files → verify adapter status.
- Drill down: `template_bundle_retrofit.md` for the full retrofit note, scope, and verification details.

## Release Workflow
- Ito’s release pipeline is split between **release-plz** and **cargo-dist**: release-plz handles versioning, crates.io publishing, and `vX.Y.Z` tagging, while cargo-dist builds artifacts from tags and publishes GitHub Releases.
- The workflow also includes Homebrew tap updates via `withakay/homebrew-ito` and CI-based release-note polishing.
- Key configuration files: `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, and `release-plz.toml`.
- Important rule: do **not** set `git_only = true` in `release-plz.toml`, because it can miscalculate repository paths during diff/worktree operations.
- Another failure condition: the `publish-homebrew-formula` job breaks if the generated formula already contains a `service do` block.
- Outputs/capabilities include GitHub Releases, cross-platform installer artifacts, Homebrew formula publishing, and local installation via the `withakay/ito` tap.
- Drill down: `release_workflow.md` for the full pipeline, CI coordination, and release configuration details.
