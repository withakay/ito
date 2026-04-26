---
children_hash: 8dfa1c9e7b005d132861f12134db1cbe4a6d5cba634d678bd482424649707fb8
compression_ratio: 0.8241965973534972
condensation_order: 1
covers: [release_workflow.md]
covers_token_total: 529
summary_level: d1
token_count: 436
type: summary
---
# Release Workflow

Ito’s release pipeline is split across two coordinated systems: **release-plz** handles versioning, publishing to crates.io, and tagging, while **cargo-dist** builds artifacts from version tags and publishes GitHub Releases. The workflow also includes **Homebrew tap updates** via `withakay/homebrew-ito`, plus release-note polishing in CI.

## Core Flow
- **release-plz** merges the release PR
- It publishes crates and creates `vX.Y.Z` tags
- **cargo-dist** consumes version tags to build and publish the GitHub Release
- Homebrew formula updates are pushed after release publication
- Release notes are polished at the end of the pipeline

## Key Configuration Files
- `.github/workflows/release-plz.yml`
- `.github/workflows/v-release.yml`
- `.github/workflows/polish-release-notes.yml`
- `dist-workspace.toml`
- `release-plz.toml`

## Dependencies and Integrations
- GitHub Actions
- release-plz
- cargo-dist
- crates.io token
- Homebrew tap token
- Optional Claude Code OAuth for release-note polishing

## Important Rules
- **Do not set `git_only = true` in `release-plz.toml`**; it can miscalculate repository paths during diff/worktree operations.
- The `publish-homebrew-formula` job fails if the generated formula already includes a `service do` block.

## Release Outputs and Capabilities
- GitHub Releases
- Cross-platform installer artifacts
- Homebrew formula publishing
- Local installation via the `withakay/ito` tap

## Local Install Notes
- `brew install withakay/ito/ito`
- `brew upgrade ito`
- `brew unlink ito-cli`
- `brew link ito`
- Verify with `/opt/homebrew/bin/ito --version`

## Drill-Down
- See **release_workflow.md** for the full pipeline, CI coordination, and release configuration details.
