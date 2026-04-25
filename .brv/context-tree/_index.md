---
children_hash: 6817e296ed95e2ab17279ad4e531adc77ab093bcda9fef5767b7a9f874066043
compression_ratio: 0.9718045112781954
condensation_order: 3
covers: [development/_index.md]
covers_token_total: 532
summary_level: d3
token_count: 517
type: summary
---
# Development

## Overview
The development knowledge area contains two core topics: **ITO template bundle retrofit** and **release workflow**. Together they describe how markdown assets are standardized and how Ito is versioned, built, and distributed.

## ITO Templates
- Covers the marker retrofit applied to template assets in `ito-rs/crates/ito-templates/assets`.
- Structural rule: add `<!-- ITO:START -->` and `<!-- ITO:END -->` to plain, unmarked `.md` files only.
- Existing compliant files are preserved unchanged; the process explicitly avoids rewriting pre-marked markdown.
- Adapter markdown under `ito-rs/crates/ito-templates/assets/adapters` is handled separately; in the recorded retrofit, no unmarked adapter sample was found, so nothing there was modified.
- Core flow: scan assets → retrofit eligible plain markdown → preserve compliant files → verify adapter status.
- Drill down: `template_bundle_retrofit.md` for scope, verification, and retrofit details.

## Release Workflow
- Ito’s release pipeline is split between **release-plz** and **cargo-dist**.
- **release-plz** handles versioning, crates.io publishing, and `vX.Y.Z` tagging.
- **cargo-dist** builds artifacts from tags and publishes GitHub Releases.
- The workflow also includes Homebrew tap updates via `withakay/homebrew-ito` and CI-based release-note polishing.
- Key configuration files: `.github/workflows/release-plz.yml`, `.github/workflows/v-release.yml`, `.github/workflows/polish-release-notes.yml`, `dist-workspace.toml`, and `release-plz.toml`.
- Important constraint: do **not** set `git_only = true` in `release-plz.toml`, because it can miscalculate repository paths during diff/worktree operations.
- Failure condition to watch: `publish-homebrew-formula` breaks if the generated formula already contains a `service do` block.
- Outputs include GitHub Releases, cross-platform installer artifacts, Homebrew formula publishing, and local installation via the `withakay/ito` tap.
- Drill down: `release_workflow.md` for pipeline coordination and configuration specifics.